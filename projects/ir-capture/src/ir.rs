//! TSOP38238 capture: mark/space timings, NEC decode when possible.

use core::fmt::Write as _;
use embassy_rp::gpio::Input;
use embassy_time::Instant;
use heapless::{String, Vec};

/// Enough edges for NEC (67) plus a little headroom.
const MAX_EDGES: usize = 96;
/// Idle time after the last edge that ends a frame.
const FRAME_GAP_US: u64 = 20_000;
/// Give up if a burst never goes idle (stuck pin).
const MAX_FRAME_MS: u64 = 150;
const MIN_EDGES: usize = 2;
const MIN_FIRST_PULSE_US: u32 = 200;

pub struct Frame {
    pub durations_us: Vec<u32, MAX_EDGES>,
}

#[derive(Clone, Copy)]
pub enum Event {
    Nec { addr: u16, cmd: u8, repeat: bool },
    Raw { count: u8, d0: u32, d1: u32 },
}

/// Wait for the TSOP output to go low, then busy-sample the rest of the burst.
pub async fn wait_and_capture(pin: &mut Input<'_>) -> Frame {
    pin.wait_for_falling_edge().await;
    capture_busy(pin)
}

fn capture_busy(pin: &Input<'_>) -> Frame {
    let mut durations: Vec<u32, MAX_EDGES> = Vec::new();
    let mut last = Instant::now();
    let frame_start = last;
    let mut high = false;

    loop {
        let now = Instant::now();
        if now.duration_since(frame_start).as_millis() > MAX_FRAME_MS {
            break;
        }
        let elapsed = now.duration_since(last).as_micros();
        if elapsed > FRAME_GAP_US {
            break;
        }
        if pin.is_high() != high {
            if durations.push(elapsed as u32).is_err() {
                break;
            }
            last = now;
            high = !high;
        }
    }

    Frame {
        durations_us: durations,
    }
}

pub fn decode(frame: &Frame, last_nec: Option<(u16, u8)>) -> Option<Event> {
    let d = frame.durations_us.as_slice();
    if d.len() < MIN_EDGES || d[0] < MIN_FIRST_PULSE_US {
        return None;
    }
    if let Some(event) = decode_nec(d, last_nec) {
        return Some(event);
    }
    Some(Event::Raw {
        count: d.len().min(255) as u8,
        d0: d[0],
        d1: d.get(1).copied().unwrap_or(0),
    })
}

fn decode_nec(d: &[u32], last_nec: Option<(u16, u8)>) -> Option<Event> {
    if !near(d[0], 9000, 25) {
        return None;
    }

    if near(d[1], 2250, 30) && d.len() <= 6 {
        let (addr, cmd) = last_nec?;
        return Some(Event::Nec {
            addr,
            cmd,
            repeat: true,
        });
    }

    if !near(d[1], 4500, 25) || d.len() < 66 {
        return None;
    }

    let data = &d[2..];
    let mut bits: u32 = 0;
    for i in 0..32 {
        let mark = data[i * 2];
        let space = data[i * 2 + 1];
        if !near(mark, 560, 50) {
            return None;
        }
        if space > 1000 {
            bits |= 1 << i;
        }
    }

    let addr8 = (bits & 0xFF) as u8;
    let addr_inv = ((bits >> 8) & 0xFF) as u8;
    let cmd = ((bits >> 16) & 0xFF) as u8;
    let addr = if addr8 ^ addr_inv == 0xFF {
        addr8 as u16
    } else {
        addr8 as u16 | (u16::from(addr_inv) << 8)
    };

    Some(Event::Nec {
        addr,
        cmd,
        repeat: false,
    })
}

fn near(value: u32, nominal: u32, pct: u32) -> bool {
    let lo = nominal.saturating_mul(100 - pct) / 100;
    let hi = nominal.saturating_mul(100 + pct) / 100;
    value >= lo && value <= hi
}

pub fn oled_line(event: &Event) -> String<16> {
    let mut line = String::new();
    match *event {
        Event::Nec { addr, cmd, repeat } => {
            if addr > 0xFF {
                let _ = write!(line, "{addr:04X}:{cmd:02X}");
            } else {
                let _ = write!(line, "{addr:02X}:{cmd:02X}");
            }
            if repeat {
                let _ = line.push_str(" rpt");
            }
        }
        Event::Raw { count, .. } => {
            let _ = write!(line, "RAW {count}");
        }
    }
    line
}

pub fn json_body(event: &Event) -> String<96> {
    let mut body = String::new();
    match *event {
        Event::Nec { addr, cmd, repeat } => {
            let _ = write!(
                body,
                "{{\"proto\":\"nec\",\"addr\":{addr},\"cmd\":{cmd},\"rep\":{}}}",
                if repeat { "true" } else { "false" }
            );
        }
        Event::Raw { count, d0, d1 } => {
            let _ = write!(
                body,
                "{{\"proto\":\"raw\",\"n\":{count},\"lead\":[{d0},{d1}]}}"
            );
        }
    }
    body
}

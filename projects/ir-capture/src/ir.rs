//! Capture and decode a burst from a Vishay TSOP38238 infrared (IR) receiver.
//!
//! # Electronics crash course (for software engineers)
//!
//! A TV remote does not send Wi-Fi or Bluetooth. It blinks an **infrared LED**
//! (light your eyes cannot see, typically around 940 nm). Those blinks are not
//! a simple on/off of the LED for each bit. Instead the LED is switched on and
//! off at about **38,000 times per second** (38 kHz). That fast flicker is the
//! **carrier**. The actual message (button A vs button B) is encoded by turning
//! that carrier **on for a while, then off for a while**. Those on/off chunks
//! last hundreds of microseconds to a few milliseconds — slow compared with
//! 38 kHz, fast compared with a human button press.
//!
//! Why the carrier? Sunlight, lamps, and phone screens also produce infrared.
//! A receiver that just looked for “any IR” would fire constantly. The
//! TSOP38238 is a tiny analog + digital chip with a **band-pass filter** tuned
//! to ~38 kHz. Ambient light is mostly ignored. When a 38 kHz burst is in
//! view, the chip **demodulates** it: it hides the 38 kHz flicker from you and
//! presents a single digital pin that means “carrier present” vs “carrier
//! absent”.
//!
//! That pin is **idle-high, active-low**:
//! - **High** (~3.3 V) = idle, no 38 kHz burst (or the burst just ended).
//! - **Low** (~0 V) = a 38 kHz burst is being seen right now.
//!
//! In IR jargon, a **mark** is “carrier on” (this chip pulls the pin **low**)
//! and a **space** is “carrier off” (pin sits **high**). A remote button press
//! is therefore a sequence of pulse widths: low for 9 ms, high for 4.5 ms,
//! low for 560 µs, high for 560 µs, and so on. This file records those widths
//! in microseconds, then tries to interpret them as **NEC** or **Samsung32**.
//! If it is neither, it still reports a `Raw` event so you can see that
//! *something* arrived.
//!
//! The Pico GPIO (GP18) is just a digital input: it samples 0 vs 1. Firmware
//! also turns on an internal **pull-up resistor** (see `main.rs`): a weak
//! connection to 3.3 V so the pin reads high if the wire is disconnected or
//! the TSOP is idle. The TSOP already has its own pull-up; doubling up is
//! harmless and makes a floating wire less noisy.
//!
//! # Timing strategy
//!
//! Waiting for the *start* of a burst can be event-driven
//! (`wait_for_falling_edge`): the CPU can sleep until the pin goes high → low.
//! Measuring the rest of the burst cannot use long async sleeps — a NEC bit is
//! only ~560 µs of mark plus 560 µs or ~1.7 ms of space. So after the first
//! falling edge we **busy-sample** the pin in a tight loop and timestamp every
//! 0↔1 change with `embassy_time::Instant`.
//!
//! That busy loop **blocks the Embassy executor** for the length of the burst
//! (typically tens of milliseconds). Wi-Fi and USB tasks do not run during
//! that window. For this sandbox that is acceptable; a game firmware with
//! several receivers would eventually want interrupts, PIO, or a dedicated
//! core so networking is not stalled.
//!
//! # NEC
//!
//! A normal NEC frame, after demodulation, looks like:
//!
//! 1. Leader: ~9000 µs mark, ~4500 µs space.
//! 2. 32 bits, least-significant bit first. Each bit is a ~560 µs mark
//!    followed by a space: ~560 µs means `0`, ~1690 µs means `1`.
//! 3. A final ~560 µs mark (stop bit). This file does not require it.
//!
//! Those 32 bits are usually four bytes: address, inverted address, command,
//! inverted command. “Inverted” means bitwise NOT (every 0 becomes 1). If
//! `address XOR inverted_address == 0xFF`, this is classic 8-bit NEC and we
//! keep an 8-bit address. Otherwise it is “extended NEC” and we stitch the
//! two address bytes into a 16-bit value. This decoder does **not** check
//! the inverted command byte; it still reports `cmd`.
//!
//! Holding a NEC button does not resend the 32 bits. The remote sends a short
//! **repeat**: ~9000 µs mark, ~2250 µs space, then a short mark. Repeat frames
//! contain no address/command, so we reuse the last successfully decoded pair.
//!
//! # Samsung32
//!
//! Many Samsung TV remotes use a close cousin: ~4500 µs mark and ~4500 µs
//! space as the leader, then the same 32 LSB-first bits as NEC. Address is
//! the first 16 bits as-is (often `0xE0E0`). A hold-repeat is a short burst
//! with the same 4.5 ms / 4.5 ms leader and no payload; we reuse the last
//! Samsung pair, same idea as NEC repeats.

// `core::fmt::Write` is the trait that makes the `write!` macro work on our
// tiny `heapless::String`. `as _` imports the trait without binding the name
// `Write` in this module (we never write `Write` in type position here).
use core::fmt::Write as _;
// Embassy's GPIO input wrapper around a Pico pin. `.is_high()` / `.is_low()`
// read the voltage *right now*; `.wait_for_falling_edge()` sleeps until
// high → low. The lifetime `'a` on `Input<'a>` is the pin's borrow from
// `embassy_rp::init` in `main`.
use embassy_rp::gpio::Input;
// Monotonic timestamp. On this firmware Embassy drives it from a hardware
// timer. `Instant::now()` is like `std::time::Instant` but works in `no_std`
// (no OS clock). Differences are in microseconds, which is the right unit
// for IR pulse widths.
use embassy_time::Instant;
// `heapless` collections live on the stack (or in a static) with a **fixed
// capacity** as a const generic. There is no heap allocator in this crate
// (`#![no_std]` in `main.rs`), so `std::vec::Vec` / `String` are unavailable.
use heapless::{String, Vec};

/// Maximum number of mark/space durations we will store for one burst.
///
/// A full NEC or Samsung32 frame is 2 leader pulses + 32 bits × 2
/// (mark+space) = 66 samples, sometimes 67 with a trailing stop mark. 96
/// leaves slack for slightly chatty remotes without using much RAM
/// (`96 * 4` bytes of `u32`).
const MAX_EDGES: usize = 96;

/// If the pin stays at the same level this long, the burst is over.
///
/// After the last mark the remote goes idle (pin high) until the next frame
/// (~40–100 ms later for NEC repeats). 25 ms is long enough that we are not
/// still inside a bit, and short enough that we finish before the next frame.
const FRAME_GAP_US: u64 = 25_000;

/// Hard cap on how long we will spin waiting for that idle gap.
///
/// If the wire is stuck low (short to ground, unplugged TSOP with a
/// conflicting driver, etc.) we would otherwise loop forever. 150 ms is
/// longer than a legitimate NEC/Samsung frame plus a generous margin.
const MAX_FRAME_MS: u64 = 150;

/// Fewer than two durations is not even “a pulse then a gap” — treat as noise.
const MIN_EDGES: usize = 2;

/// Reject a burst whose first pulse is shorter than this.
///
/// Electrical glitches can produce a few-microsecond spike. A real NEC leader
/// is ~9 ms and Samsung is ~4.5 ms; even odd protocols are usually hundreds
/// of microseconds.
const MIN_FIRST_PULSE_US: u32 = 150;

/// One captured burst: alternating pulse widths in microseconds.
///
/// Index 0 is the first **mark** (pin low, carrier present), index 1 the
/// following **space** (pin high), index 2 the next mark, and so on. We do
/// not store the idle gap that ends the frame.
pub struct Frame {
    /// Pulse widths in microseconds, at most [`MAX_EDGES`] long.
    pub durations_us: Vec<u32, MAX_EDGES>,
}

/// What [`decode`] decided a [`Frame`] meant.
///
/// `Clone` + `Copy` because this is a handful of integers; passing by value
/// is cheaper and simpler than fighting borrows in `main`.
#[derive(Clone, Copy)]
pub enum Event {
    /// A NEC (or extended-NEC) button code.
    Nec {
        /// 8-bit address, or 16-bit if the remote uses extended NEC.
        addr: u16,
        /// Button / command byte (0–255). Meaning is remote-specific.
        cmd: u8,
        /// `true` if this was a hold-repeat, not a fresh 32-bit frame.
        repeat: bool,
    },
    /// A Samsung32 TV-remote button code.
    Samsung {
        /// Usually 16-bit (often `0xE0E0`).
        addr: u16,
        /// Button / command byte (0–255). Meaning is remote-specific.
        cmd: u8,
        /// `true` if this was a hold-repeat, not a fresh 32-bit frame.
        repeat: bool,
    },
    /// 38 kHz activity that did not match NEC or Samsung. Still useful on the
    /// OLED/HTTP.
    Raw {
        /// How many mark/space values we stored (capped at 255 for the wire format).
        count: u8,
        /// First duration (usually the leader mark), microseconds.
        d0: u32,
        /// Second duration (usually the leader space), or 0 if the burst was a
        /// single pulse.
        d1: u32,
    },
}

/// Busy-wait, pushing a duration every time the pin changes level.
///
/// Called with the pin **already low** (we just saw the falling edge).
/// `last` is “when the current level began”. The first `push` is therefore
/// the width of that opening mark.
///
/// Kept separate from `wait_for_falling_edge` so `main` can race only the
/// wait against the OLED tick. Combining them would let a timer cancel the
/// capture mid-burst.
pub fn capture_busy(pin: &Input<'_>) -> Frame {
    // Empty list of pulse widths; capacity is `MAX_EDGES`, length starts at 0.
    let mut durations: Vec<u32, MAX_EDGES> = Vec::new();
    // Timestamp of the most recent edge (the falling edge we just waited for,
    // as close as we can know — a few microseconds late because we returned
    // from the await and then called `now()`).
    let mut last = Instant::now();
    // Timestamp of the start of this whole capture, for the stuck-pin timeout.
    let frame_start = last;
    // `false` = we believe the pin is currently low (mark in progress).
    // We invert this every time we record an edge.
    let mut high = false;

    loop {
        // Read the hardware timer as often as the CPU can. On a 150 MHz
        // Cortex-M33 this loop is far faster than a 560 µs IR pulse, so we
        // will see every edge; we just will not know *exactly* which
        // microsecond it happened if two edges are closer than one loop.
        let now = Instant::now();
        // Give up if we have been spinning too long with no clean idle gap.
        if now.duration_since(frame_start).as_millis() > MAX_FRAME_MS {
            break;
        }
        // How long the pin has sat at the current level.
        let elapsed = now.duration_since(last).as_micros();
        // Long quiet stretch ⇒ the remote stopped transmitting. We do **not**
        // push `elapsed` itself: that would record the idle as a fake space.
        // The last stored value is the last real mark or space.
        if elapsed > FRAME_GAP_US {
            break;
        }
        // Level changed since the last recorded edge.
        if pin.is_high() != high {
            // `heapless::Vec::push` returns `Err(the_value)` if the array is
            // full. We drop the extra edge and stop; a truncated frame will
            // fail structured decode and may still show as `Raw`.
            if durations.push(elapsed as u32).is_err() {
                break;
            }
            // This edge is the start of the new level.
            last = now;
            // We are now in the opposite level (low↔high).
            high = !high;
        }
    }

    // Move `durations` into the public struct. No clone: `Vec` is not `Copy`.
    Frame {
        durations_us: durations,
    }
}

/// Turn a captured burst into NEC, Samsung, or a `Raw` fallback.
///
/// `last` is the previous non-repeat event. Repeat frames need it because
/// they do not carry the 32 data bits. `None` here means we have never seen
/// a full frame since boot, so a lone repeat becomes `Raw`.
pub fn decode(frame: &Frame, last: Option<Event>) -> Event {
    // Cheap view of the heapless vec as a normal slice (`&[u32]`).
    let d = frame.durations_us.as_slice();
    // Too short, or a needle-spike first pulse: still report `Raw` so the
    // OLED can show timings; `main` decides whether to POST.
    if d.len() < MIN_EDGES || d[0] < MIN_FIRST_PULSE_US {
        return Event::Raw {
            count: d.len().min(255) as u8,
            d0: d.first().copied().unwrap_or(0),
            d1: d.get(1).copied().unwrap_or(0),
        };
    }
    if let Some(event) = decode_nec(d, last) {
        return event;
    }
    if let Some(event) = decode_samsung(d, last) {
        return event;
    }
    Event::Raw {
        // OLED/JSON use `u8`; 96 edges still fits. `min(255)` is defensive.
        count: d.len().min(255) as u8,
        d0: d[0],
        // `get` so a 1-element slice cannot panic. `copied` turns `&u32` into
        // `u32`. Unreachable given `MIN_EDGES == 2`, but cheap insurance.
        d1: d.get(1).copied().unwrap_or(0),
    }
}

fn last_nec(last: Option<Event>) -> Option<(u16, u8)> {
    match last {
        Some(Event::Nec { addr, cmd, .. }) => Some((addr, cmd)),
        _ => None,
    }
}

fn last_samsung(last: Option<Event>) -> Option<(u16, u8)> {
    match last {
        Some(Event::Samsung { addr, cmd, .. }) => Some((addr, cmd)),
        _ => None,
    }
}

/// Strict-ish NEC parser. Returns `None` as soon as a timing is implausible.
fn decode_nec(d: &[u32], last: Option<Event>) -> Option<Event> {
    // Every NEC frame — data or repeat — starts with an ~9 ms mark.
    // 25% tolerance: 6750–11250 µs. Cheap remotes and distance stretch this.
    if !near(d[0], 9000, 25) {
        return None;
    }

    // Repeat: 9 ms mark + ~2.25 ms space, and almost no payload after.
    // `<= 6` allows a trailing stop mark and a bit of bounce without treating
    // a full 66-edge data frame as a repeat (those have a 4.5 ms space).
    if near(d[1], 2250, 30) && d.len() <= 6 {
        // `?` on `Option`: if we never stored a command, abort (`None`).
        // Repeats are meaningless without a prior button.
        let (addr, cmd) = last_nec(last)?;
        return Some(Event::Nec {
            addr,
            cmd,
            repeat: true,
        });
    }

    // Data frame: second pulse must be the ~4.5 ms leader space.
    if !near(d[1], 4500, 25) {
        return None;
    }

    let bits = decode_32_bits(d)?;
    Some(Event::Nec {
        addr: nec_address(bits),
        cmd: ((bits >> 16) & 0xFF) as u8,
        repeat: false,
    })
}

/// Samsung TV remotes: 4.5 ms mark + 4.5 ms space, then NEC-style bits.
fn decode_samsung(d: &[u32], last: Option<Event>) -> Option<Event> {
    if !near(d[0], 4500, 30) {
        return None;
    }

    if near(d[1], 4500, 30) && d.len() <= 8 {
        let (addr, cmd) = last_samsung(last)?;
        return Some(Event::Samsung {
            addr,
            cmd,
            repeat: true,
        });
    }

    if !near(d[1], 4500, 30) {
        return None;
    }

    let bits = decode_32_bits(d)?;
    Some(Event::Samsung {
        addr: (bits & 0xFFFF) as u16,
        cmd: ((bits >> 16) & 0xFF) as u8,
        repeat: false,
    })
}

/// Skip the two leader durations; pack 32 × (mark, space) into a `u32`.
///
/// Bit 0 is the first bit received (NEC and Samsung32 are LSB first).
fn decode_32_bits(d: &[u32]) -> Option<u32> {
    // 2 leader samples + 64 bit samples (32 marks + 32 spaces).
    if d.len() < 66 {
        return None;
    }
    let data = &d[2..];
    let mut bits: u32 = 0;
    for i in 0..32 {
        // Even index: the ~560 µs mark that starts every bit.
        let mark = data[i * 2];
        // Odd index: the space whose width is the actual 0/1.
        let space = data[i * 2 + 1];
        // ~55% window around 560 µs. Wider than the leader because short
        // pulses jitter more as a fraction of their length.
        if !near(mark, 560, 55) {
            return None;
        }
        // Classic NEC/Samsung: 0 ≈ 560 µs space, 1 ≈ 1690 µs space. A 1000 µs
        // split sits between them; we do not require the space to be “near”
        // either nominal, so slightly drunk remotes still decode.
        if space > 1000 {
            // Set bit `i`. First loop iteration (`i == 0`) is the least
            // significant bit of the address byte.
            bits |= 1 << i;
        }
        // Else leave bit `i` as 0. No `else` needed.
    }
    Some(bits)
}

fn nec_address(bits: u32) -> u16 {
    // Slice the 32-bit word into the four NEC bytes. Shifts are in bit
    // positions, not byte indexes: bits [0..8) are the first byte on the wire.
    let addr8 = (bits & 0xFF) as u8;
    let addr_inv = ((bits >> 8) & 0xFF) as u8;
    // We ignore bits [24..32), the inverted command, rather than rejecting
    // the frame if they do not match `!cmd`. That is a deliberate simplification.
    //
    // Classic NEC: `addr_inv` is `!addr8`, so XOR is all ones (`0xFF`).
    // Extended NEC: those two bytes are a 16-bit address, XOR is not `0xFF`.
    if addr8 ^ addr_inv == 0xFF {
        addr8 as u16
    } else {
        // Little-endian packing: low byte = first address byte on the wire.
        addr8 as u16 | (u16::from(addr_inv) << 8)
    }
}

/// True if `value` is within `pct` percent of `nominal` (inclusive).
///
/// Example: `near(9000, 9000, 25)` allows 6750…11250. `saturating_mul`
/// clamps on overflow instead of wrapping; with these IR constants it will
/// not overflow, but it is the safe way to write `a * b` on a microcontroller.
fn near(value: u32, nominal: u32, pct: u32) -> bool {
    // Lower bound: e.g. 9000 * 75 / 100 = 6750. Integer division truncates.
    let lo = nominal.saturating_mul(100 - pct) / 100;
    // Upper bound: e.g. 9000 * 125 / 100 = 11250.
    let hi = nominal.saturating_mul(100 + pct) / 100;
    value >= lo && value <= hi
}

/// One OLED line, at most 16 characters (this project's value row).
///
/// Examples: `20:0D`, `20:0D r`, `S E0E0:1A`, `4.5 4.5 67`.
fn write_hex_cmd(line: &mut String<16>, prefix: &str, addr: u16, cmd: u8, repeat: bool) {
    let _ = line.push_str(prefix);
    if addr > 0xFF {
        // Extended NEC / 16-bit Samsung: four hex digits of address, colon,
        // two of cmd. `let _ =` discards `Result`: if the string were full,
        // we silently keep the truncated prefix rather than panicking.
        let _ = write!(line, "{addr:04X}:{cmd:02X}");
    } else {
        // Classic 8-bit address: `20:0D` rather than `0020:0D`.
        let _ = write!(line, "{addr:02X}:{cmd:02X}");
    }
    if repeat {
        let _ = line.push_str(" r");
    }
}

pub fn oled_line(event: &Event) -> String<16> {
    // Empty stack string; `write!` appends UTF-8 bytes up to 16.
    let mut line = String::new();
    match *event {
        Event::Nec { addr, cmd, repeat } => {
            write_hex_cmd(&mut line, "", addr, cmd, repeat);
        }
        Event::Samsung { addr, cmd, repeat } => {
            write_hex_cmd(&mut line, "S ", addr, cmd, repeat);
        }
        Event::Raw { count, d0, d1 } => {
            // Leader pulses in milliseconds with one decimal, then edge count
            // if it still fits. Example: `4.5 4.5 67`.
            let _ = write!(
                line,
                "{}.{} {}.{}",
                d0 / 1000,
                (d0 / 100) % 10,
                d1 / 1000,
                (d1 / 100) % 10
            );
            if line.len() < 12 {
                let _ = write!(line, " {count}");
            }
        }
    }
    line
}

/// JSON body for the HTTP POST (capacity 96 is plenty for these tiny objects).
///
/// Numbers are decimal (not hex) so a listener can parse them as JSON numbers.
/// `rep` is a JSON boolean written as the literals `true` / `false`.
pub fn json_body(event: &Event) -> String<96> {
    let mut body = String::new();
    match *event {
        Event::Nec { addr, cmd, repeat } => {
            // Inner `{{\"...\"}}` because `format!`/`write!` treat `{`/`}` as
            // placeholders; doubling them emits a literal brace. The last `{}`
            // is the `true`/`false` argument below.
            let _ = write!(
                body,
                "{{\"proto\":\"nec\",\"addr\":{addr},\"cmd\":{cmd},\"rep\":{}}}",
                if repeat { "true" } else { "false" }
            );
        }
        Event::Samsung { addr, cmd, repeat } => {
            let _ = write!(
                body,
                "{{\"proto\":\"samsung\",\"addr\":{addr},\"cmd\":{cmd},\"rep\":{}}}",
                if repeat { "true" } else { "false" }
            );
        }
        Event::Raw { count, d0, d1 } => {
            // `lead` is just the first two pulse widths, not the whole burst.
            let _ = write!(
                body,
                "{{\"proto\":\"raw\",\"n\":{count},\"lead\":[{d0},{d1}]}}"
            );
        }
    }
    body
}

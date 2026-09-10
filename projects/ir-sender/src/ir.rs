//! Drive a 940 nm IR LED with a 38 kHz carrier and a NEC envelope.
//!
//! GP18 switches a BC337-40. The LED current comes from 3.3 V through two
//! 15 Ω resistors (~58 mA), not from the GPIO. GP18 only feeds the base
//! through 220 Ω. Idle is low (transistor off).
//!
//! NEC: 9000 µs mark, 4500 µs space, then 32 LSB-first bits (560 µs mark +
//! 560 µs space = 0, 560 + 1690 = 1), then a 560 µs stop mark. Bytes are
//! address, !address, command, !command.
//!
//! Each send is three copies of the frame so the TSOP AGC can lock.

use cortex_m::peripheral::DWT;
use embassy_rp::clocks::clk_sys_freq;
use embassy_rp::gpio::{Drive, Level, Output, SlewRate};
use embassy_rp::peripherals::PIN_18;
use embassy_rp::Peri;
use embassy_time::{block_for, Duration, Instant, Timer};

const CARRIER_HZ: u32 = 38_000;
const MARK_US: u64 = 560;
const ONE_SPACE_US: u64 = 1690;
const INTER_FRAME_MS: u64 = 40;

pub struct IrLed {
    pin: Output<'static>,
    on_cycles: u32,
    off_cycles: u32,
    dwt_ok: bool,
}

impl IrLed {
    pub fn new(gpio: Peri<'static, PIN_18>) -> Self {
        enable_cycle_counter();
        let mut pin = Output::new(gpio, Level::Low);
        pin.set_drive_strength(Drive::_12mA);
        pin.set_slew_rate(SlewRate::Fast);

        let period = (clk_sys_freq() / CARRIER_HZ).max(3);
        let on_cycles = period / 3;
        Self {
            pin,
            on_cycles,
            off_cycles: period - on_cycles,
            dwt_ok: dwt_running(),
        }
    }

    pub fn idle(&mut self) {
        self.pin.set_low();
    }

    /// Hold the transistor on (DC through the LED). Meter / wiring debug only.
    pub fn dc_on(&mut self) {
        self.pin.set_high();
    }

    /// 38 kHz for `ms`. Capture’s TSOP should pull its pin low for this whole
    /// window (OLED title `L` or `stuck L`).
    #[allow(dead_code)]
    pub fn carrier_ms(&mut self, ms: u64) {
        self.mark(ms.saturating_mul(1000));
        self.idle();
    }

    /// Three NEC frames. Capture prints `42:01`…`42:0A` for addr `0x42`, cmd `1`…`10`.
    ///
    /// Yields between copies so the button task can run; each frame itself is
    /// still a busy-wait (38 kHz timing).
    pub async fn send_nec(&mut self, addr: u8, cmd: u8) {
        self.send_one(addr, cmd);
        Timer::after(Duration::from_millis(INTER_FRAME_MS)).await;
        self.send_one(addr, cmd);
        Timer::after(Duration::from_millis(INTER_FRAME_MS)).await;
        self.send_one(addr, cmd);
        self.idle();
    }

    fn send_one(&mut self, addr: u8, cmd: u8) {
        let bits = u32::from(addr)
            | (u32::from(!addr) << 8)
            | (u32::from(cmd) << 16)
            | (u32::from(!cmd) << 24);

        self.mark(9000);
        self.space(4500);
        for i in 0..32 {
            self.mark(MARK_US);
            if (bits >> i) & 1 == 1 {
                self.space(ONE_SPACE_US);
            } else {
                self.space(MARK_US);
            }
        }
        self.mark(MARK_US);
        self.idle();
    }

    fn mark(&mut self, us: u64) {
        if self.dwt_ok {
            let pulses = (us * u64::from(CARRIER_HZ) / 1_000_000).max(1);
            let t0 = DWT::cycle_count();
            let mut due = 0u32;
            for _ in 0..pulses {
                self.pin.set_high();
                due = due.wrapping_add(self.on_cycles);
                while DWT::cycle_count().wrapping_sub(t0) < due {}
                self.pin.set_low();
                due = due.wrapping_add(self.off_cycles);
                while DWT::cycle_count().wrapping_sub(t0) < due {}
            }
            return;
        }

        let end = Instant::now() + Duration::from_micros(us);
        while Instant::now() < end {
            self.pin.set_high();
            let t = Instant::now() + Duration::from_micros(8);
            while Instant::now() < t {}
            self.pin.set_low();
            let t = Instant::now() + Duration::from_micros(18);
            while Instant::now() < t {}
        }
    }

    fn space(&mut self, us: u64) {
        self.idle();
        block_for(Duration::from_micros(us));
    }
}

fn dwt_running() -> bool {
    let a = DWT::cycle_count();
    cortex_m::asm::delay(50_000);
    DWT::cycle_count().wrapping_sub(a) > 100
}

fn enable_cycle_counter() {
    unsafe {
        let mut core = cortex_m::Peripherals::steal();
        core.DCB.enable_trace();
        core.DWT.enable_cycle_counter();
    }
}

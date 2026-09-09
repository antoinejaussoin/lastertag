//! Drive a 940 nm IR LED with a 38 kHz carrier and a NEC envelope.
//!
//! # What this pin does
//!
//! GP18 is a normal digital output. Firmware runs the Pico PWM block at
//! **38 kHz** (the frequency the Vishay TSOP38238 in `ir-capture` listens
//! for). A **mark** is “carrier on”: the pin sits at ~33% duty so the LED
//! flickers at 38 kHz. A **space** is “carrier off”: duty is 0, the pin stays
//! low, the LED is dark.
//!
//! Idle is also duty 0. The LED is wired GPIO → 220 Ω → anode, cathode → GND,
//! so a low pin means no current. Never leave the pin stuck high.
//!
//! This sandbox drives the LED from the GPIO through that one resistor
//! (~9 mA). The later player gun uses a transistor so it can pulse harder;
//! do not copy this GPIO-direct wiring onto that board, and do not replace
//! the 220 Ω with a 15 Ω part from the shopping list.
//!
//! # NEC frame
//!
//! Same timings `ir-capture` already decodes:
//!
//! 1. Leader: 9000 µs mark, 4500 µs space.
//! 2. 32 bits, least-significant bit first. Each bit is a 560 µs mark then a
//!    space: 560 µs means `0`, 1690 µs means `1`.
//! 3. A final 560 µs mark (stop bit).
//!
//! The 32 bits are address, inverted address, command, inverted command.
//! This crate always sends classic 8-bit NEC (`addr` XOR `!addr` is `0xFF`),
//! so capture shows two hex digits of address, not four.
//!
//! Timings are busy-waited with [`embassy_time::Instant`]. The burst is only
//! ~70 ms; blocking the executor for that long is fine here.

use embassy_rp::clocks::clk_sys_freq;
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_time::Instant;

/// Carrier the TSOP38238 is tuned for.
const CARRIER_HZ: u32 = 38_000;

/// NEC mark / zero-space width, microseconds.
const MARK_US: u64 = 560;

/// NEC one-space width, microseconds.
const ONE_SPACE_US: u64 = 1690;

/// PWM1A on GP18: 38 kHz, idle (LED off).
pub fn pwm_config() -> Config {
    let mut cfg = Config::default();
    // `clk_sys / ((top + 1) * divider)`, divider is 1.
    // Pico 2 W Embassy default is 150 MHz → top 3946 → ~38.004 kHz.
    cfg.top = (clk_sys_freq() / CARRIER_HZ).saturating_sub(1) as u16;
    cfg.compare_a = 0;
    cfg.enable = true;
    cfg
}

/// One classic NEC frame. `addr` and `cmd` are the values capture will print.
pub fn send_nec(pwm: &mut Pwm<'_>, addr: u8, cmd: u8) {
    // ~1/3 duty is the usual IR-remote carrier. `SetDutyCycle` rejects a
    // value above `top`; `top / 3` is safely inside that.
    let duty = pwm.max_duty_cycle() / 3;
    // Wire order, LSB first: addr, !addr, cmd, !cmd.
    let bits = u32::from(addr)
        | (u32::from(!addr) << 8)
        | (u32::from(cmd) << 16)
        | (u32::from(!cmd) << 24);

    mark(pwm, duty, 9000);
    space(pwm, 4500);
    for i in 0..32 {
        mark(pwm, duty, MARK_US);
        if (bits >> i) & 1 == 1 {
            space(pwm, ONE_SPACE_US);
        } else {
            space(pwm, MARK_US);
        }
    }
    mark(pwm, duty, MARK_US);
    let _ = pwm.set_duty_cycle(0);
}

fn mark(pwm: &mut Pwm<'_>, duty: u16, us: u64) {
    let _ = pwm.set_duty_cycle(duty);
    busy_wait_us(us);
}

fn space(pwm: &mut Pwm<'_>, us: u64) {
    let _ = pwm.set_duty_cycle(0);
    busy_wait_us(us);
}

fn busy_wait_us(us: u64) {
    let start = Instant::now();
    while start.elapsed().as_micros() < us {}
}

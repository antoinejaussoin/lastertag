//! Pico 2 W: button press sends a NEC IR burst to ir-capture.

#![no_std]
#![no_main]

mod display;
mod ir;

use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::i2c::{self, I2c};
use embassy_time::Timer;
use panic_halt as _;

#[unsafe(link_section = ".start_block")]
#[used]
static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[unsafe(link_section = ".bi_entries")]
#[used]
static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"ir-sender"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_description!(c"Pico 2 W NEC IR sender + OLED"),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

/// Classic NEC pair that ir-capture prints as `42:01`.
const ADDR: u8 = 0x42;
const CMD: u8 = 0x01;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // GP18 → 220 Ω → BC337 base. LED current is from 3.3 V. Idle is low.
    let mut ir = ir::IrLed::new(p.PIN_18);
    // GP19 to GND through the tactile switch. Pull-up so open = high.
    let mut button = Input::new(p.PIN_19, Pull::Up);

    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100_000;
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);
    let mut screen = display::Screen::new(i2c);
    let mut presses: u32 = 0;

    // 38 kHz on/off so the *capture* TSOP is the detector. A phone camera
    // often shows nothing at 9 mA / 940 nm. Keep the phone away from the TSOP.
    screen.show("aim TSOP", "carrier on", presses);
    for _ in 0..8 {
        screen.show("aim TSOP", "carrier ON", presses);
        ir.carrier_ms(400);
        screen.show("aim TSOP", "carrier off", presses);
        Timer::after_millis(400).await;
    }
    'glow: loop {
        screen.show("aim TSOP", "carrier ON", presses);
        ir.carrier_ms(400);
        screen.show("aim TSOP", "carrier off", presses);
        for _ in 0..40 {
            Timer::after_millis(10).await;
            if button.is_low() {
                break 'glow;
            }
        }
    }
    ir.idle();
    Timer::after_millis(40).await;
    presses = presses.saturating_add(1);

    screen.show("hello IR", "sending", presses);
    ir.send_nec(ADDR, CMD);
    screen.show("ready", "press btn", presses);

    loop {
        button.wait_for_high().await;
        Timer::after_millis(40).await;
        button.wait_for_falling_edge().await;
        Timer::after_millis(40).await;
        if button.is_high() {
            continue;
        }

        presses = presses.saturating_add(1);
        screen.show("sent 42:01", "sending", presses);
        ir.send_nec(ADDR, CMD);
        screen.show("sent 42:01", "press btn", presses);
    }
}

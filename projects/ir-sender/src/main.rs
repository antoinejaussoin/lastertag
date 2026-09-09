//! Pico 2 W: button press sends a NEC IR burst to ir-capture.

#![no_std]
#![no_main]

mod display;
mod ir;

use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::i2c::{self, I2c};
use embassy_rp::pwm::Pwm;
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

    // GP18 / PWM1A → 220 Ω → TSAL6200 anode. Idle duty is 0 (pin low).
    let mut pwm = Pwm::new_output_a(p.PWM_SLICE1, p.PIN_18, ir::pwm_config());
    // GP19 to GND through the tactile switch. Pull-up so open = high.
    let mut button = Input::new(p.PIN_19, Pull::Up);

    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100_000;
    // Same OLED pins as ir-capture: GP17 SCL, GP16 SDA.
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);
    let mut screen = display::Screen::new(i2c);
    screen.show("ready", "press btn");

    loop {
        button.wait_for_falling_edge().await;
        Timer::after_millis(40).await;
        if button.is_high() {
            continue;
        }

        screen.show("sent 42:01", "sending");
        ir::send_nec(&mut pwm, ADDR, CMD);
        screen.show("sent 42:01", "press btn");

        button.wait_for_high().await;
        Timer::after_millis(40).await;
    }
}

//! Pico 2 W: chip temperature on SH1106 OLED, USB provisioning, Wi-Fi POST.

#![no_std]
#![no_main]

mod board;
mod cli;
mod display;
mod settings;
mod temp;
mod wifi;

use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::flash::Flash;
use embassy_rp::i2c::{self, I2c};
use embassy_time::Timer;
use panic_halt as _;

use crate::settings::ConfigFlash;

#[unsafe(link_section = ".start_block")]
#[used]
static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[unsafe(link_section = ".bi_entries")]
#[used]
static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"temperature-display"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_description!(c"Pico 2 W chip temp + Wi-Fi POST"),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut flash: ConfigFlash = Flash::new_blocking(p.FLASH);
    settings::replace(settings::load_flash(&mut flash)).await;

    let stack = wifi::start(
        spawner, p.PIN_23, p.PIN_25, p.PIN_24, p.PIN_29, p.PIO0, p.DMA_CH0,
    )
    .await;
    cli::start(spawner, p.USB, flash);

    let mut chip_temp = temp::ChipTemp::new(p.ADC, p.ADC_TEMP_SENSOR);
    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100_000;
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);
    let mut screen = display::Screen::new(i2c);

    let mut seconds: u8 = 0;
    loop {
        let celsius = chip_temp.read_celsius();
        screen.show(&temp::format_celsius(celsius), wifi::status_line());

        seconds = seconds.wrapping_add(1);
        if seconds >= 60 {
            seconds = 0;
            wifi::post_temperature(stack, celsius).await;
        }

        Timer::after_secs(1).await;
    }
}

//! Pico 2 W: TSOP38238 IR capture on SH1106 OLED, USB provisioning, Wi-Fi POST.

#![no_std]
#![no_main]

mod board;
mod cli;
mod display;
mod ir;
mod settings;
mod wifi;

use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_rp::block::ImageDef;
use embassy_rp::flash::Flash;
use embassy_rp::gpio::{Input, Pull};
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
    embassy_rp::binary_info::rp_program_name!(c"ir-capture"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_description!(c"Pico 2 W TSOP38238 capture + Wi-Fi POST"),
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

    let mut ir_pin = Input::new(p.PIN_15, Pull::Up);
    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100_000;
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);
    let mut screen = display::Screen::new(i2c);

    let mut last_nec: Option<(u16, u8)> = None;
    let mut main_line = heapless::String::<16>::new();
    let _ = main_line.push_str("ready");
    screen.show(main_line.as_str(), wifi::status_line());

    loop {
        match select(ir::wait_and_capture(&mut ir_pin), Timer::after_millis(500)).await {
            Either::First(frame) => {
                let Some(event) = ir::decode(&frame, last_nec) else {
                    continue;
                };
                if let ir::Event::Nec { addr, cmd, repeat } = event {
                    if !repeat {
                        last_nec = Some((addr, cmd));
                    }
                }
                main_line = ir::oled_line(&event);
                screen.show(main_line.as_str(), wifi::status_line());
                wifi::post_json(stack, ir::json_body(&event).as_str()).await;
                screen.show(main_line.as_str(), wifi::status_line());
            }
            Either::Second(()) => {
                screen.show(main_line.as_str(), wifi::status_line());
            }
        }
    }
}

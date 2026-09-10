//! Pico 2 W: button press sends a NEC IR burst. Optional USB serial debug mode.

#![no_std]
#![no_main]

mod cli;
mod display;
mod ir;
mod settings;

use embassy_executor::Spawner;
use embassy_futures::select::{Either, Either3, select, select3};
use embassy_rp::block::ImageDef;
use embassy_rp::flash::Flash;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::i2c::{self, I2c};
use embassy_time::{Duration, Timer};
use panic_halt as _;

use crate::settings::ConfigFlash;

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

enum DebugLed {
    AlwaysOn,
    Beacon,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut flash: ConfigFlash = Flash::new_blocking(p.FLASH);
    settings::replace(settings::load_flash(&mut flash)).await;
    cli::start(spawner, p.USB, flash);

    // GP18 → 220 Ω → BC337 base. LED current is from 3.3 V. Idle is low.
    let mut ir = ir::IrLed::new(p.PIN_18);
    // GP19 to GND through the tactile switch. Pull-up so open = high.
    let mut button = Input::new(p.PIN_19, Pull::Up);

    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100_000;
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);
    let mut screen = display::Screen::new(i2c);
    let mut presses: u32 = 0;

    loop {
        if settings::snapshot().await.debug {
            run_debug(&mut ir, &mut button, &mut screen, &mut presses).await;
            ir.idle();
        } else {
            run_normal(&mut ir, &mut button, &mut screen, &mut presses).await;
            ir.idle();
        }
    }
}

/// Button click sends one NEC burst. USB `debug on` + `save` leaves this loop.
async fn run_normal(
    ir: &mut ir::IrLed,
    button: &mut Input<'_>,
    screen: &mut display::Screen,
    presses: &mut u32,
) {
    screen.show("ready", "press btn", *presses);
    loop {
        match select(wait_click(button), settings::wait_change()).await {
            Either::First(()) => {
                *presses = presses.saturating_add(1);
                screen.show("sent 42:01", "sending", *presses);
                ir.send_nec(ADDR, CMD);
                screen.show("sent 42:01", "press btn", *presses);
            }
            Either::Second(()) => return,
        }
    }
}

/// Button toggles LED DC on vs NEC every 1 s. USB `debug off` + `save` leaves.
async fn run_debug(
    ir: &mut ir::IrLed,
    button: &mut Input<'_>,
    screen: &mut display::Screen,
    presses: &mut u32,
) {
    let mut mode = DebugLed::AlwaysOn;
    loop {
        if !settings::snapshot().await.debug {
            return;
        }
        match mode {
            DebugLed::AlwaysOn => {
                ir.dc_on();
                screen.show("always on", "debug DC", *presses);
                match select(wait_click(button), settings::wait_change()).await {
                    Either::First(()) => {
                        ir.idle();
                        *presses = presses.saturating_add(1);
                        mode = DebugLed::Beacon;
                    }
                    Either::Second(()) => {
                        if !settings::snapshot().await.debug {
                            ir.idle();
                            return;
                        }
                    }
                }
            }
            DebugLed::Beacon => {
                screen.show("every 1s", "debug NEC", *presses);
                loop {
                    if !settings::snapshot().await.debug {
                        ir.idle();
                        return;
                    }
                    screen.show("every 1s", "sending", *presses);
                    ir.send_nec(ADDR, CMD);
                    screen.show("every 1s", "debug NEC", *presses);
                    match select3(
                        wait_click(button),
                        Timer::after(Duration::from_secs(1)),
                        settings::wait_change(),
                    )
                    .await
                    {
                        Either3::First(()) => {
                            ir.idle();
                            *presses = presses.saturating_add(1);
                            mode = DebugLed::AlwaysOn;
                            break;
                        }
                        Either3::Second(()) => {}
                        Either3::Third(()) => {
                            if !settings::snapshot().await.debug {
                                ir.idle();
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn wait_click(button: &mut Input<'_>) {
    button.wait_for_high().await;
    Timer::after_millis(40).await;
    button.wait_for_falling_edge().await;
    Timer::after_millis(40).await;
}

//! Pico 2 W: button press sends a NEC IR burst. Optional USB serial debug mode.

#![no_std]
#![no_main]

mod board;
mod cli;
mod display;
mod ir;
mod led;
mod settings;

use core::fmt::Write as _;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_executor::Spawner;
use embassy_futures::select::{Either3, select3};
use embassy_rp::block::ImageDef;
use embassy_rp::clocks::RoscRng;
use embassy_rp::flash::Flash;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::i2c::{self, I2c};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use heapless::String;
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

/// NEC address. Command is rolled 1..=10 on each send.
const ADDR: u8 = 0x42;

/// Clicks from the button task. Capacity covers presses during a NEC send.
static CLICKS: Channel<CriticalSectionRawMutex, (), 8> = Channel::new();
/// `true` while GP19 is low (button closed or wired as a dead short).
static BTN_HELD: AtomicBool = AtomicBool::new(false);

enum DebugLed {
    AlwaysOn,
    Beacon,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut flash: ConfigFlash = Flash::new_blocking(p.FLASH);
    settings::replace(settings::load_flash(&mut flash)).await;
    led::start(
        spawner, p.PIN_23, p.PIN_25, p.PIN_24, p.PIN_29, p.PIO0, p.DMA_CH0,
    )
    .await;
    cli::start(spawner, p.USB, flash);

    // GP18 → 220 Ω → BC337 base. LED current is from 3.3 V. Idle is low.
    let mut ir = ir::IrLed::new(p.PIN_18);
    // GP19 to GND through the tactile switch. Pull-up so open = high.
    let mut button = Input::new(p.PIN_19, Pull::Up);
    // Schmitt rejects weak/slow breadboard contacts. Off so a brief press counts.
    button.set_schmitt(false);
    spawner.spawn(button_task(button).unwrap());

    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100_000;
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);
    let mut screen = display::Screen::new(i2c);
    let mut presses: u32 = 0;

    loop {
        if settings::snapshot().await.debug {
            run_debug(&mut ir, &mut screen, &mut presses).await;
            ir.idle();
        } else {
            run_normal(&mut ir, &mut screen, &mut presses).await;
            ir.idle();
        }
    }
}

/// Button click sends one NEC burst. USB `debug on` + `save` leaves this loop.
async fn run_normal(
    ir: &mut ir::IrLed,
    screen: &mut display::Screen,
    presses: &mut u32,
) {
    drain_clicks();
    let mut line = idle_line();
    screen.show(line.as_str(), btn_status(), *presses, btn_high());
    loop {
        match select3(
            CLICKS.receive(),
            settings::wait_change(),
            Timer::after_millis(200),
        )
        .await
        {
            Either3::First(()) => {
                *presses = presses.saturating_add(1);
                let cmd = random_cmd();
                line = sent_line(cmd);
                screen.show(line.as_str(), btn_status(), *presses, btn_high());
                ir.send_nec(ADDR, cmd).await;
            }
            Either3::Second(()) => return,
            Either3::Third(()) => {
                if line.as_str() == "ready" || line.as_str() == "unplug j31" {
                    line = idle_line();
                }
                screen.show(line.as_str(), btn_status(), *presses, btn_high());
            }
        }
    }
}

/// Button toggles LED DC on vs NEC every 1 s. USB `debug off` + `save` leaves.
async fn run_debug(
    ir: &mut ir::IrLed,
    screen: &mut display::Screen,
    presses: &mut u32,
) {
    drain_clicks();
    let mut mode = DebugLed::AlwaysOn;
    loop {
        if !settings::snapshot().await.debug {
            return;
        }
        match mode {
            DebugLed::AlwaysOn => {
                ir.dc_on();
                screen.show("always on", "debug DC", *presses, btn_high());
                match select3(
                    CLICKS.receive(),
                    settings::wait_change(),
                    Timer::after_millis(200),
                )
                .await
                {
                    Either3::First(()) => {
                        ir.idle();
                        *presses = presses.saturating_add(1);
                        mode = DebugLed::Beacon;
                    }
                    Either3::Second(()) => {
                        if !settings::snapshot().await.debug {
                            ir.idle();
                            return;
                        }
                    }
                    Either3::Third(()) => {
                        screen.show("always on", "debug DC", *presses, btn_high());
                    }
                }
            }
            DebugLed::Beacon => {
                screen.show("every 1s", "debug NEC", *presses, btn_high());
                loop {
                    if !settings::snapshot().await.debug {
                        ir.idle();
                        return;
                    }
                    let cmd = random_cmd();
                    screen.show("every 1s", "sending", *presses, btn_high());
                    ir.send_nec(ADDR, cmd).await;
                    screen.show("every 1s", "debug NEC", *presses, btn_high());
                    match select3(
                        CLICKS.receive(),
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

fn drain_clicks() {
    while CLICKS.try_receive().is_ok() {}
}

/// GPIO interrupt on low. Do not require the pin to stay low for tens of
/// milliseconds — a tactile contact on a breadboard is often shorter than that.
#[embassy_executor::task]
async fn button_task(mut button: Input<'static>) {
    loop {
        while button.is_low() {
            BTN_HELD.store(true, Ordering::Relaxed);
            Timer::after_millis(10).await;
        }
        BTN_HELD.store(false, Ordering::Relaxed);
        button.wait_for_low().await;
        BTN_HELD.store(true, Ordering::Relaxed);
        led::flash();
        let _ = CLICKS.try_send(());
        Timer::after_millis(30).await;
    }
}

fn btn_high() -> bool {
    !BTN_HELD.load(Ordering::Relaxed)
}

fn btn_status() -> &'static str {
    if btn_high() {
        "press btn"
    } else {
        "GND is d29"
    }
}

fn idle_line() -> String<12> {
    let mut line = String::new();
    let _ = line.push_str(if btn_high() { "ready" } else { "unplug j31" });
    line
}

fn random_cmd() -> u8 {
    (RoscRng::next_u8() % 10) + 1
}

fn sent_line(cmd: u8) -> String<12> {
    let mut line = String::new();
    let _ = write!(line, "sent 42:{cmd:02X}");
    line
}

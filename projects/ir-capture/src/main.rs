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

    // GP18 (physical pin 24) is wired to the TSOP38238 OUT pin.
    //
    // `Input` means this GPIO only *reads* voltage; we never drive 3.3 V or
    // 0 V ourselves (that would fight the sensor). `Pull::Up` enables the
    // Pico's internal resistor to 3.3 V so an idle or disconnected line reads
    // high. The TSOP is idle-high anyway and has its own pull-up; this is
    // belt and braces. See `ir.rs` for how high/low map onto IR marks/spaces.
    let mut ir_pin = Input::new(p.PIN_18, Pull::Up);
    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100_000;
    // Same OLED pins as temperature-display: GP17 SCL, GP16 SDA.
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);
    let mut screen = display::Screen::new(i2c);

    // Last fully-decoded NEC or Samsung address/command. Repeat frames
    // (button held) do not contain those bytes, so decode() reuses this
    // event. `None` until the first non-repeat success.
    let mut last: Option<ir::Event> = None;
    let mut main_line = heapless::String::<16>::new();
    let _ = main_line.push_str("ready");
    screen.show(main_line.as_str(), wifi::status_line(), ir_pin.is_high());

    loop {
        // Race the IR *wait* against a 300 ms tick — not the capture itself.
        //
        // `select` completes when *either* future finishes. The IR side sleeps
        // until the TSOP pin falls. Capture is a busy loop and must not be
        // cancelled by the timer. The tick refreshes the OLED Wi-Fi/POST
        // status and the title H/L pin indicator even when nobody is pressing
        // a remote. Whichever loses is dropped; that is fine: a new
        // `wait_for_falling_edge` is created next iteration, and the timer is
        // restarted.
        match select(ir_pin.wait_for_falling_edge(), Timer::after_millis(300)).await {
            Either::First(()) => {
                let frame = ir::capture_busy(&ir_pin);
                // Falling edge then almost no further edges: pin went low and
                // stayed there (short, swapped VS/OUT, etc.).
                if frame.durations_us.len() < 2 {
                    screen.show("stuck L", wifi::status_line(), ir_pin.is_high());
                    Timer::after_millis(200).await;
                    continue;
                }

                let event = ir::decode(&frame, last);
                // Remember address/command only for a full frame, never for a
                // repeat (that would just write the same pair again).
                match event {
                    ir::Event::Nec {
                        repeat: false, ..
                    }
                    | ir::Event::Samsung {
                        repeat: false, ..
                    } => {
                        last = Some(event);
                    }
                    _ => {}
                }
                main_line = ir::oled_line(&event);
                screen.show(main_line.as_str(), wifi::status_line(), ir_pin.is_high());

                // Skip POSTing tiny noise bursts. Structured NEC/Samsung
                // always go out; `Raw` only if it looks like a real frame.
                let post = match event {
                    ir::Event::Raw { count, d0, .. } => count >= 8 && d0 >= 800,
                    _ => true,
                };
                if post {
                    // Fire-and-forget from the UI's point of view: we still
                    // await the TCP POST so the next status refresh can show
                    // `post ok`.
                    wifi::post_json(stack, ir::json_body(&event).as_str()).await;
                    // POST may have changed `wifi::status_line()` (`post ok` / fail).
                    screen.show(main_line.as_str(), wifi::status_line(), ir_pin.is_high());
                }
            }
            Either::Second(()) => {
                // Idle tick: keep the bottom status row and pin H/L current.
                screen.show(main_line.as_str(), wifi::status_line(), ir_pin.is_high());
            }
        }
    }
}

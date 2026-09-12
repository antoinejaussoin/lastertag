//! USB CDC serial CLI: `debug on` / `debug off`, `save`, `show`, `clear`, `help`.

use core::fmt::Write as _;
use embassy_executor::Spawner;
use embassy_rp::Peri;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::{Builder, Config, UsbDevice};
use heapless::String;
use static_cell::StaticCell;

use crate::board::Irqs;
use crate::settings::{self, ConfigFlash};

type UsbDriver = Driver<'static, USB>;

pub fn start(spawner: Spawner, usb: Peri<'static, USB>, flash: ConfigFlash) {
    let driver = Driver::new(usb, Irqs);
    let mut config = Config::new(0xc0de, 0xcaee);
    config.manufacturer = Some("lastertag");
    config.product = Some("Pico 2 W gun");
    config.serial_number = Some("0001");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static MSOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();
    static STATE: StaticCell<State> = StaticCell::new();

    let mut builder = Builder::new(
        driver,
        config,
        CONFIG_DESC.init([0; 256]),
        BOS_DESC.init([0; 256]),
        MSOS_DESC.init([0; 256]),
        CONTROL_BUF.init([0; 64]),
    );
    let class = CdcAcmClass::new(&mut builder, STATE.init(State::new()), 64);
    let usb_dev = builder.build();

    spawner.spawn(usb_task(usb_dev).unwrap());
    spawner.spawn(cli_task(class, flash).unwrap());
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, UsbDriver>) -> ! {
    usb.run().await
}

#[embassy_executor::task]
async fn cli_task(class: CdcAcmClass<'static, UsbDriver>, flash: ConfigFlash) -> ! {
    run_cli(class, flash).await
}

async fn run_cli(mut class: CdcAcmClass<'static, UsbDriver>, mut flash: ConfigFlash) -> ! {
    loop {
        class.wait_connection().await;
        let mut line = String::<160>::new();
        let mut buf = [0u8; 64];
        let _ = write_text(&mut class, "\r\nPico 2 W gun. Type help.\r\n> ").await;
        loop {
            match class.read_packet(&mut buf).await {
                Ok(n) => {
                    for &b in &buf[..n] {
                        match b {
                            b'\r' | b'\n' => {
                                let _ = class.write_packet(b"\r\n").await;
                                if !line.is_empty() {
                                    handle_line(&mut class, &mut flash, line.as_str()).await;
                                    line.clear();
                                }
                                let _ = write_text(&mut class, "> ").await;
                            }
                            0x08 | 0x7f => {
                                if line.pop().is_some() {
                                    let _ = write_text(&mut class, "\x08 \x08").await;
                                }
                            }
                            b if b.is_ascii_graphic() || b == b' ' => {
                                if line.push(b as char).is_ok() {
                                    let _ = class.write_packet(&[b]).await;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }
}

async fn handle_line(
    class: &mut CdcAcmClass<'static, UsbDriver>,
    flash: &mut ConfigFlash,
    line: &str,
) {
    let line = line.trim();
    let (cmd, rest) = split_cmd(line);
    match cmd {
        "help" | "?" => {
            let _ = write_text(
                class,
                "debug on         LED DC / 1s burst (button toggles)\r\ndebug off        button sends one NEC (default)\r\nsave             store in flash and apply now\r\nshow\r\nclear            erase saved settings\r\n",
            )
            .await;
        }
        "debug" => match rest {
            "on" => {
                settings::update(|cfg| cfg.debug = true).await;
                let _ = write_text(class, "ok. type save when ready.\r\n").await;
            }
            "off" => {
                settings::update(|cfg| cfg.debug = false).await;
                let _ = write_text(class, "ok. type save when ready.\r\n").await;
            }
            _ => {
                let _ = write_text(class, "usage: debug on | debug off\r\n").await;
            }
        },
        "show" => {
            let cfg = settings::snapshot().await;
            let mut msg = String::<64>::new();
            let _ = write!(
                msg,
                "debug: {}\r\n",
                if cfg.debug { "on" } else { "off" }
            );
            let _ = write_text(class, msg.as_str()).await;
        }
        "save" => {
            let cfg = settings::snapshot().await;
            match settings::save_flash(flash, &cfg) {
                Ok(()) => {
                    settings::notify();
                    let _ = write_text(class, "saved.\r\n").await;
                }
                Err(_) => {
                    let _ = write_text(class, "flash write failed.\r\n").await;
                }
            }
        }
        "clear" => {
            settings::replace(settings::Config::empty()).await;
            let _ = settings::erase_flash(flash);
            settings::notify();
            let _ = write_text(class, "cleared.\r\n").await;
        }
        _ => {
            let _ = write_text(class, "unknown command. type help.\r\n").await;
        }
    }
}

fn split_cmd(line: &str) -> (&str, &str) {
    match line.split_once(char::is_whitespace) {
        Some((cmd, rest)) => (cmd, rest.trim()),
        None => (line, ""),
    }
}

async fn write_text(class: &mut CdcAcmClass<'static, UsbDriver>, text: &str) -> Result<(), ()> {
    for chunk in text.as_bytes().chunks(64) {
        class.write_packet(chunk).await.map_err(|_| ())?;
    }
    Ok(())
}

#![no_std]
#![no_main]

mod config;

use core::fmt::Write as _;
use core::sync::atomic::{AtomicU8, Ordering};
use cyw43::JoinOptions;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_net::dns::DnsQueryType;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config as NetStackConfig, IpEndpoint, Stack, StackResources};
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::dma;
use embassy_rp::flash::Flash;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{Config as I2cConfig, I2c};
use embassy_rp::peripherals::{DMA_CH0, PIO0, USB};
use embassy_rp::pio::{InterruptHandler as PioInterruptHandler, Pio};
use embassy_rp::usb::{Driver, InterruptHandler as UsbInterruptHandler};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::{Builder, UsbDevice};
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_10X20};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};
use embedded_io_async::Write;
use heapless::String;
use oled_i2c::{Oled, OledConfig};
use panic_halt as _;
use static_cell::StaticCell;

use crate::config::{
    ConfigFlash, FLASH_SIZE, NetConfig, erase as erase_config, load as load_config, parse_server,
    save as save_config,
};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

#[unsafe(link_section = ".start_block")]
#[used]
static IMAGE_DEF: embassy_rp::block::ImageDef = embassy_rp::block::ImageDef::secure_exe();

#[unsafe(link_section = ".bi_entries")]
#[used]
static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 3] = [
    embassy_rp::binary_info::rp_program_name!(c"temp-oled-experiment pico2w"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

type UsbDriver = Driver<'static, USB>;

static CONFIG: Mutex<CriticalSectionRawMutex, NetConfig> = Mutex::new(NetConfig::empty());
static JOIN: Signal<CriticalSectionRawMutex, ()> = Signal::new();
static WIFI_STATUS: AtomicU8 = AtomicU8::new(ST_SETUP);
static POST_STATUS: AtomicU8 = AtomicU8::new(POST_NONE);

const ST_SETUP: u8 = 0;
const ST_JOINING: u8 = 1;
const ST_UP: u8 = 2;
const ST_FAIL: u8 = 3;
const POST_NONE: u8 = 0;
const POST_OK: u8 = 1;
const POST_FAIL: u8 = 2;

/// Convert a Pico 2 W (RP2350) temperature-sensor ADC reading to Celsius.
///
/// Same approximation as the Pico SDK: T = 27 - (V - 0.706) / 0.001721.
fn adc_to_celsius(raw: u16) -> f32 {
    27.0 - (raw as f32 * 3.3 / 4096.0 - 0.706) / 0.001721
}

fn format_temp(celsius: f32) -> String<16> {
    let mut line: String<16> = String::new();
    let tenths = (celsius * 10.0) as i32;
    let whole = tenths / 10;
    let frac = tenths.abs() % 10;
    let _ = write!(line, "{whole}.{frac} C");
    line
}

fn status_line() -> &'static str {
    match WIFI_STATUS.load(Ordering::Relaxed) {
        ST_JOINING => "joining wifi",
        ST_UP => match POST_STATUS.load(Ordering::Relaxed) {
            POST_OK => "wifi ok  post ok",
            POST_FAIL => "wifi ok  post fail",
            _ => "wifi ok",
        },
        ST_FAIL => "wifi fail",
        _ => "USB: wifi/save",
    }
}

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, UsbDriver>) -> ! {
    usb.run().await
}

#[embassy_executor::task]
async fn wifi_task(mut control: cyw43::Control<'static>, stack: Stack<'static>) -> ! {
    loop {
        let (ssid, psk) = {
            let cfg = CONFIG.lock().await;
            (cfg.ssid.clone(), cfg.psk.clone())
        };

        if ssid.is_empty() {
            WIFI_STATUS.store(ST_SETUP, Ordering::Relaxed);
            let _ = control.gpio_set(0, false).await;
            JOIN.wait().await;
            continue;
        }

        WIFI_STATUS.store(ST_JOINING, Ordering::Relaxed);
        POST_STATUS.store(POST_NONE, Ordering::Relaxed);
        control.leave().await;
        Timer::after_millis(200).await;

        let join = if psk.is_empty() {
            control.join(ssid.as_str(), JoinOptions::new_open()).await
        } else {
            control
                .join(ssid.as_str(), JoinOptions::new(psk.as_bytes()))
                .await
        };

        let up = match join {
            Ok(()) => embassy_time::with_timeout(Duration::from_secs(20), stack.wait_config_up())
                .await
                .is_ok(),
            Err(_) => false,
        };

        if up {
            WIFI_STATUS.store(ST_UP, Ordering::Relaxed);
            control.gpio_set(0, true).await;
            JOIN.wait().await;
        } else {
            WIFI_STATUS.store(ST_FAIL, Ordering::Relaxed);
            control.gpio_set(0, false).await;
            match select(JOIN.wait(), Timer::after_secs(15)).await {
                Either::First(()) | Either::Second(()) => {}
            }
        }
    }
}

#[embassy_executor::task]
async fn cli_task(mut class: CdcAcmClass<'static, UsbDriver>, mut flash: ConfigFlash) -> ! {
    loop {
        class.wait_connection().await;
        let _ = write_text(&mut class, "\r\nPico 2 W temp setup. Type help\r\n> ").await;
        if run_cli(&mut class, &mut flash).await.is_err() {
            continue;
        }
    }
}

async fn run_cli(
    class: &mut CdcAcmClass<'static, UsbDriver>,
    flash: &mut ConfigFlash,
) -> Result<(), EndpointError> {
    let mut line: String<192> = String::new();
    let mut pkt = [0u8; 64];
    loop {
        let n = class.read_packet(&mut pkt).await?;
        for &b in &pkt[..n] {
            match b {
                b'\r' | b'\n' => {
                    if !line.is_empty() {
                        write_text(class, "\r\n").await?;
                        handle_line(class, flash, line.as_str()).await?;
                        line.clear();
                    }
                    write_text(class, "> ").await?;
                }
                0x08 | 0x7f => {
                    if line.pop().is_some() {
                        write_text(class, "\x08 \x08").await?;
                    }
                }
                b if b.is_ascii_graphic() || b == b' ' => {
                    if line.push(char::from(b)).is_ok() {
                        class.write_packet(&[b]).await?;
                    }
                }
                _ => {}
            }
        }
    }
}

async fn handle_line(
    class: &mut CdcAcmClass<'static, UsbDriver>,
    flash: &mut ConfigFlash,
    line: &str,
) -> Result<(), EndpointError> {
    let line = line.trim();
    if line.is_empty() {
        return Ok(());
    }
    let cmd = line.split_once(' ').map(|(c, _)| c).unwrap_or(line);
    let rest = line.split_once(' ').map(|(_, r)| r.trim()).unwrap_or("");

    if cmd.eq_ignore_ascii_case("help") || cmd == "?" {
        write_text(
            class,
            "wifi <ssid>\r\n\
             psk <password>   (bare psk = open network)\r\n\
             server <host:port/path>\r\n\
             save             write flash and join\r\n\
             show\r\n\
             clear            erase saved settings\r\n\
             help\r\n",
        )
        .await
    } else if cmd.eq_ignore_ascii_case("wifi") {
        if rest.is_empty() {
            return write_text(class, "usage: wifi <ssid>\r\n").await;
        }
        let mut cfg = CONFIG.lock().await;
        cfg.ssid.clear();
        if cfg.ssid.push_str(rest).is_err() {
            return write_text(class, "ssid too long (max 32)\r\n").await;
        }
        write_text(class, "ssid set (save to persist)\r\n").await
    } else if cmd.eq_ignore_ascii_case("psk") {
        let mut cfg = CONFIG.lock().await;
        cfg.psk.clear();
        if !rest.is_empty() && cfg.psk.push_str(rest).is_err() {
            return write_text(class, "psk too long (max 64)\r\n").await;
        }
        if rest.is_empty() {
            write_text(class, "open network (save to persist)\r\n").await
        } else {
            write_text(class, "psk set (save to persist)\r\n").await
        }
    } else if cmd.eq_ignore_ascii_case("server") {
        if rest.is_empty() || parse_server(rest).is_none() {
            return write_text(class, "usage: server 192.168.1.10:8080/temp\r\n").await;
        }
        let mut cfg = CONFIG.lock().await;
        cfg.server.clear();
        if cfg.server.push_str(rest).is_err() {
            return write_text(class, "server too long\r\n").await;
        }
        write_text(class, "server set (save to persist)\r\n").await
    } else if cmd.eq_ignore_ascii_case("show") {
        let cfg = CONFIG.lock().await;
        let mut msg: String<192> = String::new();
        let _ = write!(
            msg,
            "ssid: {}\r\npsk: {}\r\nserver: {}\r\n",
            cfg.ssid.as_str(),
            if cfg.psk.is_empty() {
                "(open)"
            } else {
                "(set)"
            },
            cfg.server.as_str()
        );
        write_text(class, msg.as_str()).await
    } else if cmd.eq_ignore_ascii_case("save") {
        let cfg = CONFIG.lock().await.clone();
        if !cfg.is_ready() {
            return write_text(class, "need wifi and server before save\r\n").await;
        }
        match save_config(flash, &cfg) {
            Ok(()) => {
                JOIN.signal(());
                write_text(class, "saved. joining wifi...\r\n").await
            }
            Err(_) => write_text(class, "flash write failed\r\n").await,
        }
    } else if cmd.eq_ignore_ascii_case("clear") {
        let mut cfg = CONFIG.lock().await;
        *cfg = NetConfig::empty();
        drop(cfg);
        let _ = erase_config(flash);
        JOIN.signal(());
        write_text(class, "cleared\r\n").await
    } else {
        write_text(class, "unknown command. help\r\n").await
    }
}

async fn write_text(
    class: &mut CdcAcmClass<'static, UsbDriver>,
    text: &str,
) -> Result<(), EndpointError> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let end = core::cmp::min(i + 64, bytes.len());
        class.write_packet(&bytes[i..end]).await?;
        i = end;
    }
    Ok(())
}

async fn post_temperature(stack: Stack<'static>, celsius: f32) {
    if WIFI_STATUS.load(Ordering::Relaxed) != ST_UP || !stack.is_config_up() {
        return;
    }
    let server = CONFIG.lock().await.server.clone();
    let Some(target) = parse_server(server.as_str()) else {
        POST_STATUS.store(POST_FAIL, Ordering::Relaxed);
        return;
    };

    let ips = match stack.dns_query(target.host.as_str(), DnsQueryType::A).await {
        Ok(v) if !v.is_empty() => v,
        _ => {
            POST_STATUS.store(POST_FAIL, Ordering::Relaxed);
            return;
        }
    };
    let ip = ips[0];

    let temp = format_temp(celsius);
    let mut body: String<32> = String::new();
    let _ = write!(body, "{{\"celsius\":{}}}", temp.trim_end_matches(" C"));

    let mut req: String<256> = String::new();
    let _ = write!(
        req,
        "POST {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        target.path,
        target.host,
        target.port,
        body.len(),
        body
    );

    let mut rx = [0u8; 512];
    let mut tx = [0u8; 512];
    let mut socket = TcpSocket::new(stack, &mut rx, &mut tx);
    socket.set_timeout(Some(Duration::from_secs(8)));

    let ok = async {
        socket
            .connect(IpEndpoint::new(ip, target.port))
            .await
            .ok()?;
        socket.write_all(req.as_bytes()).await.ok()?;
        let mut head = [0u8; 16];
        let n = socket.read(&mut head).await.ok()?;
        let text = core::str::from_utf8(&head[..n]).ok()?;
        if text.contains(" 2") || text.starts_with("HTTP/1.1 2") || text.starts_with("HTTP/1.0 2") {
            Some(())
        } else {
            None
        }
    }
    .await
    .is_some();

    POST_STATUS.store(if ok { POST_OK } else { POST_FAIL }, Ordering::Relaxed);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut flash = Flash::<_, _, FLASH_SIZE>::new_blocking(p.FLASH);
    {
        let loaded = load_config(&mut flash);
        let mut cfg = CONFIG.lock().await;
        *cfg = loaded;
    }

    let fw = cyw43::aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = cyw43::aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram = cyw43::aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        dma::Channel::new(p.DMA_CH0, Irqs),
    );

    static CYW_STATE: StaticCell<cyw43::State> = StaticCell::new();
    let (net_device, mut control, runner) =
        cyw43::new(CYW_STATE.init(cyw43::State::new()), pwr, spi, fw, nvram).await;
    spawner.spawn(cyw43_task(runner).unwrap());
    control.init(clm.as_ref()).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let mut rng = RoscRng;
    static RESOURCES: StaticCell<StackResources<8>> = StaticCell::new();
    let (stack, net_runner) = embassy_net::new(
        net_device,
        NetStackConfig::dhcpv4(Default::default()),
        RESOURCES.init(StackResources::new()),
        rng.next_u64(),
    );
    spawner.spawn(net_task(net_runner).unwrap());
    spawner.spawn(wifi_task(control, stack).unwrap());

    let driver = Driver::new(p.USB, Irqs);
    let mut usb_config = embassy_usb::Config::new(0xc0de, 0xcaed);
    usb_config.manufacturer = Some("lastertag");
    usb_config.product = Some("Pico 2 W temp");
    usb_config.serial_number = Some("0001");
    usb_config.max_power = 100;
    usb_config.max_packet_size_0 = 64;

    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static MSOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();
    let mut builder = Builder::new(
        driver,
        usb_config,
        CONFIG_DESC.init([0; 256]),
        BOS_DESC.init([0; 256]),
        MSOS_DESC.init([0; 256]),
        CONTROL_BUF.init([0; 64]),
    );
    static CDC_STATE: StaticCell<State> = StaticCell::new();
    let class = CdcAcmClass::new(&mut builder, CDC_STATE.init(State::new()), 64);
    let usb = builder.build();
    spawner.spawn(usb_task(usb).unwrap());
    spawner.spawn(cli_task(class, flash).unwrap());

    let mut adc = Adc::new_blocking(p.ADC, AdcConfig::default());
    let mut temp_sensor = Channel::new_temp_sensor(p.ADC_TEMP_SENSOR);

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 100_000;
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);
    let mut display =
        Oled::new(i2c, 0x3C, OledConfig::sh1106_128x64().with_column_offset(0)).unwrap();

    let title_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let temp_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    let mut seconds: u8 = 0;
    loop {
        let raw = adc.blocking_read(&mut temp_sensor).unwrap();
        let celsius = adc_to_celsius(raw);
        let line = format_temp(celsius);

        display.clear_buffer();
        Text::with_baseline(
            "Pico chip temp",
            Point::new(0, 4),
            title_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();
        Text::with_baseline(line.as_str(), Point::new(8, 24), temp_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline(status_line(), Point::new(0, 52), title_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().ok();

        seconds = seconds.wrapping_add(1);
        if seconds >= 60 {
            seconds = 0;
            post_temperature(stack, celsius).await;
        }

        Timer::after_secs(1).await;
    }
}

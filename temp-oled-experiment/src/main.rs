#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};
use embassy_rp::i2c::{Config as I2cConfig, I2c};
use embassy_time::Timer;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_10X20};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};
use heapless::String;
use panic_halt as _;
use ssd1306::prelude::*;
use ssd1306::{I2CDisplayInterface, Ssd1306};

#[unsafe(link_section = ".bi_entries")]
#[used]
static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 3] = [
    embassy_rp::binary_info::rp_program_name!(c"temp-oled-experiment"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

/// Convert an RP2350 temperature-sensor ADC reading to Celsius.
///
/// Same approximation as the Pico SDK: T = 27 - (V - 0.706) / 0.001721.
fn adc_to_celsius(raw: u16) -> f32 {
    27.0 - (raw as f32 * 3.3 / 4096.0 - 0.706) / 0.001721
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut adc = Adc::new_blocking(p.ADC, AdcConfig::default());
    let mut temp_sensor = Channel::new_temp_sensor(p.ADC_TEMP_SENSOR);

    // I2C0 on GP17 = SCL (physical pin 22) and GP16 = SDA (physical pin 21).
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 400_000;
    let i2c = I2c::new_blocking(p.I2C0, p.PIN_17, p.PIN_16, i2c_config);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let title_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let temp_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    loop {
        let raw = adc.blocking_read(&mut temp_sensor).unwrap();
        let celsius = adc_to_celsius(raw);

        let mut line: String<16> = String::new();
        let tenths = (celsius * 10.0) as i32;
        let whole = tenths / 10;
        let frac = tenths.abs() % 10;
        let _ = write!(line, "{whole}.{frac} C");

        display.clear(BinaryColor::Off).unwrap();
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
        Text::with_baseline(
            "updates every 1s",
            Point::new(0, 52),
            title_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();
        display.flush().unwrap();

        Timer::after_secs(1).await;
    }
}

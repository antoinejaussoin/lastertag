//! RP2350 on-die temperature sensor.

use core::fmt::Write as _;
use embassy_rp::Peri;
use embassy_rp::adc::{Adc, Blocking, Channel, Config};
use embassy_rp::peripherals::{ADC, ADC_TEMP_SENSOR};
use heapless::String;

/// Blocking ADC reader for the Pico 2 W chip temperature sensor.
pub struct ChipTemp<'d> {
    adc: Adc<'d, Blocking>,
    sensor: Channel<'d>,
}

impl<'d> ChipTemp<'d> {
    pub fn new(adc: Peri<'d, ADC>, temp_sensor: Peri<'d, ADC_TEMP_SENSOR>) -> Self {
        Self {
            adc: Adc::new_blocking(adc, Config::default()),
            sensor: Channel::new_temp_sensor(temp_sensor),
        }
    }

    pub fn read_celsius(&mut self) -> f32 {
        let raw = self.adc.blocking_read(&mut self.sensor).unwrap();
        adc_to_celsius(raw)
    }
}

/// Pico SDK approximation: T = 27 - (V - 0.706) / 0.001721.
pub fn adc_to_celsius(raw: u16) -> f32 {
    27.0 - (raw as f32 * 3.3 / 4096.0 - 0.706) / 0.001721
}

/// `"27.4"` with no unit, for JSON.
pub fn format_celsius_value(celsius: f32) -> String<16> {
    let tenths = (celsius * 10.0) as i32;
    let whole = tenths / 10;
    let frac = tenths.abs() % 10;
    let mut line = String::new();
    let _ = write!(line, "{whole}.{frac}");
    line
}

/// `"27.4 C"` for the OLED.
pub fn format_celsius(celsius: f32) -> String<16> {
    let mut line = format_celsius_value(celsius);
    let _ = line.push_str(" C");
    line
}

//! SH1106 OLED drawing for this experiment.

use embassy_rp::i2c::{Blocking, I2c};
use embassy_rp::peripherals::I2C0;
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_10X20};
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};
use oled_i2c::{Oled, OledConfig};

pub type OledI2c = I2c<'static, I2C0, Blocking>;

pub struct Screen {
    display: Oled<OledI2c>,
    title: MonoTextStyle<'static, BinaryColor>,
    value: MonoTextStyle<'static, BinaryColor>,
}

impl Screen {
    pub fn new(i2c: OledI2c) -> Self {
        Self {
            display: Oled::new(i2c, 0x3C, OledConfig::sh1106_128x64().with_column_offset(0))
                .unwrap(),
            title: MonoTextStyleBuilder::new()
                .font(&FONT_6X10)
                .text_color(BinaryColor::On)
                .build(),
            value: MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .text_color(BinaryColor::On)
                .build(),
        }
    }

    pub fn show(&mut self, main_line: &str, status: &str) {
        self.display.clear_buffer();
        Text::with_baseline("IR capture", Point::new(0, 4), self.title, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
        Text::with_baseline(main_line, Point::new(8, 24), self.value, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
        Text::with_baseline(status, Point::new(0, 52), self.title, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
        self.display.flush().ok();
    }
}

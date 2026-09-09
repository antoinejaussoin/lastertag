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

    pub fn show(&mut self, main_line: &str, status: &str, presses: u32) {
        self.display.clear_buffer();
        Text::with_baseline("IR sender", Point::new(0, 4), self.title, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
        let mut count_buf = [0u8; 10];
        let count = u32_str(presses, &mut count_buf);
        let count_x = 128 - 6 * count.len() as i32;
        Text::with_baseline(count, Point::new(count_x, 4), self.title, Baseline::Top)
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

fn u32_str(n: u32, buf: &mut [u8; 10]) -> &str {
    if n == 0 {
        buf[0] = b'0';
        return core::str::from_utf8(&buf[..1]).unwrap();
    }
    let mut i = buf.len();
    let mut rest = n;
    while rest > 0 {
        i -= 1;
        buf[i] = b'0' + (rest % 10) as u8;
        rest /= 10;
    }
    core::str::from_utf8(&buf[i..]).unwrap()
}

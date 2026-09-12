//! Pack a 1600×1200 PNG into the Inky / Tesserae Spectra 6 `.bin` wire format.
//!
//! Headerless, exactly `1600 * 1200 / 2` bytes, scanline order, two pixels
//! per byte: high nibble = even column, low nibble = odd column.
//!
//! Palette (nibble values):
//! `0` black, `1` white, `2` yellow, `3` red, `5` blue, `6` green.

use anyhow::{bail, Result};
use image::{Rgba, RgbaImage};

pub const PANEL_WIDTH: u32 = 1600;
pub const PANEL_HEIGHT: u32 = 1200;
pub const PANEL_BYTES: usize = (PANEL_WIDTH as usize * PANEL_HEIGHT as usize) / 2;

/// Spectra 6 indices and the sRGB primaries the Inky firmware expects.
pub const SPECTRA6: [(u8, [u8; 3]); 6] = [
    (0, [0x00, 0x00, 0x00]),
    (1, [0xff, 0xff, 0xff]),
    (2, [0xff, 0xff, 0x00]),
    (3, [0xff, 0x00, 0x00]),
    (5, [0x00, 0x00, 0xff]),
    (6, [0x00, 0xff, 0x00]),
];

pub fn pack_png_to_spectra6(png: &[u8]) -> Result<Vec<u8>> {
    let img = image::load_from_memory(png)?.to_rgba8();
    pack_rgba(&img)
}

pub fn pack_rgba(img: &RgbaImage) -> Result<Vec<u8>> {
    if img.width() != PANEL_WIDTH || img.height() != PANEL_HEIGHT {
        bail!(
            "frame must be {PANEL_WIDTH}x{PANEL_HEIGHT}, got {}x{}",
            img.width(),
            img.height()
        );
    }
    let indexed = dither_floyd_steinberg(img);
    Ok(pack_nibbles(&indexed))
}

fn nearest_index(r: i32, g: i32, b: i32) -> (u8, [u8; 3]) {
    let mut best = SPECTRA6[0];
    let mut best_d = i32::MAX;
    for &(idx, rgb) in &SPECTRA6 {
        let dr = r - rgb[0] as i32;
        let dg = g - rgb[1] as i32;
        let db = b - rgb[2] as i32;
        let d = dr * dr + dg * dg + db * db;
        if d < best_d {
            best_d = d;
            best = (idx, rgb);
        }
    }
    best
}

/// Floyd–Steinberg dither to the six panel colours.
fn dither_floyd_steinberg(img: &RgbaImage) -> Vec<u8> {
    let w = img.width() as usize;
    let h = img.height() as usize;
    let mut work: Vec<[i32; 3]> = img
        .pixels()
        .map(|px| {
            let Rgba([r, g, b, _]) = *px;
            [r as i32, g as i32, b as i32]
        })
        .collect();
    let mut out = vec![0u8; w * h];

    for y in 0..h {
        let left_to_right = y % 2 == 0;
        let xs: Box<dyn Iterator<Item = usize>> = if left_to_right {
            Box::new(0..w)
        } else {
            Box::new((0..w).rev())
        };
        for x in xs {
            let i = y * w + x;
            let [r, g, b] = work[i];
            let (idx, rgb) = nearest_index(r, g, b);
            out[i] = idx;
            let err = [
                r - rgb[0] as i32,
                g - rgb[1] as i32,
                b - rgb[2] as i32,
            ];
            let mut spread = |wx: isize, wy: isize, num: i32, den: i32| {
                let nx = x as isize + wx;
                let ny = y as isize + wy;
                if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                    return;
                }
                let j = ny as usize * w + nx as usize;
                for c in 0..3 {
                    work[j][c] += err[c] * num / den;
                }
            };
            if left_to_right {
                spread(1, 0, 7, 16);
                spread(-1, 1, 3, 16);
                spread(0, 1, 5, 16);
                spread(1, 1, 1, 16);
            } else {
                spread(-1, 0, 7, 16);
                spread(1, 1, 3, 16);
                spread(0, 1, 5, 16);
                spread(-1, 1, 1, 16);
            }
        }
    }
    out
}

fn pack_nibbles(indexed: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(PANEL_BYTES);
    for pair in indexed.chunks_exact(2) {
        out.push((pair[0] << 4) | (pair[1] & 0x0f));
    }
    out
}

/// Debug PNG: expand packed nibbles back to the six RGB primaries.
pub fn unpack_preview_png(bin: &[u8]) -> Result<Vec<u8>> {
    if bin.len() != PANEL_BYTES {
        bail!("packed frame must be {PANEL_BYTES} bytes, got {}", bin.len());
    }
    let mut img = RgbaImage::new(PANEL_WIDTH, PANEL_HEIGHT);
    let mut i = 0;
    for y in 0..PANEL_HEIGHT {
        for x in (0..PANEL_WIDTH).step_by(2) {
            let byte = bin[i];
            i += 1;
            let hi = byte >> 4;
            let lo = byte & 0x0f;
            img.put_pixel(x, y, rgba_for(hi));
            img.put_pixel(x + 1, y, rgba_for(lo));
        }
    }
    let mut png = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png),
        image::ImageFormat::Png,
    )?;
    Ok(png)
}

fn rgba_for(idx: u8) -> Rgba<u8> {
    let rgb = SPECTRA6
        .iter()
        .find(|(i, _)| *i == idx)
        .map(|(_, rgb)| *rgb)
        .unwrap_or([0, 0, 0]);
    Rgba([rgb[0], rgb[1], rgb[2], 255])
}

#[allow(dead_code)]
pub fn solid_png(r: u8, g: u8, b: u8) -> Result<Vec<u8>> {
    let img = RgbaImage::from_pixel(PANEL_WIDTH, PANEL_HEIGHT, Rgba([r, g, b, 255]));
    let mut png = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png),
        image::ImageFormat::Png,
    )?;
    Ok(png)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_size_is_960000() {
        let png = solid_png(255, 255, 255).unwrap();
        let bin = pack_png_to_spectra6(&png).unwrap();
        assert_eq!(bin.len(), PANEL_BYTES);
        assert!(bin.iter().all(|&b| b == 0x11), "white should pack as 0x11");
    }

    #[test]
    fn black_and_red_nibbles() {
        let mut img = RgbaImage::from_pixel(PANEL_WIDTH, PANEL_HEIGHT, Rgba([0, 0, 0, 255]));
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        img.put_pixel(1, 0, Rgba([0, 0, 0, 255]));
        let bin = pack_rgba(&img).unwrap();
        assert_eq!(bin[0] >> 4, 3, "even column red");
        assert_eq!(bin[0] & 0x0f, 0, "odd column black");
    }

    #[test]
    fn wrong_size_is_rejected() {
        let img = RgbaImage::new(10, 10);
        assert!(pack_rgba(&img).is_err());
    }

}

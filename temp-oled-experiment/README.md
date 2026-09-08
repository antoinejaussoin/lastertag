# Temperature-on-OLED experiment

A standalone first hardware test, separate from the laser-tag firmware. It
reads the temperature sensor **inside** the Raspberry Pi Pico 2 and shows it
on the 0.96" I²C OLED.

You do not need to know Rust or electronics already. Follow the steps in order.

## What you need

- Raspberry Pi Pico 2 (this experiment targets the RP2350 chip)
- 0.96" four-pin I²C OLED (often sold as SSD1306; many yellow/blue 128×64
  modules are actually SH1106)
- the full-size breadboard and jumper wires from the shopping list
- a **data** USB cable (charge-only cables will not work)

The PiCowbell and LiPo are not used for this test. USB powers the Pico.

## Safety

- Unplug USB while you insert parts.
- Power the OLED from **3.3 V only**. Do not use `VBUS` (physical pin 40, the
  5 V USB pin).

## How to plug it together

Open the drawing in this folder:

- [`wiring.svg`](wiring.svg)

Useful references:

- [Interactive Pico pinout](https://pico.pinout.xyz/) — hover GP16, GP17, 3V3, GND
- [Pi Hut 0.96" OLED](https://thepihut.com/products/0-96-oled-display-module-128x64) — this module’s pins and I²C address `0x3C`
- [Raspberry Pi Pico getting started](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html) — BOOTSEL / UF2 upload

### Assembly

1. Unplug USB. Sit the Pico 2 **across the centre gap** of the breadboard,
   like a bridge. The USB socket should hang off the top so the cable still
   fits. Count pins from that USB end.
2. Put the OLED in four unused columns, away from the Pico, so each OLED pin
   has its own column.
3. Four jumper wires. Match the **printed names on the OLED**, not a photo of
   a different module:

   | OLED pin | Pico pin | Physical pin (USB at the top) |
   |---|---|---:|
   | `VCC` | `3V3` | 36 (fifth pin down the **right** side) |
   | `GND` | `GND` | 38 (third pin down the **right** side) |
   | `SDA` | `GP16` | 21 (bottom pin on the **right** side) |
   | `SCL` | `GP17` | 22 (second pin from the bottom on the **right** side) |

4. On a breadboard, every hole in the same numbered column (on one side of
   the gap) is already connected. Plug the jumper into the same column as the
   Pico pin, then into the OLED pin’s column.

You can ignore the long coloured power rails for this test.

## What the firmware does

The Pico 2 has a small temperature sensor on the RP2350 silicon. The program
reads that sensor about once a second and draws the value on the OLED.

That number is the **chip** temperature. It is usually a few degrees above the
room, and it rises while the chip runs. It is not a weather reading.

## Install the software tools (once)

You will compile a program on your computer, then copy a small file onto the
Pico. The Pico is a microcontroller: it does not run macOS or Linux.

### 1. Install Rust

On macOS, in Terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts (the default options are fine). Then close Terminal and open
a new one, or run:

```bash
source "$HOME/.cargo/env"
```

Check:

```bash
rustc --version
cargo --version
```

### 2. Add the Pico 2 CPU target

This crate’s `rust-toolchain.toml` asks `rustup` to install
`thumbv8m.main-none-eabihf` when you enter the folder. You can also install it
yourself:

```bash
rustup target add thumbv8m.main-none-eabihf
```

That target is the Pico 2’s ARM Cortex-M33 CPU.

## Build and upload

Put the Pico 2 in bootloader mode first:

1. Unplug the USB cable.
2. Hold the **BOOTSEL** button on the Pico (the small button next to the USB
   socket).
3. Plug the USB cable in while still holding BOOTSEL.
4. Release BOOTSEL.
5. Finder should show a drive called **RP2350**.

If nothing appears, the cable is probably charge-only. Try another cable.

Then, from the `lastertag` repository root:

```bash
cd temp-oled-experiment
make
```

That compiles the firmware, converts it to a `.uf2`, and copies it onto the
Pico. The `RP2350` drive disappears; that is normal. The program starts
immediately.

The first build downloads crates and can take several minutes. After that,
`make` is the only command you need when the board is already in BOOTSEL.

If `make` builds the UF2 but cannot copy it, drag
`temp-oled-experiment.uf2` onto the **RP2350** drive in Finder.

If you only want the files without copying them onto the board:

```bash
make build   # ELF only
make uf2     # ELF plus temp-oled-experiment.uf2
```

## What you should see

Text updating about once a second, similar to:

```text
Pico chip temp
  27.4 C
updates every 1s
```

On a dual-colour 0.96" panel the top band is yellow and the rest is blue.
The title sits in the yellow band; the temperature number is blue. That is
the glass, not a wiring fault.

A value between about 20 °C and 45 °C at a desk is typical.

SH1106 panels have 132 columns of RAM and 128 visible. This firmware starts
the write window at column 0 so the left edge is not leftover RAM. If a 2-pixel
bar appears on the **right** instead, change `with_column_offset(0)` in
`src/main.rs` to `with_column_offset(2)` and flash again.

## If the title is readable but the rest is noise

The firmware talks to the panel as an **SH1106** (132-column RAM). Cheap
yellow/blue 0.96" modules are often SH1106 even when the listing says
SSD1306. Driving SH1106 as SSD1306 commonly leaves the first rows readable
and fills the blue area with random pixels.

Reflash this folder with BOOTSEL + `make`. If the temperature is still
garbage, in `src/main.rs` change `OledConfig::sh1106_128x64()` to
`OledConfig::ssd1306_128x64()` and flash again.

## If the screen stays black

- USB is plugged into the Pico (the board is powered from that cable).
- Every OLED label matches the table. Swapping SDA/SCL is the most common
  mistake.
- `VCC` is on **3V3** (pin 36), not **VBUS** (pin 40).
- You counted pins from the USB end. `GP16`/`GP17` are on the **right** side,
  at the end opposite the USB plug.
- You actually copied a UF2 (the `RP2350` drive vanished after the copy).
- Some modules use I²C address `0x3D` instead of `0x3C`. In `src/main.rs`,
  change the `0x3C` passed to `Oled::new` to `0x3D`.
- The plastic film on a new OLED can make it look dim; peel it off.

## Changing the program

Edit `src/main.rs`, save, put the Pico back in BOOTSEL, then `make` again.

This folder is only a sandbox. Game firmware will live later under
`source/firmware/`.

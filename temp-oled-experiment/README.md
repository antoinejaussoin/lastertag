# Temperature-on-OLED experiment

A standalone first hardware test, separate from the laser-tag firmware. It
reads the temperature sensor **inside** the Raspberry Pi Pico chip and shows it
on the SSD1306 OLED.

You do not need to know Rust or electronics already. Follow the steps in order.

## What you need

- Raspberry Pi Pico WH (headers already fitted)
- 0.96" SSD1306 OLED (the Pi Hut four-pin module)
- the full-size breadboard and jumper wires from the shopping list
- a **data** Micro-USB cable (charge-only cables will not work)

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

1. Unplug USB. Sit the Pico WH **across the centre gap** of the breadboard,
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

The Pico has a small temperature sensor on the RP2040 silicon. The program
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

### 2. Add the Pico CPU target

This crate’s `rust-toolchain.toml` asks `rustup` to install
`thumbv6m-none-eabi` when you enter the folder. You can also install it
yourself:

```bash
rustup target add thumbv6m-none-eabi
```

That target is the Pico’s ARM Cortex-M0+ CPU. Your Mac cannot run this binary
directly; only the Pico can.

### 3. Install the UF2 helper

```bash
cargo install elf2uf2-rs --locked
```

UF2 is the file format the Pico’s built-in bootloader understands. You copy a
`.uf2` file onto a fake USB disk called `RPI-RP2`, and the Pico writes it to
flash and reboots.

## Build and upload

In Terminal, from the `lastertag` repository root:

```bash
cd temp-oled-experiment
make
```

That runs a release build. The compiled firmware is:

`target/thumbv6m-none-eabi/release/temp-oled-experiment`

The first build downloads crates and can take several minutes. `make uf2` also
writes a `.uf2` you can copy onto the Pico. `make help` lists the other
targets.

### Put the Pico in bootloader mode

The chip must look like a USB drive named **RPI-RP2**:

1. Unplug the USB cable.
2. Hold the **BOOTSEL** button on the Pico (the small button next to the USB
   socket).
3. Plug the USB cable in while still holding BOOTSEL.
4. Release BOOTSEL.
5. Finder should show a drive called `RPI-RP2`.

If nothing appears, the cable is probably charge-only. Try another cable.

### Option A — one command (easiest once the drive is visible)

With `RPI-RP2` already mounted:

```bash
make flash
```

This builds, converts the program to UF2, and copies it to the Pico. The drive
will disappear; that is normal. The program starts immediately.

If Finder shows **RP2350** instead of **RPI-RP2**, that board is a Pico 2, not
a Pico WH. This firmware will not run on it.

### Option B — drag the UF2 in Finder

```bash
make uf2
```

That writes `temp-oled-experiment.uf2` in this folder. Put the Pico in
bootloader mode, then copy that file onto `RPI-RP2`. The drive ejects itself
and the screen should show a temperature.

## What you should see

White text on a black OLED, updating about once a second, similar to:

```text
Pico chip temp
  27.4 C
updates every 1s
```

A value between about 20 °C and 45 °C at a desk is typical.

## If the screen stays black

- USB is plugged into the Pico (the board is powered from that cable).
- Every OLED label matches the table. Swapping SDA/SCL is the most common
  mistake.
- `VCC` is on **3V3** (pin 36), not **VBUS** (pin 40).
- You counted pins from the USB end. `GP16`/`GP17` are on the **right** side,
  at the end opposite the USB plug.
- You actually copied a UF2 (the `RPI-RP2` drive vanished after the copy).
- Some SSD1306 modules use I²C address `0x3D` instead of `0x3C`. In
  `src/main.rs`, replace `I2CDisplayInterface::new(i2c)` with
  `I2CDisplayInterface::new_alternate_address(i2c)`.
- The plastic film on a new OLED can make it look dim; peel it off.

## Changing the program

Edit `src/main.rs`, save, then build and upload again. The Pico only runs the
last UF2 you copied; there is no “run” button on the board.

This folder is only a sandbox. Game firmware will live later under
`source/firmware/`.

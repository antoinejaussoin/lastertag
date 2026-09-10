# IR-sender experiment

A standalone hardware test, separate from the laser-tag firmware. It sits on
the **second** Pico 2 W. Press a button and it blinks the infrared LED with a
NEC remote code. The [`ir-capture`](../ir-capture/) board should show `42:01`.

Leave ir-capture assembled. Do not unplug that breadboard.

You do not need to know Rust or electronics already. Follow the steps in order.

## What you need

- the **second** Raspberry Pi Pico 2 W (keep the first one running ir-capture)
- 0.96" four-pin I²C OLED (the second module from the shopping list)
- Vishay TSAL6200 940 nm IR LED (Rapid)
- one Diotec **BC337-40** transistor
- two **15 Ω, 1 W** resistors (brown-green-black)
- one **220 Ω** resistor (red-red-brown) — GPIO to the transistor **base** only
- one **10 kΩ** resistor (brown-black-orange) — base to GND
- one tactile push button (Diptronics DTS-644K)
- the second full-size breadboard and jumper wires
- a **data** USB cable (charge-only cables will not work)

The PiCowbell and LiPo are **not** used here. USB powers the Pico.

## Safety

- Unplug USB while you insert parts.
- Power the OLED from **3.3 V only**. Do not use `VBUS` (physical pin 40, the
  5 V USB pin).
- The two 15 Ω resistors feed the LED from **3.3 V** through the transistor.
  Never put a 15 Ω resistor on GP18. The 220 Ω is only between GP18 and the
  base.

## How to plug it together

Two drawings in this folder:

- [`connections.svg`](connections.svg) — three diagrams (OLED, button, TSAL), Pico on each, no breadboard
- [`wiring.svg`](wiring.svg) — the same circuit, hole by hole on the breadboard

Useful references:

- [Interactive Pico pinout](https://pico.pinout.xyz/) — choose Pico 2 W; hover GP16, GP17, GP18, GP19, 3V3, GND
- [Pi Hut 0.96" OLED](https://thepihut.com/products/0-96-oled-display-module-128x64)
- [Vishay TSAL6200](https://www.rapidonline.com/vishay-tsal6200-5mm-940nm-ir-transmitter-diode-49-4513) — long lead is the anode
- [Raspberry Pi Pico getting started](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html) — BOOTSEL / UF2 upload

### TSAL6200 pinout

Looks like a clear or smoke-grey 5 mm LED:

| Leg | Name | Goes to |
|---|---|---|
| long | anode (`A`) | two 15 Ω from `+3V3` |
| short, next to the flat rim | cathode (`K`) | BC337 collector |

### BC337-40 pinout

Small black TO-92, one flat face, three legs. For the Diotec part, with the
**flat toward you** and the legs down, the order is **C-B-E** (left to right).
On this breadboard, point the flat at the **trench** and use
`c26` = C, `c25` = B, `c24` = E. All IR parts stay in rows `a`–`d`.

### Assembly

Unplug USB. Hold the breadboard so **printed column 64 is on the left** (Pico /
USB) and **column 1 is on the right**. The Pico sits **lower**: pins in **row
`e`** (above the trench) and **row `j`** (below the trench). On this board the
lower letters run `j` `i` `h` `g` `f` from the trench down. Top rails are `+`
then `−`. After row `f`, the bottom rails are `+` then `−` at the very bottom.

Follow [`wiring.svg`](wiring.svg) hole for hole. Yellow holes on the drawing
already have something in them. If your last printed number is 63, not 64,
shift every hole one column toward the Pico.

1. Sit the Pico 2 W **across the centre trench**, like a DIP chip. USB hangs
   off the left at column 64 so the cable still fits. 20 pins go in **row e**
   (columns 64–45) and 20 pins go in **row j**. The antenna is at column 45.

   With the chip facing you and USB on the left, the **top** row (e) is pins
   40–21 and the **bottom** row (j) is pins 1–20. GP16–GP19 are on the top.

   | Pico pin | Hole | Name |
   |---|---|---|
   | 40 | `e64` | VBUS — leave empty |
   | 38 | `e62` | GND |
   | 36 | `e60` | 3V3 |
   | 25 | `e49` | GP19 (button) |
   | 24 | `e48` | GP18 (IR LED) |
   | 23 | `e47` | GND — skip this column |
   | 22 | `e46` | GP17 (SCL) |
   | 21 | `e45` | GP16 (SDA) |
   | 1 | `j64` | GP0 |

2. Power the long rails from the Pico, then join top and bottom:

   | Colour | From | To |
   |---|---|---|
   | red | `a60` | top `+` rail |
   | black | `a62` | top `−` rail |
   | red | bottom `+` column 20 | top `+` column 20 |
   | black | bottom `−` column 19 | top `−` column 19 |

3. OLED pins in **`a42` `a41` `a40` `a39`**, from the Pico toward column 1.
   Typical Pi Hut order is `VCC`, `GND`, `SCL`, `SDA`. If your module prints a
   different order, keep those four holes and move the wires to the printed
   names.

   | OLED name | Hole | Jumper |
   |---|---|---|
   | `VCC` | `a42` | top `+` rail |
   | `GND` | `a41` | top `−` rail |
   | `SCL` | `c40` | `c46` (GP17) |
   | `SDA` | `d39` | `d45` (GP16) |

4. IR driver on the **top** half (rows `a`–`d`), **to the right of the OLED**.
   Q1 needs three consecutive holes of its own. Unplug USB first. Pull the old
   IR parts if they were in columns 37–32 — those holes stacked the LED on
   top of Q1, so the middle leg (B) often never sat in its strip.

   | Part | Holes |
   |---|---|
   | BC337-40, flat toward the trench | `c26` C, `c25` B, `c24` E |
   | red jumper +3V3 → first 15 Ω | top `+` column 37 → `b37` |
   | 15 Ω | `b37` – `b32` |
   | 15 Ω | `b32` – `b28` |
   | TSAL6200 anode / cathode | `c28` / `c26` (cathode shares C) |
   | orange jumper GP18 → 220 Ω | `b48` → `d21` |
   | 220 Ω (base) | `d21` – `d25` |
   | 10 kΩ base pull-down | `a25` → top `−` rail |
   | black jumper emitter → GND | `a24` → top `−` rail |

   Long LED lead in `c28`. Short lead and flat rim in `c26`.

   A meter on **B** (`c25`) while GP18 is high must read about **0.7 V**. If
   that hole is still **3.3 V**, the middle transistor leg is not in `c25`.

   Bands: 15 Ω is **brown-green-black**. 220 Ω is **red-red-brown**. 10 kΩ is
   **brown-black-orange**. A 15 Ω on GP18 can damage the Pico.

5. Button across the trench so pressing it joins `e` to `j`. A four-leg switch
   has two permanently connected legs on each side. Keep it off column 32 —
   that strip is the 15 Ω midpoint.

   | Part | Holes |
   |---|---|
   | button legs | `e31` `e29` `j31` `j29` |
   | purple jumper GP19 | `d49` → `d31` |
   | black jumper to GND | `j31` → bottom `−` rail |

Rows `a`–`e` in one numbered column are already joined. Rows `j`–`f` in that
column are a second, separate strip. That is why a jumper in `b48` is already
connected to Pico GP18 in `e48`.

## What the firmware does

GP18 is a GPIO into the BC337 base. Marks are a 38 kHz square wave on that
pin; spaces are the pin low (transistor off). The LED itself runs from 3.3 V
at about 58 mA. On each button press the program sends three **NEC** frames:
address `0x42`, command `0x01`. ir-capture already understands that encoding,
so its OLED should show `42:01` and its HTTP POST looks like:

```json
{"proto":"nec","addr":66,"cmd":1,"rep":false}
```

A held button does not repeat. Release and press again for another burst.

Boot starts at `ready`. There is no Wi-Fi on this board. A USB serial CLI
(same idea as capture’s Wi-Fi setup) can turn on a **debug** mode for a meter:

```text
debug on
save
```

Then the button toggles **`always on`** (LED DC, ~58 mA, TSOP ignores it) and
**`every 1s`** (NEC `42:01` once a second). `debug off` then `save` returns to
one NEC per click. `show` / `clear` work like capture. After `save`, the flag
survives reboot.

Keep phones **away from the capture TSOP**. An iPhone’s Face ID / LiDAR
illuminator is also 940 nm; that chip will report short garbled `raw` bursts
that have nothing to do with this LED.

## Install the software tools (once)

Same tools as [`ir-capture`](../ir-capture/README.md). If Rust is already
installed, skip to **Build and upload**.

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

### 2. Add the Pico 2 W CPU target

This crate’s `rust-toolchain.toml` asks `rustup` to install
`thumbv8m.main-none-eabihf` when you enter the folder. You can also install it
yourself:

```bash
rustup target add thumbv8m.main-none-eabihf
```

That target is the Pico 2 W’s ARM Cortex-M33 CPU.

## Build and upload

Put **this** Pico 2 W in bootloader mode first (the sender board, not the
capture board):

1. Unplug the USB cable.
2. Hold the **BOOTSEL** button on the Pico (the small button next to the USB
   socket).
3. Plug the USB cable in while still holding BOOTSEL.
4. Release BOOTSEL.
5. Finder should show a drive called **RP2350**.

If nothing appears, the cable is probably charge-only. Try another cable.

Then, from the `lastertag` repository root:

```bash
cd projects/ir-sender
make
```

That compiles the firmware, converts it to a `.uf2`, and copies it onto the
Pico. The `RP2350` drive disappears; that is normal. The program starts
immediately.

The first build downloads crates and can take several minutes. After that,
`make` is the only command you need when the board is already in BOOTSEL.

If `make` builds the UF2 but cannot copy it, drag `ir-sender.uf2` onto the
**RP2350** drive in Finder.

If you only want the files without copying them onto the board:

```bash
make build   # ELF only
make uf2     # ELF plus ir-sender.uf2
```

## What you should see

Right after flash, the **sender** OLED shows:

```text
IR sender
  ready
press btn
```

Press the button. The line changes to `sent 42:01`.

On the **capture** Pico, pointed at this LED from a short distance:

```text
IR capture    H
  42:01
wifi ok  post ok
```

(`wifi ok  post ok` only if that Pico is already joined to Wi-Fi.)

On a dual-colour 0.96" panel the top band is yellow and the rest is blue.
The title sits in the yellow band. That is the glass, not a wiring fault.

## USB serial debug setup

Debug mode is **off** by default. Leave it off for the real test: one button
click sends one NEC burst and stops.

Flash the sender, wait until the OLED shows `ready`, then open a serial
terminal. On macOS:

```bash
ls /dev/cu.usbmodem*
screen /dev/cu.usbmodem* 115200
```

Capture and sender both appear as `usbmodem` devices. Try each until the
banner says `Pico 2 W IR sender`. Type `help`. Leave `screen` with `Ctrl-A`
then `K`, then `Y`.

```text
debug on
save
```

`save` writes flash and applies now. The OLED switches to `always on` (LED
DC, ~58 mA — the capture TSOP ignores DC). Press the button to toggle
`every 1s` (NEC `42:01` once a second). Press again to go back to DC.

```text
debug off
save
```

That returns to one NEC per click. `show` prints the RAM copy. `clear` wipes
the saved flag. After `save`, the setting survives reboot.

Range is tens of centimetres, not metres. Point the LED at the TSOP **lens**.
The LED does not light visibly; infrared is outside human vision.

## If the title is readable but the rest is noise

The firmware talks to the panel as an **SH1106** (132-column RAM). Cheap
yellow/blue 0.96" modules are often SH1106 even when the listing says
SSD1306. Driving SH1106 as SSD1306 commonly leaves the first rows readable
and fills the blue area with random pixels.

Reflash this folder with BOOTSEL + `make`. If the value is still garbage, in
`src/display.rs` change `OledConfig::sh1106_128x64()` to
`OledConfig::ssd1306_128x64()` and flash again.

SH1106 panels have 132 columns of RAM and 128 visible. This firmware starts
the write window at column 0 so the left edge is not leftover RAM. If a 2-pixel
bar appears on the **right** instead, change `with_column_offset(0)` in
`src/display.rs` to `with_column_offset(2)` and flash again.

## If the screen stays black

- USB is plugged into the Pico (the board is powered from that cable).
- Every OLED label matches the table. Swapping SDA/SCL is the most common
  mistake.
- `VCC` is on **3V3** (pin 36), not **VBUS** (pin 40).
- You counted pins from the USB end. `GP16`/`GP17` are on the **top** row,
  at the end opposite the USB plug (`e45` / `e46`).
- You actually copied a UF2 (the `RP2350` drive vanished after the copy).
- Some modules use I²C address `0x3D` instead of `0x3C`. In `src/display.rs`,
  change the `0x3C` passed to `Oled::new` to `0x3D`.
- The plastic film on a new OLED can make it look dim; peel it off.

## If capture shows nothing, or raw times instead of `42:01`

- After flash, OLED must say `ready` (otherwise this UF2 is not on the board).
  Point the LED at the capture TSOP and press: you should get `42:01`. For a
  meter, USB serial `debug on` then `save` — OLED `always on` is DC (capture
  will not show `L`). Button then toggles `every 1s`. Still nothing: Q1 flat
  toward the trench (`c26` C, `c25` B, `c24` E), long LED lead in `c28`, orange
  jumper from `b48` (GP18) not column 47, 15 Ω only in the 3.3 V LED path.
  If GP18 is 3.3 V but **B (`c25`) is also 3.3 V**, the middle Q1 leg is not
  in `c25` — a real base clamps at ~0.7 V.
- Do **not** hold an iPhone (or any Face ID / LiDAR phone) near the capture
  TSOP. That illuminator is 940 nm and shows up as short `raw` junk. A TV
  remote is the right “capture still works” check.
- Point the LED at the TSOP lens on the other breadboard, a few centimetres
  away. Do not aim at the Pico or the table. A phone camera is a wiring test,
  not a protocol test: 38 kHz bursts are too short and too dim to trust.
- The orange jumper must be column **48 on the top** (`b48`), the same strip
  as Pico `e48` / GP18. Column **47** on that row is GND — an easy miss.
- Long LED lead is in `c28` (anode). Cathode must sit on the collector (`c26`),
  not on GND.
- The 220 Ω is `d21`–`d25` (base). The 15 Ω pair is `b37`–`b32`–`b28`.
- You flashed **this** folder (`projects/ir-sender`) onto the sender Pico, and
  ir-capture is still running on the other Pico.
- Capture’s title shows **`H`** at rest. If it shows **`L`**, that board’s
  receiver wiring is wrong; see the ir-capture README.

## Changing the program

Edit `src/main.rs` or `src/ir.rs`, save, put the Pico back in BOOTSEL, then
`make` again.

This folder is only a sandbox. Game firmware will live later under
`projects/lasertag/source/firmware/`. The player board uses this same
transistor driver on GP0.

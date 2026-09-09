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
- one **220 Ω** resistor (from the pack of 100)
- one tactile push button (Diptronics DTS-644K)
- the second full-size breadboard and jumper wires
- a **data** USB cable (charge-only cables will not work)

The PiCowbell, LiPo, transistor, and 15 Ω resistors are **not** used here. USB
powers the Pico.

## Safety

- Unplug USB while you insert parts.
- Power the OLED from **3.3 V only**. Do not use `VBUS` (physical pin 40, the
  5 V USB pin).
- The IR LED goes through **220 Ω** on a GPIO. Never use a 15 Ω resistor on
  this pin: that would pull far too much current from the Pico. The later
  player gun uses a transistor driver; this sandbox does not.

## How to plug it together

Two drawings in this folder:

- [`connections.svg`](connections.svg) — what joins to what, no breadboard
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
| long | anode (`A`) | 220 Ω (the other side of the resistor is Pico `GP18`) |
| short, next to the flat rim | cathode (`K`) | Pico `GND` |

### Assembly

Unplug USB. Hold the breadboard so **printed column 1 is on the left**. Follow
[`wiring.svg`](wiring.svg) hole for hole. Yellow holes on the drawing already
have something in them.

1. Sit the Pico 2 W **across the centre trench**, like a DIP chip. USB hangs
   off the left so the cable still fits. 20 pins go in **row e** (columns 1–20)
   and 20 pins go in **row f**. The antenna is at column 20.

   With the chip facing you and USB on the left, the **top** row (e) is pins
   40–21 and the **bottom** row (f) is pins 1–20. GP16–GP19 are on the top.

   | Pico pin | Hole | Name |
   |---|---|---|
   | 40 | `e1` | VBUS — leave empty |
   | 38 | `e3` | GND |
   | 36 | `e5` | 3V3 |
   | 25 | `e16` | GP19 (button) |
   | 24 | `e17` | GP18 (IR LED) |
   | 22 | `e19` | GP17 (SCL) |
   | 21 | `e20` | GP16 (SDA) |
   | 1 | `f1` | GP0 |

2. Power the long rails from the Pico, then join top and bottom:

   | Colour | From | To |
   |---|---|---|
   | red | `a5` | top `+3V3` rail |
   | black | `a3` | top `GND` rail |
   | red | bottom `+` column 35 | top `+` column 35 |
   | black | bottom `GND` column 34 | top `GND` column 34 |

3. OLED pins in **`a23` `a24` `a25` `a26`**. Typical Pi Hut order is
   `VCC`, `GND`, `SCL`, `SDA`. If your module prints a different order, keep
   those four holes and move the wires to the printed names.

   | OLED name | Hole | Jumper |
   |---|---|---|
   | `VCC` | `a23` | top `+3V3` rail |
   | `GND` | `a24` | top `GND` rail |
   | `SCL` | `c25` | `c19` (GP17) |
   | `SDA` | `d26` | `d20` (GP16) |

4. IR LED and 220 Ω. The resistor has no polarity. The LED does.

   The 220 Ω from the Royal Ohm pack is **red-red-brown** (then usually gold).
   That is not the 10 kΩ pack (**brown-black-orange**) and not a 15 Ω
   (**brown-green-black**). A 10 kΩ here leaves the LED dark. A 15 Ω here can
   damage GP18.

   | Part | Holes |
   |---|---|
   | orange jumper GP18 → resistor | `b17` → `b28` |
   | 220 Ω | `a28` – `a32` |
   | TSAL6200 anode / cathode | `c32` / `c33` |
   | black jumper LED cathode → GND | `a33` → top `GND` rail |

   Long LED lead in `c32`. Short lead and flat rim in `c33`.

5. Button across the trench so pressing it joins the two sides. A four-leg
   switch has two permanently connected legs on each side.

   | Part | Holes |
   |---|---|
   | button legs | `e34` `e36` `f34` `f36` |
   | purple jumper GP19 | `d16` → `d34` |
   | black jumper to GND | `j34` → bottom `GND` rail |

Rows `a`–`e` in one numbered column are already joined. Rows `f`–`j` in that
column are a second, separate strip. That is why a jumper in `b17` is already
connected to Pico GP18 in `e17`.

## What the firmware does

GP18 is a GPIO. Marks are a 38 kHz square wave; spaces are the pin low. On
each button press the program sends three **NEC** frames: address `0x42`,
command `0x01`. ir-capture already understands that encoding, so its OLED
should show `42:01` and its HTTP POST looks like:

```json
{"proto":"nec","addr":66,"cmd":1,"rep":false}
```

A held button does not repeat. Release and press again for another burst.

Right after you flash, the sender OLED shows `aim TSOP` and the LED blinks a
38 kHz carrier about once a second. **Do not use a phone camera** — 940 nm at
this current is usually invisible on an iPhone. Point the LED at the capture
TSOP from a few centimetres. Capture should flip to `L` / `stuck L` in time
with `carrier ON`. Then it sends NEC once and sits at `ready`. After a press
it shows `sent 42:01`.

Keep phones **away from the capture TSOP**. An iPhone’s Face ID / LiDAR
illuminator is also 940 nm; that chip will report short garbled `raw` bursts
that have nothing to do with this LED.

There is no Wi-Fi on this board. Capture still POSTs what it hears if you
already set that up.

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

Right after flash, the **sender** OLED shows `aim TSOP` and `carrier ON` /
`carrier off`. Point it at capture. Then:

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
  at the end opposite the USB plug (`e20` / `e19`).
- You actually copied a UF2 (the `RP2350` drive vanished after the copy).
- Some modules use I²C address `0x3D` instead of `0x3C`. In `src/display.rs`,
  change the `0x3C` passed to `Oled::new` to `0x3D`.
- The plastic film on a new OLED can make it look dim; peel it off.

## If capture shows nothing, or raw times instead of `42:01`

- After flash, OLED must say `aim TSOP` (otherwise this UF2 is not on the
  board). Ignore the phone camera. Point the LED at the capture TSOP: that
  screen should show `L` or `stuck L` while sender says `carrier ON`. Still
  nothing: both LED legs on the **top** half of the board (rows `a`–`e`,
  not across the trench), orange jumper in column **17** not **18** (two
  columns left of the blue SCL wire, with the GND column in between), long
  lead in `c32`. Swap the LED if needed — there are spares in the TSAL6200
  pack.
- Do **not** hold an iPhone (or any Face ID / LiDAR phone) near the capture
  TSOP. That illuminator is 940 nm and shows up as short `raw` junk. A TV
  remote is the right “capture still works” check.
- Point the LED at the TSOP lens on the other breadboard, a few centimetres
  away. Do not aim at the Pico or the table. A phone camera is a wiring test,
  not a protocol test: 38 kHz bursts are too short and too dim to trust.
- The orange jumper must be column **17 on the top** (`b17`), the same strip
  as Pico `e17` / GP18. Column **18** on that row is GND — an easy miss.
- Long LED lead is in `c32` (anode). Swapping the LED means it never lights.
- The 220 Ω is from column 28 to 32, not a 15 Ω or 100 Ω.
- You flashed **this** folder (`projects/ir-sender`) onto the sender Pico, and
  ir-capture is still running on the other Pico.
- Capture’s title shows **`H`** at rest. If it shows **`L`**, that board’s
  receiver wiring is wrong; see the ir-capture README.

## Changing the program

Edit `src/main.rs` or `src/ir.rs`, save, put the Pico back in BOOTSEL, then
`make` again.

This folder is only a sandbox. Game firmware will live later under
`projects/lasertag/source/firmware/`. The player board’s IR LED uses a
transistor on GP0, not this GPIO-plus-resistor circuit.

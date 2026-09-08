# IR-capture experiment

A standalone hardware test, separate from the laser-tag firmware. It reads a
Vishay TSOP38238 38 kHz infrared receiver, shows each captured remote button
on the 0.96" I²C OLED, and POSTs the same event to the HTTP listener used by
[`temperature-display`](../temperature-display/).

You do not need to know Rust or electronics already. Follow the steps in order.
If the OLED is already wired from the temperature test, leave those four wires
and add only the receiver.

## What you need

- Raspberry Pi Pico 2 W (RP2350 plus onboard 2.4 GHz Wi-Fi)
- a **2.4 GHz** Wi-Fi network (the Pico cannot join 5 GHz-only access points)
- 0.96" four-pin I²C OLED (the same module as the temperature test)
- Vishay TSOP38238 38 kHz IR receiver (Pi Hut)
- 100 Ω resistor, 100 nF ceramic capacitor, and 4.7 µF electrolytic capacitor
  from the shopping list (the TSOP supply filter)
- a TV remote, or any 38 kHz IR remote
- the full-size breadboard and jumper wires from the shopping list
- a **data** USB cable (charge-only cables will not work)

The PiCowbell and LiPo are not used for this test. USB powers the Pico.

## Safety

- Unplug USB while you insert parts.
- Power the OLED and the TSOP from **3.3 V only**. Do not use `VBUS`
  (physical pin 40, the 5 V USB pin).

## How to plug it together

Open the drawing in this folder:

- [`wiring.svg`](wiring.svg)

Useful references:

- [Interactive Pico pinout](https://pico.pinout.xyz/) — choose Pico 2 W; hover GP15, GP16, GP17, 3V3, GND
- [Pi Hut TSOP38238](https://thepihut.com/products/ir-infrared-receiver-tsop38238)
- [Vishay TSOP382 datasheet](https://www.vishay.com/docs/82491/tsop382.pdf) — pin 1 `OUT`, pin 2 `GND`, pin 3 `VS`
- [Pi Hut 0.96" OLED](https://thepihut.com/products/0-96-oled-display-module-128x64)
- [Raspberry Pi Pico getting started](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html) — BOOTSEL / UF2 upload

### TSOP38238 pinout

Hold the part with the **rounded lens toward you** and the three legs pointing
down. Left to right:

| Leg | Name | Goes to |
|---|---|---|
| left | `OUT` | Pico `GP15` (physical pin 20) |
| middle | `GND` | Pico `GND` (physical pin 38) |
| right | `VS` | Pico `3V3` through 100 Ω, with capacitors to `GND` |

The output is idle-high and goes low while a 38 kHz burst is in view. The chip
has an internal pull-up; firmware also enables the Pico pull-up.

### Assembly

1. Unplug USB. Sit the Pico 2 W **across the centre gap** of the breadboard,
   like a bridge. The USB socket should hang off the top so the cable still
   fits. Count pins from that USB end. The antenna is at the opposite end
   from USB.
2. Keep the OLED in four unused columns, away from the Pico, so each OLED pin
   has its own column. Wire it exactly as in the temperature test:

   | OLED pin | Pico pin | Physical pin (USB at the top) |
   |---|---|---:|
   | `VCC` | `3V3` | 36 (fifth pin down the **right** side) |
   | `GND` | `GND` | 38 (third pin down the **right** side) |
   | `SDA` | `GP16` | 21 (bottom pin on the **right** side) |
   | `SCL` | `GP17` | 22 (second pin from the bottom on the **right** side) |

3. Put the TSOP in three unused columns on the **left** side, lens facing you
   so a remote can see it. Do not bury the lens against the Pico.
4. Receiver wires and filter:

   | TSOP pin | Connection |
   |---|---|
   | `OUT` | jumper to `GP15`, physical pin 20 (bottom pin on the **left** side) |
   | `GND` | jumper to Pico `GND`, physical pin 38 |
   | `VS` | **100 Ω** from Pico `3V3` (pin 36) into the `VS` column |
   | `VS` to `GND` | 100 nF ceramic (either way) in the same two columns |
   | `VS` to `GND` | 4.7 µF electrolytic: **positive** on `VS`, **stripe/minus** on `GND` |

5. On a breadboard, every hole in the same numbered column (on one side of
   the gap) is already connected. Plug each jumper into the same column as the
   Pico pin, then into the part.

The 100 Ω resistor has no polarity. The ceramic capacitor (often marked `104`)
has no polarity. The electrolytic can is polarized: the stripe is negative.

This sandbox uses `GP15` because it sits opposite the OLED pins and is easy to
find. The later player board maps body receivers to `GP10`–`GP13` instead.

## What the firmware does

The TSOP38238 demodulates 38 kHz infrared. The program waits for a burst,
records mark/space times, and:

- decodes **NEC** TV remotes (address and command in hex);
- shows other 38 kHz protocols as `RAW` plus the number of edges;
- treats a held NEC button as a repeat (`rpt` on the OLED).

Wi-Fi SSID, password, and the HTTP POST URL are **not** compiled in. You type
them over USB serial; `save` stores them in flash. After a reboot they are
still there. Flash layout matches `temperature-display`, so a Pico that was
already provisioned for that project keeps its Wi-Fi settings.

Once joined, each captured button POSTs JSON to that URL, for example:

```json
{"proto":"nec","addr":32,"cmd":13,"rep":false}
```

or, if the burst is not NEC:

```json
{"proto":"raw","n":67,"lead":[9020,4480]}
```

The onboard LED (on the wireless chip) turns on when Wi-Fi is up.

## Install the software tools (once)

Same tools as [`temperature-display`](../temperature-display/README.md). If
Rust is already installed, skip to **Build and upload**.

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

Put the Pico 2 W in bootloader mode first:

1. Unplug the USB cable.
2. Hold the **BOOTSEL** button on the Pico (the small button next to the USB
   socket).
3. Plug the USB cable in while still holding BOOTSEL.
4. Release BOOTSEL.
5. Finder should show a drive called **RP2350**.

If nothing appears, the cable is probably charge-only. Try another cable.

Then, from the `lastertag` repository root:

```bash
cd projects/ir-capture
make
```

That compiles the firmware, converts it to a `.uf2`, and copies it onto the
Pico. The `RP2350` drive disappears; that is normal. The program starts
immediately.

The first build downloads crates and can take several minutes. After that,
`make` is the only command you need when the board is already in BOOTSEL.

If `make` builds the UF2 but cannot copy it, drag `ir-capture.uf2` onto the
**RP2350** drive in Finder.

If you only want the files without copying them onto the board:

```bash
make build   # ELF only
make uf2     # ELF plus ir-capture.uf2
```

## What you should see

Text on the OLED, similar to:

```text
IR capture
  ready
USB: wifi/save
```

Point a TV remote at the TSOP lens and press a button. A typical NEC remote
shows address and command in hex:

```text
IR capture
  20:0D
wifi ok  post ok
```

Holding the button adds `rpt`. Unknown 38 kHz encodings show as `RAW` and an
edge count.

The bottom line is status: `joining wifi`, `wifi ok`, `wifi fail`, or
`wifi ok  post ok` after a successful POST.

On a dual-colour 0.96" panel the top band is yellow and the rest is blue.
The title sits in the yellow band; the hex value is blue. That is the glass,
not a wiring fault.

If every press shows `RAW`, the remote is not NEC (common for some TVs). The
timings are still captured and posted.

## USB Wi-Fi and server setup

Flash the board, wait until the OLED is showing `ready`, then open a serial
terminal. On macOS:

```bash
ls /dev/cu.usbmodem*
screen /dev/cu.usbmodem* 115200
```

If several `usbmodem` devices appear, try each until you see a `>` prompt.
Type `help`. Leave `screen` with `Ctrl-A` then `K`, then `Y`.

On this computer, start a listener (binds all interfaces, port 8090). This is
the same program as in `temperature-display`; you only need one copy running:

```bash
bun listen_post.ts
```

It prints this machine’s LAN IP. On the Pico console, using that IP:

```text
wifi YourNetworkName
psk YourPassword
server 192.168.1.23:8090/ir
save
```

You can keep path `/temp` if this Pico was already set up for the temperature
project; the listener accepts any POST path. `psk` with no password means an
open network. `server` can also be a hostname (`nas.local:8090/ir`). The Pico
is 2.4 GHz only.

`save` writes flash and starts joining. `show` prints the current RAM copy.
`clear` wipes the saved settings. After `save`, you can unplug USB and power
from the PiCowbell; the board still has the settings.

Press a remote button. The listener should print a JSON body. The Pico talks
to your **LAN IP**, not `localhost`.

SH1106 panels have 132 columns of RAM and 128 visible. This firmware starts
the write window at column 0 so the left edge is not leftover RAM. If a 2-pixel
bar appears on the **right** instead, change `with_column_offset(0)` in
`src/display.rs` to `with_column_offset(2)` and flash again.

## If the title is readable but the rest is noise

The firmware talks to the panel as an **SH1106** (132-column RAM). Cheap
yellow/blue 0.96" modules are often SH1106 even when the listing says
SSD1306. Driving SH1106 as SSD1306 commonly leaves the first rows readable
and fills the blue area with random pixels.

Reflash this folder with BOOTSEL + `make`. If the value is still garbage, in
`src/display.rs` change `OledConfig::sh1106_128x64()` to
`OledConfig::ssd1306_128x64()` and flash again.

## If the screen stays black

- USB is plugged into the Pico (the board is powered from that cable).
- Every OLED label matches the table. Swapping SDA/SCL is the most common
  mistake.
- `VCC` is on **3V3** (pin 36), not **VBUS** (pin 40).
- You counted pins from the USB end. `GP16`/`GP17` are on the **right** side,
  at the end opposite the USB plug.
- You actually copied a UF2 (the `RP2350` drive vanished after the copy).
- Some modules use I²C address `0x3D` instead of `0x3C`. In `src/display.rs`,
  change the `0x3C` passed to `Oled::new` to `0x3D`.
- The plastic film on a new OLED can make it look dim; peel it off.

## If nothing happens when you press the remote

- The TSOP lens faces the remote, not the Pico or the table.
- `OUT` is on **GP15** (pin 20), not an adjacent pin. Swapping `OUT` and `VS`
  can make the part hot; unplug USB and recheck left-to-right: `OUT`, `GND`,
  `VS` with the lens toward you.
- `VS` is 3.3 V through 100 Ω, not 5 V.
- The electrolytic stripe is on **GND**, not on `VS`.
- Use a 38 kHz remote. The TSOP38238 ignores other carriers (for example
  36 kHz-only or 56 kHz gadgets).
- Range is typically tens of centimetres to a few metres indoors. Try closer,
  and avoid pointing a bright lamp straight into the lens.

## Changing the program

Edit `src/main.rs` or `src/ir.rs`, save, put the Pico back in BOOTSEL, then
`make` again.

This folder is only a sandbox. Game firmware will live later under
`projects/lasertag/source/firmware/`.

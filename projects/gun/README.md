# Laser-tag gun

Handheld IR gun based on the [`ir-sender`](../ir-sender/) experiment. Firmware
is the same NEC button-to-LED behaviour for now; hardware diverges into a
soldered assembly and a Glock 17–proportioned printable shell.

Leave [`ir-sender`](../ir-sender/) and [`ir-capture`](../ir-capture/) as they
are. This folder is the place to evolve the gun.

## What you need

Electronics you already have (shopping list):

- Raspberry Pi Pico 2 W (with headers)
- 0.96" I²C OLED
- Vishay TSAL6200 IR LED
- BC337-40, two 15 Ω (1 W), one 220 Ω, one 10 kΩ
- 100 nF ceramic and 4.7 µF electrolytic (decoupling at the LED)
- Diptronics DTS-644K tactile switch (trigger)
- 500 mAh LiPo with JST-PH

Extras for the permanent build (see [`hardware/README.md`](hardware/README.md)):

- soldering tools, stranded hookup wire, heat-shrink
- JST-PH 2.0 mm female pigtail
- Schottky diode (1N5817 or similar) for battery → VSYS
- M2×8 screws, bright filament + orange for the muzzle

The Adafruit PiCowbell stays on the bench as a **charging dock**. It does not
fit in the grip. Remove the dummy magazine, unplug the cell, charge on the
PiCowbell.

## Pin map (unchanged from ir-sender)

| Function | Pico | Notes |
|---|---|---|
| OLED SDA | GP16 | I²C |
| OLED SCL | GP17 | I²C |
| IR LED base drive | GP18 | through 220 Ω → BC337 |
| Trigger | GP19 | to GND through the switch; internal pull-up |
| Logic supply | 3V3 (pin 36) | OLED and IR LED path |
| Ground | GND (pin 38) | common |
| Battery | VSYS (pin 39) | via Schottky from the mag JST |
| USB 5 V | VBUS (pin 40) | leave empty |

## Hardware and CAD

- [`hardware/README.md`](hardware/README.md) — gap list, power rules, soldering
- [`hardware/connections.svg`](hardware/connections.svg) — soldered nets (no breadboard)
- [`cad/README.md`](cad/README.md) — print settings and assembly
- [`cad/gun.scad`](cad/gun.scad) — printable Glock-17-proportioned shell

## Safety

- Infrared LED only. Never substitute a laser diode.
- Power the OLED and IR LED from **3.3 V**, not `VBUS`.
- Battery `+` goes through a Schottky diode into **VSYS**. Never wire the cell
  straight across USB 5 V.
- **Remove the magazine before plugging Micro-USB into the Pico.**
- Charge the LiPo on the PiCowbell where you can watch it. Do not use a
  swollen, punctured, or hot cell.

## What the firmware does

Same as ir-sender: each trigger press sends three NEC frames (address `0x42`,
command random 1–10). The OLED shows `gun` / `ready` / `sent 42:xx`. Optional
USB serial `debug on` / `save` toggles DC / 1 s beacon modes for a meter.

Point the muzzle at an [`ir-capture`](../ir-capture/) board; it should show
`42:01`–`42:0A`.

## Build and upload

1. Hold **BOOTSEL** on the Pico, plug USB (cable must be data-capable), release.
2. Finder should show **RP2350**.
3. From the repository root:

```bash
cd projects/gun
make
```

That builds `gun.uf2` and copies it onto the drive. Or `make uf2` and drag the
file yourself.

First build downloads crates and can take several minutes.

## USB serial

```bash
ls /dev/cu.usbmodem*
screen /dev/cu.usbmodem* 115200
```

Banner: `Pico 2 W gun`. Leave `screen` with `Ctrl-A` then `K`, then `Y`.

## Next steps (not this folder yet)

Game firmware, Wi-Fi, body receivers, and the player pin map (GP0 / GP2 / …)
live later under `projects/lasertag/`. Keep this pinout until then so the
soldered harness stays compatible with the current UF2.

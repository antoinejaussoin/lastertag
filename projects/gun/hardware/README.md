# Gun hardware (soldered, no breadboard)

This is the permanent packing for [`projects/gun`](../). The electrical net
matches [`ir-sender`](../../ir-sender/) plus battery power on `VSYS`. There is
no PiCowbell and no breadboard in the shell.

Drawing: [`connections.svg`](connections.svg).

## Gap list — what you have vs what you need

### Already good enough to shoot

| Part | Role |
|---|---|
| Pico 2 W | controller, USB programming, onboard LED |
| TSAL6200 + BC337-40 + 2×15 Ω + 220 Ω + 10 kΩ | IR transmitter |
| DTS-644K | trigger |
| 0.96" I²C OLED | status display |
| 100 nF + 4.7 µF | LED supply decoupling (use them; breadboard skipped them) |
| 500 mAh LiPo (JST-PH) | portable power |

### Not in the gun (on purpose)

| Part | Why |
|---|---|
| Adafruit Proto Doubler PiCowbell | too large (~51×51 mm); keep as **bench charger** |
| Breadboard / jumper kit | replaced by soldered harness |
| Body TSOP receivers | wearable harness later |
| Extra reload / menu buttons | firmware later |

### Buy before soldering

| Item | Purpose |
|---|---|
| Soldering iron, flux, solder, cutters, strippers | assembly |
| 22 AWG stranded hookup wire (or cut solid jumpers) | flying leads |
| Heat-shrink tubing | insulation on the IR nest |
| **JST-PH 2.0 mm 2-pin female pigtail** | mates the battery in the magazine |
| **1N5817 Schottky** (or similar) | battery `+` → diode → Pico `VSYS` |
| 8–12× M2×8 screws | clamshell |
| Bright filament + orange filament | body + toy muzzle collar |

## Power rules

```text
LiPo + ──► JST female ──► Schottky anode ──► cathode ──► Pico VSYS (pin 39)
LiPo − ──► JST female ───────────────────────────────► Pico GND  (pin 38)
USB Micro ───────────────────────────────────────────► Pico USB (VBUS unused on GPIO)
```

- The Pico onboard diode already feeds USB into `VSYS`. The **extra** Schottky
  stops USB 5 V from charging the LiPo through the battery lead.
- Charge only on the PiCowbell: mag out → unplug cell JST → plug into PiCowbell
  → charge from that Pico’s USB.
- Operational rule even with the diode: **remove the magazine before plugging
  Micro-USB into the gun Pico.**
- Never connect the battery to `VBUS` (pin 40). Leave pin 40 empty.
- IR LED and OLED always from regulated **3V3** (pin 36), never from `VSYS`
  (battery voltage rises to ~4.2 V when charged and would change LED current).

## Pin map

| Net | Pico pin | Goes to |
|---|---|---|
| `+3V3` | 36 | OLED VCC, two 15 Ω, decoupling |
| `GND` | 38 | OLED GND, BC337 emitter, 10 kΩ, button, battery − |
| `VSYS` | 39 | Schottky cathode |
| `SDA` | GP16 / 21 | OLED SDA |
| `SCL` | GP17 / 22 | OLED SCL |
| `IR_TX` | GP18 / 24 | 220 Ω → BC337 base |
| `BTN` | GP19 / 25 | one side of trigger switch |
| `VBUS` | 40 | **leave open** |

Keep headers on the Pico. Solder wires onto the header pins you use; leave the
rest free so the board sits flat in the grip tray.

## BC337-40 (Diotec)

Flat face toward you, legs down: **C · B · E** left to right.

- Collector ← LED cathode (short lead / flat rim)
- Base ← 220 Ω from GP18, and 10 kΩ to GND
- Emitter → GND

## OLED

Typical Pi Hut order is `VCC GND SCL SDA`. **Read the silk on your module** —
orders vary. VCC is 3.3 V only.

## Trigger button (DTS-644K)

Four legs. The **6.5 mm** pair is already joined inside the switch. Use the
**4.5 mm** pair: one leg → GP19, other → GND. Do not also ground a permanently
joined leg or the pin stays low forever.

## Soldering sequence (three harnesses + power)

Build and heat-shrink on the bench before stuffing the shell.

### 1. IR nest (muzzle end)

Point-to-point / dead-bug, keep the high-current loop short:

1. Join the two 15 Ω in series.
2. Free end of the pair → Pico 3V3 (will run as a long lead into the grip).
3. Other end → TSAL6200 **anode** (long lead).
4. Cathode → BC337 **collector**.
5. Emitter → GND lead.
6. GP18 lead → 220 Ω → base.
7. Base → 10 kΩ → same GND.
8. 100 nF across local 3V3/GND at the LED.
9. 4.7 µF across the same nodes; **stripe / short lead = minus (GND)**.

Approximate LED current from 3.3 V with 30 Ω: ~58 mA. Safe starting value.

### 2. OLED pigtail

Four wires: VCC, GND, SCL, SDA → pins 36, 38, GP17, GP16.

### 3. Trigger

Two wires from the DTS-644K 4.5 mm pair → GP19 and GND.

### 4. Battery pigtail

- Female JST-PH hangs into the mag well.
- Red (battery +) → Schottky **anode**; cathode → VSYS (pin 39).
- Black → GND (pin 38).
- Confirm LiPo red aligns with `+` before first plug-in.

## Bench test order

1. **USB only, no battery.** Flash `projects/gun`, OLED shows `gun` / `ready`,
   title ends in `H` at rest. Press trigger → `sent 42:xx`. Capture board sees
   `42:01`–`42:0A`.
2. **Battery only, no USB.** Mag in, diode polarity correct, same behaviour.
3. Fit into the printed shell (see [`../cad/README.md`](../cad/README.md)).
4. Only then combine USB + battery if needed for serial while powered; still
   prefer mag-out when plugging USB.

## Inspection before power

1. No wire on VBUS.
2. OLED VCC on 3V3, not 5 V.
3. No 15 Ω on GP18 (only 220 Ω to the base).
4. Electrolytic polarity and LED anode/cathode correct.
5. BC337 C-B-E matched to the Diotec package.
6. Button does not short GP19 at rest (title must be `H`).

## PiCowbell charging dock

1. Power the gun off by removing the magazine.
2. Unplug the cell’s male JST from the gun’s female pigtail.
3. Plug the cell into the PiCowbell JST (red to `+`).
4. Charge from USB on the Pico seated in the PiCowbell.
5. Never charge a damaged cell.

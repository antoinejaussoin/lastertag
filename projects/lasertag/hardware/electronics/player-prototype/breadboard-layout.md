# Beginner breadboard assembly map

This guide builds **one player board**. Repeat it exactly on the second
breadboard for player two.

![Breadboard placement map](breadboard-layout.svg)

Do not connect the battery or USB cable until the final inspection. Never move
a wire or component while power is connected.

## 1. How the breadboard works

Place the breadboard horizontally with column `1` at the left and column `63`
at the right:

```text
top power rails       red (+) and blue (-), connected sideways
upper terminal strip  rows a b c d e
centre trench         no electrical connection across it
lower terminal strip  rows f g h i j
bottom power rails    blue (-) and red (+), connected sideways
```

Within one numbered column, holes `a` through `e` are connected together.
Holes `f` through `j` in that column form a second, separate group. Adjacent
numbered columns are not connected. The long power rails connect sideways,
although some boards split each rail at the centre mark.

Coordinates such as `c20` mean row `c`, column `20`. The drawing is a
placement aid; the coordinate tables below remove any ambiguity.

## 2. Recognize and orient the parts

- **TSOP38238 receiver:** black rounded package with three legs. Hold the
  rounded lens toward you and the legs downward. Left to right is
  `OUT`, `GND`, `VS`.
- **TSAL6200 infrared LED:** looks like a clear or blue-grey 5 mm LED. The
  long leg is the positive **anode**. The short leg beside the flat rim is the
  negative **cathode**.
- **BC337-40 transistor:** small black part with one flat face and three legs.
  For the purchased Diotec part, flat face toward you and legs downward is
  `C`, `B`, `E` from left to right. Do not substitute a differently named
  transistor without checking its pin order.
- **4.7 uF electrolytic capacitor:** small cylinder. The stripe marks the
  negative leg. The unstriped/longer leg is positive.
- **100 nF ceramic capacitor:** small non-cylindrical part marked `104`. It
  has no positive or negative side.
- **Resistors:** they have no positive or negative side. Keep them separated
  in their labelled bags while assembling.
- **Push button:** four legs form two permanently joined pairs. It must bridge
  the breadboard centre trench as shown.
- **OLED:** use the labels printed beside its four pins. Do not infer the power
  pins from their left-to-right position.

## 3. Connect the power rails

Use red jumpers for `3V3` and black jumpers for `GND`.

1. PiCowbell/Pico physical pin `36`, labelled `3V3`, to the **top red rail** at
   column 1.
2. PiCowbell/Pico physical pin `38`, labelled `GND`, to the **top blue rail**
   at column 1.
3. Bridge the left and right halves of both top rails across the centre split.
   Even if your rails are continuous, these two short jumpers do no harm.
4. Join the top blue rail to the bottom blue rail at the far right.
5. Join the top red rail to the bottom red rail at the far right.

Never connect Pico `VBUS` or `VSYS` to a breadboard rail.

## 4. Fit the OLED

Insert its four pins into `a3`, `a4`, `a5`, and `a6`. The display body should
extend away from the centre trench.

Follow the words printed on the actual OLED:

| OLED label | Connect to |
|---|---|
| `VCC` | top red `3V3` rail |
| `GND` | top blue `GND` rail |
| `SCL` | PiCowbell/Pico GP5, physical pin 7 |
| `SDA` | PiCowbell/Pico GP4, physical pin 6 |

Because OLED pin order varies, the map labels these as `P1` to `P4` instead of
guessing which physical hole is VCC.

## 5. Fit the three buttons

Each button straddles the centre trench. Its four legs occupy:

| Button | Four holes | Signal jumper | Ground jumper |
|---|---|---|---|
| Trigger SW1 | `e8`, `e10`, `f8`, `f10` | GP2 physical pin 4 to `j8` | `j10` to bottom blue rail |
| Reload SW2 | `e13`, `e15`, `f13`, `f15` | GP3 physical pin 5 to `j13` | `j15` to bottom blue rail |
| Menu SW3 | `e17`, `e19`, `f17`, `f19` | GP6 physical pin 9 to `j17` | `j19` to bottom blue rail |

The signal and ground must go to opposite internally connected sides. If a
button appears permanently pressed later, rotate it 90 degrees and place it
again across the trench.

## 6. Build the four receiver zones

Insert every receiver with its rounded lens facing the **bottom edge** of the
breadboard. Its pins then run left-to-right as `OUT`, `GND`, `VS`.

| Zone | Receiver pins `OUT,GND,VS` | 100 ohm from red rail to | 100 nF capacitor | 4.7 uF capacitor | GPIO jumper |
|---|---|---|---|---|---|
| Front U2 | `c20,c21,c22` | `a22` | `d22` to `d23` | `e22` (+) to `e24` (-) | `e20` to GP10 pin 14 |
| Back U3 | `c29,c30,c31` | `a31` | `d31` to `d32` | `e31` (+) to `e33` (-) | `e29` to GP11 pin 15 |
| Left U4 | `c38,c39,c40` | `a40` | `d40` to `d41` | `e40` (+) to `e42` (-) | `e38` to GP12 pin 16 |
| Right U5 | `c47,c48,c49` | `a49` | `d49` to `d50` | `e49` (+) to `e51` (-) | `e47` to GP13 pin 17 |

For each row in the table:

1. Put one leg of its 100 ohm resistor in the top red rail and the other in the
   listed `a` hole.
2. Put the 100 nF capacitor in its two listed holes.
3. Put the electrolytic capacitor in its two listed holes, with positive on
   the receiver `VS` column and striped negative on the other column.
4. Join the receiver `GND` column to the top blue rail with black wire.
5. Join both capacitor ground columns to the top blue rail with black wire.
6. Add the blue signal jumper from `OUT` to the listed Pico pin.

Do not replace the three separate ground jumpers with links between columns:
joining adjacent columns could short a receiver supply to ground.

## 7. Build the IR transmitter

Place the Diotec BC337-40 at `h55`, `h56`, `h57`, with its **flat face toward
the bottom edge**. This makes:

- `h55` = collector `C`;
- `h56` = base `B`;
- `h57` = emitter `E`.

Then place:

| Part | First end | Second end | Orientation |
|---|---|---|---|
| R1, 15 ohm 1 W | bottom red rail column 45 | `j45` | either way |
| R4, 15 ohm 1 W | `i45` | `i52` | either way |
| D1, TSAL6200 | long anode in `h52` | short/flat-side cathode in `h55` | polarity matters |
| R2, 220 ohm | `f63` | `f59` | either way |
| R5, 220 ohm | `g59` | `g54` | either way |
| R3, 10 kohm | `j56` | bottom blue rail column 56 | either way |
| C1, 4.7 uF | bottom red rail column 47 | bottom blue rail column 47 | positive to red |
| C2, 100 nF | bottom red rail column 50 | bottom blue rail column 50 | either way |

Finish with:

1. GP0, physical pin 1, to `j63` with a blue jumper.
2. Join `i54` to `i56` with a short insulated jumper. This connects the end of
   R5 to the transistor base.
3. `j57` to the bottom blue ground rail with black wire.

R1 and R4 are electrically in series because holes `i45` and `j45` are joined
inside the breadboard. R2 and R5 are similarly joined through column 59. Do
not omit either resistor or the `i54`-to-`i56` jumper.

## 8. PiCowbell signal checklist

These are the only signal jumpers between the PiCowbell/Pico and breadboard:

| Pico physical pin | Printed GPIO | Breadboard destination |
|---:|---|---|
| 1 | GP0 | `j63`, IR transmitter |
| 4 | GP2 | `j8`, trigger |
| 5 | GP3 | `j13`, reload |
| 6 | GP4 | OLED pin labelled `SDA` |
| 7 | GP5 | OLED pin labelled `SCL` |
| 9 | GP6 | `j17`, menu |
| 14 | GP10 | `e20`, front receiver |
| 15 | GP11 | `e29`, back receiver |
| 16 | GP12 | `e38`, left receiver |
| 17 | GP13 | `e47`, right receiver |
| 36 | 3V3 | top red rail |
| 38 | GND | top blue rail |

Use the labels printed on the PiCowbell. Do not count pins from memory.

## 9. Inspect before connecting power

With USB and battery still disconnected:

1. Check every part against the map, one column at a time from left to right.
2. Check every striped electrolytic side goes to a blue ground rail.
3. Check D1's long leg is at `h52` and short/flat side is at `h55`.
4. Check Q1's flat face points toward the bottom edge.
5. Check all four receivers' rounded lenses point toward the bottom edge.
6. Check no component or wire joins a red rail directly to a blue rail.
7. Check neither `VSYS` nor `VBUS` is connected to the breadboard.
8. Ask another person to repeat the visual check if possible.

Only after inspection:

1. Put the PiCowbell switch in the off position.
2. Verify the LiPo red wire aligns with `+` at the PiCowbell socket.
3. Connect the LiPo.
4. Switch on for five seconds while watching, smelling, and carefully checking
   for heat without touching bare conductors.
5. Switch off immediately if anything heats, smells, discolours, or if the
   battery changes shape.

Nothing useful will appear on the OLED and the IR circuit will remain off until
firmware is installed. That is expected.


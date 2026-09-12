# CAD — printable gun shell

Glock 17 Gen5 proportions with pockets sized for the Pico 2 W, OLED, LiPo
magazine, IR LED, and trigger. Source: [`gun.scad`](gun.scad).

This is a **toy IR marker**, not a firearm replica: bright body colour, orange
muzzle collar, no trademarks or serial markings. The frame is wider than a real
G17 so the electronics fit.

## Parts

| STL / part | Filament | Notes |
|---|---|---|
| `frame_left.stl` | bright body colour | grip, mag well, Pico tray, USB heel cutout |
| `frame_right.stl` | bright body colour | mates with left; M2 screw tunnels |
| `slide.stl` | body or darker accent | OLED optic hump at rear |
| `trigger.stl` | body | pivot on 1.75 mm filament scrap |
| `magazine.stl` | body | holds 500 mAh LiPo + foam |
| `muzzle_collar.stl` | **orange** | toy tip over barrel |
| `oled_bezel.stl` | body or black | window over the OLED glass |

Preview the assembly in OpenSCAD with `part = "preview";` at the top of
`gun.scad`.

## Measure before a long print

Edit the named constants in `gun.scad` if your parts differ:

| Constant | Default (mm) | What to measure |
|---|---:|---|
| `oled_pcb_l` / `oled_pcb_w` | 28 / 28 | your 0.96" module PCB |
| `lipo_l` / `lipo_w` / `lipo_h` | 36 / 31 / 7 | cell + thin foam |
| `pico_h` | 12 | board + headers + solder blobs |
| `frame_width` | 42 | widen further if wires bind |

Print a 20 mm calibration cube and a quick `oled_bezel` first.

## Print settings

| Setting | Value |
|---|---|
| Material | **PETG** preferred (PLA OK for fit-check) |
| Layer height | 0.2 mm |
| Walls | 4 |
| Infill | 25% gyroid or grid |
| Supports | yes under trigger guard and Pico overhangs |
| Bed | clean; brim optional on the tall grip halves |

Orient:

- frame halves: mating face on the bed (flat split)
- slide: top deck on the bed
- magazine: base on the bed
- muzzle collar: flat end on the bed
- trigger: pivot ear on the bed

## Export STLs

```bash
cd projects/gun/cad
make
```

If [OpenSCAD](https://openscad.org/) is installed, that renders [`gun.scad`](gun.scad)
into `stl/`. If not, `make` runs [`export_stl.py`](export_stl.py) and writes
**approximate** box/cylinder meshes with the same pocket dimensions so you can
slice something immediately. Prefer OpenSCAD for the final print:

```bash
brew install --cask openscad   # macOS
cd projects/gun/cad && make    # re-export from gun.scad
```

Or open `gun.scad`, set `part = "frame_left";`, and use **File → Export → Export as STL**.

## Assembly

1. Solder and bench-test harnesses ([`../hardware/README.md`](../hardware/README.md)).
2. Seat the Pico in the left grip tray: USB toward the heel opening, antenna
   toward the beavertail (plastic keep-out only — no battery under the antenna).
3. Route OLED wires up into the slide optic pocket; clip the bezel over the glass.
4. Drop the IR nest into the muzzle cavity; LED faces forward in the barrel bore.
5. Glue or friction-fit the orange muzzle collar.
6. Place the DTS-644K in the trigger pocket; install the printed trigger; pin
   with a short length of 1.75 mm filament through the pivot holes.
7. Join left/right frames with M2×8 screws (8 points). Slide sits on the frame
   rails — friction or a dab of glue for the prototype.
8. Pad the LiPo with foam in the magazine; lead exits the top to the female JST
   hanging in the mag well. Insert mag to power on.

## Screws and hardware

- 8–12× M2×8 machine screws (or self-tapping into the bosses)
- 1× ~20 mm of 1.75 mm filament as trigger pivot
- Soft foam scrap for LiPo padding
- Optional: a drop of cyanoacrylate on the muzzle collar

## Fit checklist

- Micro-USB plug clears the heel without stressing the Pico
- BOOTSEL reachable through the left-side hole
- Mag inserts/removes without snagging the JST
- Trigger returns and the OLED title shows `H` at rest / `L` when held
- IR LED is recessed and points straight down the barrel

## Legal / safety notes (UK)

- Present and use as a **toy laser-tag marker**.
- Keep the **orange muzzle** fitted for public carry/play.
- Do not add realistic trademarks, serial numbers, or steel finishes that
  obscure the toy nature.
- Infrared only — never a laser diode.

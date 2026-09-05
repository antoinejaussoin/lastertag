# Shopping list

This is a complete procurement list for the first **two-player, four-zone per
player prototype**. Quantities include a small number of fragile-part spares.
It assumes access to a phone or computer for programming and viewing the web
interface, but no electronics tools, network, server, or workshop supplies.

Product availability varies by country. Match the manufacturer part number or
the stated electrical specification; do not replace parts solely because they
look similar.

## Core player electronics

| Qty | Buy | Required specification / suggested part |
|---:|---|---|
| 3 | Raspberry Pi Pico WH | Official Pico W with pre-soldered headers; one per player and one spare |
| 3 | 0.96-inch I²C OLED module | 128×64, SSD1306, four-pin, explicitly 3.3 V compatible; two used and one spare |
| 6 | 940 nm IR emitter LED | Vishay **TSAL6200**; two used and four for range experiments/spares |
| 10 | 38 kHz IR receiver | Vishay **TSOP38438**; eight used and two spares |
| 10 | NPN switching transistor | **BC337-40**, through-hole TO-92, from Diotec or another established manufacturer; two used, remainder are spares/future feedback drivers |
| 4 | Lever microswitch | Omron **SS-5GL2-F**, SPDT hinge roller lever with low operating force and solder terminals; two triggers and two spares |
| 10 | Momentary push button | R-TECH **780382**, black, SPST normally open, panel-mount, solder tags; Rapid order code **78-0382**. Four used for reload/menu controls; sold in multiples of five |
| 3 | Pico LiPo charger/power board | Pimoroni **LiPo SHIM for Pico, PIM557**; two used and one spare. Includes USB charging, battery protection, and power button |
| 3 | Rechargeable LiPo battery | **1200 mAh, 3.7 V, single-cell, 2-pin JST-PH 2.0 mm**, as sold by The Pi Hut; two used and one spare |
| 1 | USB wall charger | Reputable UKCA/CE-marked, two USB-A outputs, regulated 5 V, at least 2 A total |
| 2 | Micro-USB charging/data cable | Reputable 1 m cable, one per player for simultaneous charging |

The PIM557 is soldered to the Pico W and charges the attached battery when the
Pico's Micro-USB port receives power. It has overcurrent and over-discharge
protection plus its own power button, so no separate power switch is needed.
Always verify that battery connector polarity matches the `+` and `-` markings
on the SHIM before connection. Do not use ordinary USB power banks: many shut
themselves off at Pico-sized loads.

## Prototype passives and interconnect

| Qty | Buy | Purpose |
|---:|---|---|
| 2 | Full-size solderless breadboard | TruComponents **654958**, 830 points, 184×106 mm; Rapid code **65-4958**. One complete player circuit per board | https://thepihut.com/products/full-sized-breadboard
| 1 | 20 cm male–male jumper ribbon | Rapid **RW-D40-MM**, 40 ways; Rapid code **34-0688** |
| 1 | 20 cm male–female jumper ribbon | Rapid **RW-D40-MF**, 40 ways; Rapid code **34-0686** |
| 1 kit | Through-hole resistor assortment | Velleman **K/RES-E12**, 610-piece 1/4 W carbon-film E12 kit; Rapid code **13-0201**. Includes 100 Ω, 330 Ω, 1 kΩ, 4.7 kΩ, and 10 kΩ |
| 10 | 15 Ω, 1 W resistor | Royal Ohm **MFF1WFF0150A10**, flame-proof axial metal-film, 1%; Rapid code **62-8705** |
| 1 kit | Ceramic capacitor assortment | Velleman **K/CAP1**, 224 pieces, 50 V; Rapid code **13-0206**. Includes twenty-one 100 nF capacitors |
| 1 kit | Electrolytic capacitor assortment | Velleman **K/CAP2**, 120 pieces, 1–1000 µF; Rapid code **13-0221** |
| 10 | 3-way JST-XH PCB header | JST **B3B-XH-A(LF)(SN)**; Rapid code **22-5322** |
| 25 | 3-way JST-XH cable housing | JST **XHP-3**; Rapid code **22-5362**. Ten used; Rapid pack quantity provides spares |
| 10 | 4-way JST-XH PCB header | JST **B4B-XH-A(LF)(SN)**; Rapid code **22-5324**. Four used |
| 25 | 4-way JST-XH cable housing | JST **XHP-4**; Rapid code **22-5364**. Four used; Rapid pack quantity provides spares |
| 50 | JST-XH female crimp contact | JST **BXH-001T-P0.6**; Rapid code **22-5374**. Shared by the 3- and 4-way housings |
| 1 set | Stranded hookup wire | Quadrios **23T038**, 10 m assorted 0.14 mm² wire; Rapid code **06-3528** |
| 1 reel | Three-conductor receiver cable | Donau **318-014**, 5 m, 3×0.14 mm² red/black/green; Rapid code **02-5180** |
| 1 set | Pre-cut solid breadboard jumpers | TruComponents **654961**, 75-piece multicolour jumper set; Rapid code **65-4961** |
| 1 box | Heat-shrink assortment | DSG-Canusa **8011005990**, 64-piece, 2:1, 2.4–19 mm; Rapid code **63-0077** |
| 1 bag | Small cable ties | UniStrand **UNI-CT1B**, black 100×2.5 mm, pack of 100; Rapid code **04-0861** |
| 2 rolls | Fabric wiring-harness tape | Coroplast **80284**, black 15 mm × 10 m; Rapid code **11-6563**. Rapid minimum quantity is two |

The resistor kit values are for GPIO/base bias and pull-ups/pull-downs. The
separate 15 Ω, 1 W parts are the starting emitter resistors, not a final value:
measure LED pulse current before range testing and change the value if needed.

## Permanent prototype assembly

Buy these after the breadboard IR link passes its electrical tests, but include
them in the budget now.

| Qty | Buy | Required specification |
|---:|---|---|
| 4 | Through-hole prototyping board | Approximately 70×90 mm, plated holes; two used and two spares |
| 6 | 1×20 female header strip | 2.54 mm pitch; two form a removable socket for each Pico WH |
| 1 | JST-compatible crimp tool | Engineer **PA-09** or ratcheting tool specified for the purchased contacts |
| 1 box | Nylon M2/M2.5 standoff kit | Electrically isolated board mounting |
| 1 roll | Hook-and-loop strap | 20–25 mm wide, for temporary wearable mounting |
| 1 m | 25 mm elastic webbing | Prototype shoulder/torso receiver harness |
| 8 | Small reusable hook-and-loop pads | Attach four receiver pods per player |
| 2 | Small ABS project box | One temporary controller/battery enclosure per player |

## Local game server and network

This section makes the system self-contained. If a suitable computer and Wi-Fi
router are already available, they can replace this entire section during
development.

| Qty | Buy | Required specification / suggested part |
|---:|---|---|
| 1 | Raspberry Pi 5 | 4 GB RAM |
| 1 | Raspberry Pi 5 power supply | Official 27 W USB-C supply for the local mains plug |
| 1 | Raspberry Pi 5 case with cooling | Official case with fan, or case plus official active cooler |
| 1 | microSD card | 64 GB, Application Performance Class A2, reputable brand |
| 1 | USB microSD reader | USB 3; needed unless the setup computer has a reader |
| 1 | Travel Wi-Fi router | GL.iNet **GL-MT3000 (Beryl AX)** or equivalent private 2.4 GHz network |
| 1 | Ethernet cable | Cat 5e or better, 1–2 m, server to router |

Pico W uses 2.4 GHz Wi-Fi, so the router must expose a 2.4 GHz SSID even if it
also supports 5 GHz. Keep this game network private and change all default
passwords. Internet service is not required.

## Soldering and electronics tools

| Qty | Buy | Required specification / suggested part |
|---:|---|---|
| 1 | Temperature-controlled soldering station | Hakko **FX-888DX** with a small chisel tip, correct local voltage |
| 1 | Soldering-iron stand/tip cleaner | Stand plus brass wool; omit only if included with the station |
| 1 roll | Lead-free electronics solder | 0.6–0.8 mm, Sn99/Cu or SAC alloy, flux core, 100 g |
| 1 | No-clean flux pen | Electronics-grade; compatible with chosen solder |
| 1 roll | Desoldering braid | 2–2.5 mm, fluxed copper |
| 1 | Solder sucker | Spring-loaded through-hole desoldering pump |
| 1 | Digital multimeter | Auto-ranging with continuity buzzer, DC voltage/current, and fused current inputs |
| 1 | Flush cutter | Small electronics-grade side/flush cutter |
| 1 | Wire stripper | Covers 20–30 AWG |
| 1 | Needle-nose pliers | Small, smooth-jaw electronics type |
| 1 | Precision screwdriver set | Phillips, slotted, hex, and Torx bits |
| 1 | Helping-hands PCB holder | Stable weighted base or articulated board vise |
| 1 | Silicone soldering mat | Heat resistant, approximately A3 size |
| 1 | Fume extractor | Bench unit with replaceable activated-carbon filter |
| 2 pairs | Safety glasses | One for the builder and one for anyone observing |
| 1 | USB-C data cable/adapter | Appropriate for the setup computer if it lacks USB-A |

Use ventilation even with a fume extractor. Wear eye protection while cutting
leads and soldering. Wash hands after handling electronics and do not eat or
drink at the soldering bench. Verify the multimeter's lead sockets before every
current or voltage measurement.

## General fabrication supplies

| Qty | Buy | Purpose |
|---:|---|---|
| 1 | Metric ruler and digital caliper | Enclosure and mounting measurements |
| 1 | Craft knife with spare blades | Heat-shrink, tape, and template work |
| 1 | Small hand-drill set | 1–5 mm bits for temporary project boxes |
| 1 | Deburring tool or small file set | Remove sharp enclosure edges |
| 1 | Hot-glue gun and glue sticks | Temporary strain relief only; not electrical insulation |
| 1 roll | Quality electrical tape | Temporary insulation and labeling |
| 1 | Permanent marker/label set | Device, cable, battery-set, and zone IDs |

## 3D-printing placeholder

Do **not** buy a printer yet. The enclosure dimensions will change after
electrical and ergonomic testing. The planned printable parts are:

- left/right gun shell and removable electronics cover;
- trigger and button mounts;
- IR LED/optics carrier;
- OLED bezel;
- receiver sensor pods;
- Pico W controller and battery enclosures;
- cable clips and strain-relief pieces.

For the first enclosure iteration, order PLA/PETG parts from a print service or
use the temporary ABS project boxes. Printer, material, fastener, and finishing
recommendations will be added after CAD requirements exist.

## Scaling beyond two players

For every additional player, add:

- 1 Raspberry Pi Pico WH board;
- 1 SSD1306 OLED;
- 1 TSAL6200 IR LED and 1 BC337-40 driver;
- 4 TSOP38438 receivers (or one per chosen body zone);
- 1 trigger microswitch and 2 control buttons;
- 1 PIM557 LiPo SHIM and 1 compatible 1200 mAh JST-PH LiPo battery;
- 4 three-pin receiver cable pairs;
- enough wire, perfboard, connectors, harness material, and enclosures for one
  complete player unit.

Server and router capacity must be load-tested before choosing a maximum `N`.
The architecture should not encode a fixed two-player limit.

## Before placing the order

1. Confirm mains voltage and plug types for the soldering station, server power
   supply, charger, and router.
2. Confirm that OLED listings explicitly support 3.3 V logic.
3. Confirm exact TSOP38438 and TSAL6200 part numbers; generic IR sensors often
   use different carrier frequencies or pin orders.
4. Download each semiconductor datasheet and verify pin orientation before
   wiring. In particular, the BC337-40 pin order is commonly different from
   PN2222/2N2222 parts.
5. Buy LiPo batteries from a reputable seller, verify JST polarity before
   connection, and reject any cell that arrives swollen, bent, punctured, or
   otherwise damaged.

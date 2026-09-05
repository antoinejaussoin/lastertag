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
| 5 | Raspberry Pi Pico WH | Official Pico W with pre-soldered headers; four used and one spare |
| 3 | 0.96-inch I²C OLED module | 128×64, SSD1306, four-pin, explicitly 3.3 V compatible; two used and one spare |
| 6 | 940 nm IR emitter LED | Vishay **TSAL6200**; two used and four for range experiments/spares |
| 10 | 38 kHz IR receiver | Vishay **TSOP38438**; eight used and two spares |
| 10 | NPN switching transistor | onsemi **PN2222AG**, through-hole TO-92; two used, remainder are spares/future feedback drivers |
| 4 | Lever microswitch | Omron **SS-5GL2** or equivalent; two triggers and two spares |
| 6 | Momentary push button | Normally open, panel-mount, 12 mm; reload/menu controls plus two spares |
| 6 | Latching power switch | SPST, rated for at least 1 A at 5 V DC; four used and two spares |
| 4 | 3×AA battery holder | Leads attached, no built-in series resistor; one per Pico W node |
| 16 | Rechargeable AA cells | Panasonic **eneloop BK-3MCCA**, low-self-discharge NiMH; twelve installed and four spares |
| 1 | Smart NiMH charger | Panasonic **BQ-CC55** or equivalent independent-channel charger for AA NiMH |

The three-cell NiMH packs feed Pico `VSYS` and ground, never the 3.3 V pin.
Freshly charged packs must remain below the Pico W's 5.5 V `VSYS` maximum.
Do not use ordinary USB power banks: many shut themselves off at Pico-sized
loads. Do not charge cells while they are installed in the project.

## Prototype passives and interconnect

| Qty | Buy | Purpose |
|---:|---|---|
| 4 | Full-size solderless breadboard | One per gun/hub during parallel prototyping |
| 2 packs | 20 cm Dupont jumper wires | One male–male pack and one male–female pack, at least 40 wires each |
| 1 kit | 1/4 W through-hole metal-film resistor kit | Must contain 100 Ω, 1 kΩ, 4.7 kΩ, and 10 kΩ values |
| 10 | 15 Ω, 1 W resistor | IR LED pulse-current limiting; two used and eight experiment/spares |
| 1 kit | Ceramic capacitor assortment | Must contain at least twelve 100 nF, 25 V or higher parts |
| 1 kit | Electrolytic capacitor assortment | Must contain at least twelve 4.7 µF and four 100 µF, 10 V or higher parts |
| 10 | 3-pin JST-XH cable pair | One plug/receptacle pair per receiver pod plus two spares |
| 4 | 4-pin JST-XH cable pair | Detachable OLED/controls and spares |
| 10 m | 26 AWG stranded hookup wire | Flexible silicone insulation, mixed colors |
| 5 m | Three-conductor flexible cable | 26–28 AWG for receiver zone power, ground, and signal |
| 2 m | 22 AWG solid-core wire | Breadboard links and short power runs |
| 1 box | Heat-shrink assortment | 2:1 ratio, approximately 1.5–10 mm sizes |
| 1 bag | Small cable ties | Approximately 100 mm length |
| 1 roll | Fabric wiring-harness tape | Wearable cable bundling; avoid rigid bare adhesive near skin |

The resistor kit values are for GPIO/base bias and pull-ups/pull-downs. The
separate 15 Ω, 1 W parts are the starting emitter resistors, not a final value:
measure LED pulse current before range testing and change the value if needed.

## Permanent prototype assembly

Buy these after the breadboard IR link passes its electrical tests, but include
them in the budget now.

| Qty | Buy | Required specification |
|---:|---|---|
| 6 | Through-hole prototyping board | Approximately 70×90 mm, plated holes; four used and two spares |
| 10 | 2×20 female header strip | 2.54 mm pitch, breakable; sockets for removable Pico WH boards |
| 1 kit | JST-XH crimp housing/contact kit | 2-, 3-, and 4-pin housings with matching contacts |
| 1 | JST-compatible crimp tool | Engineer **PA-09** or ratcheting tool specified for the purchased contacts |
| 1 box | Nylon M2/M2.5 standoff kit | Electrically isolated board mounting |
| 1 roll | Hook-and-loop strap | 20–25 mm wide, for temporary wearable mounting |
| 1 m | 25 mm elastic webbing | Prototype shoulder/torso receiver harness |
| 8 | Small reusable hook-and-loop pads | Attach four receiver pods per player |
| 4 | Small ABS project box | Temporary enclosures for two hubs and two gun electronics assemblies |

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
| 1 | USB data cable | USB-A to Micro-USB, known to carry data, for Pico W programming |
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
- wearable hub and battery boxes;
- cable clips and strain-relief pieces.

For the first enclosure iteration, order PLA/PETG parts from a print service or
use the temporary ABS project boxes. Printer, material, fastener, and finishing
recommendations will be added after CAD requirements exist.

## Scaling beyond two players

For every additional player, add:

- 2 Raspberry Pi Pico WH boards;
- 1 SSD1306 OLED;
- 1 TSAL6200 IR LED and 1 PN2222AG driver;
- 4 TSOP38438 receivers (or one per chosen body zone);
- 1 trigger microswitch and 2 control buttons;
- 2 power switches, 2 three-AA holders, and 6 AA NiMH cells;
- 4 three-pin receiver cable pairs;
- enough wire, perfboard, connectors, harness material, and enclosures for one
  gun and one receiver hub.

Server and router capacity must be load-tested before choosing a maximum `N`.
The architecture should not encode a fixed two-player limit.

## Before placing the order

1. Confirm mains voltage and plug types for the soldering station, server power
   supply, charger, and router.
2. Confirm that OLED listings explicitly support 3.3 V logic.
3. Confirm exact TSOP38438 and TSAL6200 part numbers; generic IR sensors often
   use different carrier frequencies or pin orders.
4. Download each semiconductor datasheet and verify pin orientation before
   wiring.
5. Buy batteries from an authorized seller; counterfeit NiMH cells are common.

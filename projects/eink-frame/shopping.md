# Shopping list: 13.3″ family e-ink frame

This list is **only** for the wall frame. Do not mix it with the
[laser-tag two-player prototype list](../../docs/shopping-list.md).

Pick **one** board path. List A is the original (standard Pico outline).
List B is the stand-in while the Plus 2 W is out of stock: same chip,
PSRAM, and 2.4 GHz radio, longer board, charger on the PCB.

A flat pouch cell hides behind an A4-sized panel (the Inky PCB is
297 × 210 mm). Prices move; links were checked on 12 September 2026.

## List A — Pico Plus 2 W (preferred if in stock)

| Done | Qty | Item | Exact product and shop | Purpose |
|:---:|---:|---|---|---|
| | 1 | Colour e-ink panel | [Pimoroni Inky Impression 13.3″ (2025 Edition, PIM774), The Pi Hut](https://thepihut.com/products/inky-impression-13-3-2025-edition) | 1600×1200 Spectra 6 glass. Image stays with the power off. |
| | 1 | Wi-Fi + PSRAM board | [Pimoroni Pico Plus 2 W, The Pi Hut](https://thepihut.com/products/pimoroni-pico-plus-2-w) | RP2350B, 8 MB PSRAM, 2.4 GHz Wi-Fi. A stock Pico 2 W has no PSRAM; Pico LiPo 2 has charging but **no Wi-Fi**. |
| | 1 | Pico-to-Pi adapter | [Hard Stuff Pico to Pi HAT (buy the soldered-header version), The Pi Hut](https://thepihut.com/products/pico-to-pi-hat) | Carries the Pico onto the Inky’s 40-pin Pi header. Meter-check SCLK/MOSI — some units swap clock and data. |
| | 1 | Flat LiPo | [Adafruit 3.7 V 2500 mAh pouch, The Pi Hut](https://thepihut.com/products/lithium-ion-polymer-battery-3-7v-2500mah) | Thin enough for a box frame. ~2–3 weeks at one update/hour. |
| | 1 | Bigger flat LiPo (optional) | [Adafruit 3.7 V 6600 mAh pack, Adafruit](https://www.adafruit.com/product/353) | Closer to 5–6 weeks. 18 mm thick — only if the frame is deep. |
| | 1 | USB-C LiPo charger | [Pimoroni LiPo Amigo **Pro**, Pimoroni](https://shop.pimoroni.com/products/lipo-amigo) | Charge the pouch, switch power, feed `VSYS` at 3.0–4.2 V. Do **not** use a 5 V boost pack that stays on all day. |
| | 1 | USB-C data cable | [USB-A to USB-C, The Pi Hut](https://thepihut.com/products/usb-a-to-usb-c-cable-black) | The Plus 2 W is USB-C, not Micro-USB. Charge-only cables will not flash firmware. |
| | 1 | Deep box frame | A4 / 30 × 21 cm landscape, **at least 20 mm internal depth** (IKEA SANNAHED / RIBBA box, or a custom A3 with an A4 window) | Hides the Pico, adapter, charger, and pouch behind the glass. The Inky PCB is exactly A4. |
| | 1 pack | M2 standoffs + screws | [M2 brass standoff kit, The Pi Hut](https://thepihut.com/products/brass-m2-standoff-kit) | Inky already ships some; extras keep the pouch off the PCB. |
| | 1 | JST-PH 2-pin pigtail | [JST-PH battery extension, The Pi Hut](https://thepihut.com/products/jst-ph-2-pin-cable) | LiPo Amigo Pro → Pico `VSYS` / `GND` if you do not solder to the adapter. |

Expected electronics cost is roughly **£180–220 before the frame**, dominated by the 13.3″ panel.

The drawings in [`wiring.svg`](wiring.svg) and [`connections.svg`](connections.svg)
show this stack.

## List B — Pico LiPo 2 XL W (Plus 2 W out of stock)

Same RP2350B, 8 MB PSRAM, and RM2 Wi-Fi. The board is **24 mm longer**
than a Pico (~77 mm). The Pico-to-Pi HAT only covers the **USB-end**
20 pins. Onboard LiPo charging replaces the Amigo Pro.

Do **not** buy the non-XL [Pico LiPo 2](https://shop.pimoroni.com/products/pimoroni-pico-lipo-2)
(no Wi-Fi).

| Done | Qty | Item | Exact product and shop | Purpose |
|:---:|---:|---|---|---|
| | 1 | Colour e-ink panel | [Pimoroni Inky Impression 13.3″ (2025 Edition, PIM774), The Pi Hut](https://thepihut.com/products/inky-impression-13-3-2025-edition) | Same panel as list A. |
| | 1 | Wi-Fi + PSRAM + charger | [Pimoroni Pico LiPo 2 XL W (PIM776), The Pi Hut](https://thepihut.com/products/pimoroni-pico-lipo-2-xl-w) | RP2350B, 8 MB PSRAM, 2.4 GHz Wi-Fi, JST-PH + MCP73831 charger. Headers are **not** in the box. [Pimoroni page](https://shop.pimoroni.com/products/pimoroni-pico-lipo-2-xl-w?variant=55447911006587) if Pi Hut is empty. |
| | 1 | Male headers for the XL W | [Male Header Set for Raspberry Pi Pico, The Pi Hut](https://thepihut.com/products/male-headers-for-raspberry-pi-pico) | **This is the header pack to buy.** Two 1×20 male strips (2.54 mm) plus a 3-pin. Solder the two 20-pin strips on the **USB-end** holes only. See below. |
| | 1 | Pico-to-Pi adapter | [Hard Stuff Pico to Pi HAT **H** (soldered female headers), The Pi Hut](https://thepihut.com/products/pico-to-pi-hat) | Must be the **H** version (female sockets + 40-pin spacer), not **X** (bare PCB). Seat the XL W toward the USB end. Meter-check SCLK/MOSI. |
| | 1 | Flat LiPo | [Adafruit 3.7 V 2500 mAh pouch, The Pi Hut](https://thepihut.com/products/lithium-ion-polymer-battery-3-7v-2500mah) | Plugs straight into the XL W JST-PH. Charge is only 215 mA (overnight). |
| | 1 | Bigger flat LiPo (optional) | [Adafruit 3.7 V 6600 mAh pack, Adafruit](https://www.adafruit.com/product/353) | Same as list A. 18 mm thick. Still charges on the XL W; just slower. |
| | 1 | USB-C data cable | [USB-A to USB-C, The Pi Hut](https://thepihut.com/products/usb-a-to-usb-c-cable-black) | Flash **and** charge. Charge-only cables will not flash. |
| | 1 | Deep box frame | A4 / 30 × 21 cm landscape, **at least 20 mm internal depth** (IKEA SANNAHED / RIBBA box, or a custom A3 with an A4 window) | Same as list A. No Amigo in the stack, but the XL W is 24 mm longer. |
| | 1 pack | M2 standoffs + screws | [M2 brass standoff kit, The Pi Hut](https://thepihut.com/products/brass-m2-standoff-kit) | Same as list A. |
| | 1 | JST-PH 2-pin extension (optional) | [JST-PH battery extension, The Pi Hut](https://thepihut.com/products/jst-ph-2-pin-cable) | Only if the pouch lead will not lie flat. Not required for power — the XL W already has the socket. |

Skip the LiPo Amigo Pro on this path. Expected electronics cost is still
about **£180–220 before the frame** (the XL W is cheaper than Plus 2 W +
Amigo; the panel dominates).

### Headers — what to solder

The XL W has **two rows of 30 holes**. The HAT socket is a standard Pico
**2×20**. You only populate the 20 holes at the **USB-C end**.

1. Buy the [Male Header Set for Raspberry Pi Pico](https://thepihut.com/products/male-headers-for-raspberry-pi-pico) (£1). That is two **1×20 male** strips and a spare 3-pin. Pitch is 0.1″ / 2.54 mm. These are the same legs as a Pico H.
2. Solder the two 20-pin strips into the USB-end holes. Plastic collar on the **button / JST / Qw/ST** face. Bare pins exit the **back** (radio-module side) so they plug down into the HAT H sockets.
3. Leave the extra ten holes (antenna end) empty. The HAT does not reach them.
4. The 3-pin in the pack is for the debug header. You can ignore it.

Do **not** buy these instead:

- **Stacking** Pico headers (male-female, 11 mm) — extra height, fiddly, not what the HAT wants.
- **Short-plug** Pico headers — may not reach the HAT H sockets.
- Pimoroni **20 mm “long” breakaways** — meant for socket-to-socket; they stick through a thin frame.
- A **2×20 Pi GPIO** header — wrong shape; the XL W is two single rows.

If you later want every XL W hole filled, snap two **1×30** from a
[36-pin breakaway 10-pack](https://thepihut.com/products/break-away-0-1-36-pin-strip-male-header-black-10-pack).
That is optional. The HAT still only mates with the USB-end 20.

### After the XL W parts arrive

1. Confirm JST red = `+` on both the pouch and the board print. Never force a reversed plug.
2. Seat the HAT on the USB-end 20 pins. The extra 24 mm of board hangs off the antenna end.
3. If the radio module fouls the HAT PCB, use the included 40-pin spacer / taller stand-offs before you force it.
4. For weeks of sleep, cut the rear **power-LED** trace (LED symbol, USB-C end).
5. If a refresh browns out or the board dies mid-update, solder the rear **`+1A Mode`** jumper (only if the cell can deliver that).
6. Stack: frame glass → Inky → Pico-to-Pi HAT H → XL W (USB end) → pouch in the JST.
7. Run the server simulator in [`server/`](server/) and lock the HTML layout **before** writing Pico firmware.

## Already required but not purchased here

- a **2.4 GHz** Wi-Fi network (the Pico cannot join 5 GHz-only access points);
- a computer that can run the Rust server (this repo) and Chromium, to turn HTML into a 1600×1200 PNG;
- an Apple Account with two-factor authentication if you want the Family iCloud calendar (app-specific password from [account.apple.com](https://account.apple.com));
- a non-flammable surface for first LiPo charges;
- a soldering iron and 60/40 or SAC solder **if you chose list B** (the XL W ships without pins).

## Do not buy

- Raspberry Pi Zero / Zero 2 W — days of battery life, not weeks, unless you hard-cut 5 V every hour.
- Waveshare 13.3″ **IT8951** HAT — the controller board draws 20–100 mA while “idle”.
- A USB power bank — many shut off at Pico sleep current; the 5 V boost wastes energy all day.
- Pico 2 W (no Plus / no PSRAM) — the packed frame is 960000 bytes; SRAM is 520 KB.
- Pimoroni Pico LiPo 2 (**not** XL W) — charging, no Wi-Fi.
- LiPo Amigo Pro if you chose list B — the XL W already charges on USB-C.
- Stacking or short-plug Pico headers on list B — buy the standard male Pico set above.
- The laser-tag PiCowbell / 500 mAh cells / OLEDs — wrong shape and capacity for a wall frame.

## Safety

- Confirm JST red = `+` before plugging in. Never force a reversed connector.
- Charge where you can see the cell. Discard a swollen, bent, punctured, or hot pack.
- The pouch sits **flat**, padded, and cannot be pinched by the frame back.
- Inky refresh can pull a short spike; the LiPo handles that. Do not power the panel from the Pico `3V3` pin through thin jumper wire if you later abandon the HAT — use the 40-pin header.

## After the parts arrive (list A)

1. Open [`wiring.svg`](wiring.svg) and [`connections.svg`](connections.svg).
2. Stack: frame glass → Inky → Pico-to-Pi HAT → Pico Plus 2 W → LiPo Amigo Pro → pouch.
3. Run the server simulator in [`server/`](server/) and lock the HTML layout **before** writing Pico firmware.

List B assembly is under **After the XL W parts arrive** above.

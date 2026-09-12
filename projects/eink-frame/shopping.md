# Shopping list: 13.3″ family e-ink frame

This list is **only** for the wall frame. Do not mix it with the
[laser-tag two-player prototype list](../../docs/shopping-list.md).

The Pico Plus 2 W sleeps between hourly Wi-Fi fetches. A flat pouch cell
hides behind an A4-sized panel (the Inky PCB is 297 × 210 mm). Prices
move; links were checked on 12 September 2026.

## Buy this list

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

## Already required but not purchased here

- a **2.4 GHz** Wi-Fi network (the Pico cannot join 5 GHz-only access points);
- a computer that can run the Rust server (this repo) and Chromium, to turn HTML into a 1600×1200 PNG;
- an Apple Account with two-factor authentication if you want the Family iCloud calendar (app-specific password from [account.apple.com](https://account.apple.com));
- a non-flammable surface for first LiPo charges.

## Do not buy

- Raspberry Pi Zero / Zero 2 W — days of battery life, not weeks, unless you hard-cut 5 V every hour.
- Waveshare 13.3″ **IT8951** HAT — the controller board draws 20–100 mA while “idle”.
- A USB power bank — many shut off at Pico sleep current; the 5 V boost wastes energy all day.
- Pico 2 W (no Plus) if you want on-device JPEG later; this project’s server packs the frame, but the verified Inky bring-up used Plus 2 W PSRAM.
- The laser-tag PiCowbell / 500 mAh cells / OLEDs — wrong shape and capacity for a wall frame.

## Safety

- Confirm JST red = `+` before plugging in. Never force a reversed connector.
- Charge where you can see the cell. Discard a swollen, bent, punctured, or hot pack.
- The pouch sits **flat**, padded, and cannot be pinched by the frame back.
- Inky refresh can pull a short spike; the LiPo handles that. Do not power the panel from the Pico `3V3` pin through thin jumper wire if you later abandon the HAT — use the 40-pin header.

## After the parts arrive

1. Open [`wiring.svg`](wiring.svg) and [`connections.svg`](connections.svg).
2. Stack: frame glass → Inky → Pico-to-Pi HAT → Pico Plus 2 W → LiPo Amigo Pro → pouch.
3. Run the server simulator in [`server/`](server/) and lock the HTML layout **before** writing Pico firmware.

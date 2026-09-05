# Shopping list: working two-player prototype

This is the complete hardware list for a functional, rechargeable, bench-top
prototype with:

- one Raspberry Pi Pico W per player;
- one IR transmitter and four independent IR receiver zones per player;
- one OLED and three controls per player;
- Wi-Fi communication with a web server running on an existing computer;
- solderless construction on two breadboards.

The prototype is not yet wearable or impact-resistant. Enclosures, body wiring,
sound, permanent connectors, dedicated server hardware, and fabrication tools
are deliberately deferred until the core design works.

Prices and stock change frequently. Links and approximate prices were checked
on 5 September 2026. Rapid prices may be displayed without VAT.

## Buy this list

| Done | Qty | Item | Exact product and UK shop | Purpose |
|:---:|---:|---|---|---|
| ✅ | 2 | Wi-Fi microcontroller | [Raspberry Pi Pico WH — select “Pico WH” (£6.70 each), The Pi Hut](https://thepihut.com/products/raspberry-pi-pico-w) | Official Pico W with headers already fitted; one per player |
| ✅ | 2 | Rechargeable power/prototyping board | [Adafruit Proto Doubler PiCowbell for Pico and Pico W (£7.30 each), The Pi Hut](https://thepihut.com/products/adafruit-proto-doubler-picowbell-for-pico-and-pico-w) | No-solder Pico socket, LiPo charger, power switch, and accessible GPIO |
| ✅ | 2 | Rechargeable battery | [500 mAh 3.7 V LiPo with JST-PH connector (£6 each), The Pi Hut](https://thepihut.com/products/500mah-3-7v-lipo-battery) | One per player; plugs directly into the PiCowbell |
| ✅ | 2 | Player display | [0.96-inch 128×64 I²C SSD1306 OLED (£4 each), The Pi Hut](https://thepihut.com/products/0-96-oled-display-module-128x64) | Health, ammunition, score, and connection status |
| ✅ | 8 | IR receiver | [Vishay TSOP38238 38 kHz receiver (£1 each), The Pi Hut](https://thepihut.com/products/ir-infrared-receiver-tsop38238) | Four separately wired hit zones per player |
| ✅ | 10 | IR emitter | [Vishay TSAL6200 940 nm LED, Rapid code 49-4513](https://www.rapidonline.com/vishay-tsal6200-5mm-940nm-ir-transmitter-diode-49-4513) | Two used; Rapid's price break begins at ten |
| ✅ | 50 | IR-driver transistor | [Diotec BC337-40, Rapid code 50-3113](https://www.rapidonline.com/bc337-40-diotec-bipolar-npn-transistor-45v-50-3113) | Two used; Rapid's minimum order is fifty |
| ✅ | 1 pack | 100 Ω resistors | [Royal Ohm CFR0W4J0101KIT, pack of 100, Rapid code 62-0346](https://www.rapidonline.com/royal-ohm-cfr0w4j0101kit-100r-carbon-film-resistor-0-25w-pack-of-100-62-0346) | One supply-filter resistor per IR receiver |
| ✅ | 1 pack | 220 Ω resistors | [Royal Ohm CFR0W4J0221KIT, pack of 100, Rapid code 62-0354](https://www.rapidonline.com/royal-ohm-cfr0w4j0221kit-220r-carbon-film-resistor-0-25w-pack-of-100-62-0354) | Two in series per BC337 base |
| ✅ | 1 pack | 10 kΩ resistors | [Royal Ohm CFR0W4J0103KIT, pack of 100, Rapid code 62-0394](https://www.rapidonline.com/royal-ohm-cfr0w4j0103kit-10k-carbon-film-resistor-0-25w-pack-of-100-62-0394) | Transistor pull-downs and button inputs |
| ✅ | 10 | 15 Ω, 1 W resistors | [Royal Ohm MFF1WFF0150A10, Rapid code 62-8705](https://www.rapidonline.com/royal-ohm-mff1wff0150a10-15r-1-1w-flame-proof-metal-film-resistor-62-8705) | Two in series per IR LED give a safe 30 Ω starting value |
| ✅ | 10 | 100 nF ceramic capacitors | [Suntan TS170R2A104KSBBA0R, Rapid code 11-3442](https://www.rapidonline.com/suntan-ts170r2a104ksbba0r-0-1uf-10-100v-x7r-2-54mm-radial-ceramic-capacitor-11-3442) | Receiver filtering and transmitter decoupling |
| ✅ | 10 | 4.7 µF electrolytic capacitors | [JB Capacitors JRK1H4R7M015000400070000B, Rapid code 11-4117](https://www.rapidonline.com/jb-capacitors-jrk1h4r7m015000400070000b-4-7uf-50v-20-4x7mm-mini-radial-alum-cap-11-4117) | Receiver filtering and transmitter decoupling; observe polarity |
| ✅ | 10 | Tactile push buttons | [Diptronics DTS-644K, Rapid code 78-1150](https://www.rapidonline.com/diptronics-dts-644k-square-button-through-hole-6-x-6mm-tactile-switch-100gf-78-1150) | Trigger, reload, and menu controls; six used and four spares |
| ✅ | 2 | Full-size breadboards | [830-point full-size breadboard (£5 each), The Pi Hut](https://thepihut.com/products/full-sized-breadboard) | One complete player circuit per breadboard |
| ✅ | 1 box | Breadboard jumper wires | [140-piece 22 AWG solid jumper kit (£4), The Pi Hut](https://thepihut.com/products/jumper-wire-kit-140-piece) | Connects PiCowbell GPIO, receivers, display, buttons, and IR driver |
| ✅ | 2 | Micro-USB data/charging cables | [Official Raspberry Pi Micro-USB cable (£1.20 each), The Pi Hut](https://thepihut.com/products/raspberry-pi-micro-usb-cable) | Programming and charging; one cable per player |

Expected component cost is roughly **£90 before delivery**, based on the
listed prices and typical Rapid pack pricing. No soldering iron is required for
this breadboard build.

## Already required but not purchased here

- a computer with a USB-A port; for a USB-C-only computer, buy two
  [USB-C to Micro-USB data cables](https://thepihut.com/products/usb-c-to-micro-usb-cable-black)
  instead of the USB-A cables listed above;
- a private 2.4 GHz Wi-Fi network or phone hotspot;
- a modern web browser;
- a safe, non-flammable surface on which to test and charge the LiPo batteries.

The game server runs on the computer during prototyping, so a Raspberry Pi
server and dedicated router are not required yet.

## Important assembly notes

- Select **Pico WH**, not the headerless Pico W, so it plugs into the PiCowbell
  without soldering.
- Confirm each LiPo's red wire aligns with `+` on the PiCowbell before plugging
  it in. Never force or reverse a JST connector.
- Charge batteries where they can be observed. Do not use a battery that is
  swollen, bent, punctured, hot, or otherwise damaged.
- Power the prototype IR LED from regulated `3V3 OUT` through two series 15 Ω
  resistors and the BC337-40. Never drive it directly from a GPIO. Do not use
  `VSYS` here: its voltage rises while USB is connected and would increase LED
  current.
- Verify BC337-40 collector/base/emitter order from its datasheet; it differs
  from many 2N2222-family parts.
- The two series 15 Ω emitter resistors form a conservative 30 Ω starting
  value. Measure pulse current before attempting maximum range or reducing it.
- Electrolytic capacitors are polarized. Ceramic capacitors are not.
- Use infrared LEDs only. Never substitute a laser diode.

## Explicitly deferred

Do not buy these until the breadboard game works:

- wearable receiver cables and harnesses;
- trigger microswitches and panel-mounted controls;
- soldering equipment, perfboard, JST connectors, and hookup wire;
- enclosures and 3D-printed parts;
- buzzers, vibration motors, decorative lights, and optics;
- dedicated Raspberry Pi server or travel router;
- larger batteries.

The 500 mAh cells are intended to prove rechargeable operation, not establish
final match duration. Battery capacity will be selected after measuring the
complete player's average and peak current.

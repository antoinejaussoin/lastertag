# Hardware

This directory is reserved for version-controlled hardware design artifacts.

## Planned structure

- `electronics/` — schematics, pin maps, power budget, PCB sources, and BOMs
- `3d-printing/` — CAD sources, exported meshes, drawings, and print notes
- `test/` — range, current, thermal, battery, and environmental test results

Only source/design files should be authoritative. Exported manufacturing files
must record the source revision, units, tolerances, and generation settings.

The first electrical artifact should be a reviewed breadboard schematic for:

- Pico W power from three NiMH AA cells through `VSYS`;
- transistor-driven 940 nm IR emitter;
- four separately addressable 38 kHz receiver inputs;
- 3.3 V I²C OLED and buttons;
- decoupling, connectors, switches, and test points.

Do not connect a laser diode. Do not drive an emitter LED from a GPIO pin.

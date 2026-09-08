# Lastertag

An open, custom-built laser-tag system based on the Raspberry Pi Pico W.

> Despite the name, this project uses eye-safe infrared LEDs rather than laser
> diodes. Never substitute a laser module for the infrared emitter.

The initial reference build supports two players and is designed to scale to
`N` players. Each player has:

- one Pico W controlling the gun, display, and all body sensors;
- four wired, body-mounted infrared receiver zones.

A local web application coordinates games, displays live statistics, and
persists match history.

## Repository layout

- [`temp-oled-experiment/`](temp-oled-experiment/) — first hardware sandbox: Pico 2 temperature on the OLED
- [`source/`](source/) — embedded firmware, shared protocol code, and web app
- [`docs/`](docs/) — architecture, project plan, and purchasing guide
- [`hardware/`](hardware/) — electronics notes and future 3D-printable parts

Start with the [high-level plan](docs/high-level-plan.md), then use the
[shopping list](docs/shopping-list.md) for the two-player prototype. The
[Pico W capabilities guide](docs/pico-w-capabilities.md) explains what runs on
each player controller, its memory, storage, networking, and limitations.

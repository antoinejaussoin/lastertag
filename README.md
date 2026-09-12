# Lastertag

An open, custom-built laser-tag system based on the Raspberry Pi Pico 2 W.

> Despite the name, this project uses eye-safe infrared LEDs rather than laser
> diodes. Never substitute a laser module for the infrared emitter.

The initial reference build supports two players and is designed to scale to
`N` players. Each player has:

- one Pico 2 W controlling the gun, display, and all body sensors;
- four wired, body-mounted infrared receiver zones.

A local web application coordinates games, displays live statistics, and
persists match history.

## Repository layout

- [`projects/lasertag/source/`](projects/lasertag/source/) — embedded firmware, shared protocol code, and web app
- [`projects/lasertag/hardware/`](projects/lasertag/hardware/) — electronics notes and future 3D-printable parts
- [`projects/temperature-display/`](projects/temperature-display/) — first hardware sandbox: Pico 2 W temperature on the OLED
- [`projects/ir-capture/`](projects/ir-capture/) — TSOP38238 sandbox: TV-remote capture on the OLED and HTTP POST
- [`projects/eink-frame/`](projects/eink-frame/) — 13.3″ family wall frame: Pico Plus 2 W + Rust HTML/CSS server (own shopping list)
- [`docs/`](docs/) — architecture, project plan, and purchasing guide

Start with the [high-level plan](docs/high-level-plan.md), then use the
[shopping list](docs/shopping-list.md) for the two-player prototype. The
[Pico 2 W capabilities guide](docs/pico-w-capabilities.md) explains what runs
on each player controller, its memory, storage, networking, and limitations.

# Source code

This directory will contain all executable project code.

## Intended structure

- `firmware/gun/` — trigger input, IR transmission, screen UI, and networking
- `firmware/receiver/` — body-zone receivers, hit validation, and networking
- `shared/` — shot packet format, game events, IDs, and common test vectors
- `web/` — game server, browser UI, API, and persistent storage

The embedded targets are Raspberry Pi Pico W boards. The first implementation
decision will be whether to use the Pico SDK (C/C++) or MicroPython; the
milestones in the documentation keep the protocol independent of that choice.

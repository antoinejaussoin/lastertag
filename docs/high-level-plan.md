# High-level project plan

## 1. Goal and first playable version

Build a reusable laser-tag platform for `N` players, starting with a complete
two-player prototype. The game must continue to register shots on a local
network without internet access.

The first playable version consists of:

- two handheld guns;
- one Raspberry Pi Pico W controller per player;
- four receiver zones per player (front, back, left shoulder, right shoulder);
- a 128×64 OLED on each gun showing health, ammunition, score, and status;
- a local server with an operator dashboard, live scoreboard, and persistent
  match history;
- placeholders for printable gun shells, receiver pods, and wearable mounts.

The word “laser” is conventional: shots use modulated 940 nm infrared LEDs.
Laser diodes are unnecessary and create avoidable eye-safety risk.

## 2. System architecture

### Per-player hardware

Each player has one Raspberry Pi Pico W. It:

- reads trigger, reload, and menu buttons;
- emits a short encoded shot through a 940 nm IR LED at a 38 kHz carrier;
- connects to at least four separately wired TSOP38238 receiver modules;
- decodes shots, rejects its own player ID, and records the body zone;
- controls the OLED;
- exchanges hit events and authoritative game state with the server over Wi-Fi;
- provides future connection points for vibration, light, or sound feedback.

Each receiver has a dedicated GPIO, preserving zone information while avoiding
a microcontroller in every sensor pod. Receiver and control cables converge on
the Pico W, which can be mounted in the gun or in a wearable enclosure. Add
zones subject to GPIO, cabling, power, and interrupt-handling limits.

### Game server

The server runs on a Raspberry Pi or development computer and is authoritative
for match state. It provides:

- an HTTP API for setup and history;
- a WebSocket or server-sent-event stream for live updates;
- device presence/health monitoring;
- scoring and game-rule enforcement;
- an SQLite database;
- a responsive browser interface for phones, tablets, and computers.

The preferred first network topology is an ordinary private Wi-Fi network.
Later, the server can provide a dedicated access point for portable play. Core
gameplay must not depend on cloud services.

### Event flow

1. The operator assigns stable device IDs to players and starts a match.
2. The server distributes the match configuration to each player controller.
3. A gun checks local fire-rate/ammunition rules and emits an IR packet.
4. One or more body sensors receive it; the target Pico validates and
   deduplicates it.
5. The target Pico submits
   `shot ID + shooter + victim + zone + local sequence/time`.
6. The server applies authoritative rules, persists the event, and broadcasts
   updated state.
7. The victim and shooter screens are refreshed.

Devices should use monotonically increasing sequence numbers and bounded
queues. This makes retries idempotent and tolerates short Wi-Fi interruptions.

## 3. Protocol outline

### Infrared

Start with a compact binary packet containing:

- protocol version;
- shooter/device ID;
- team ID;
- shot sequence number;
- weapon/damage type;
- checksum or CRC.

Use a 38 kHz carrier compatible with the TSOP38238. Define timings and publish
test vectors before optimizing range. A packet must be validated completely
before it becomes a hit. The player controller must collapse the same packet seen by
adjacent body zones into one hit while preserving the strongest/first zone.

### Network

Use versioned JSON during development for observability. Define messages for
registration, heartbeat, configuration, shot/hit event, acknowledgement, state
snapshot, and error. Move to a compact binary encoding only if measurement
shows it is needed.

Security for the prototype: private Wi-Fi, unique device credentials, server
validation of IDs and match membership, and no unauthenticated admin controls.
Internet exposure requires TLS, real user authentication, secret management,
updates, and a separate threat review.

## 4. Hardware design checkpoints

Before soldering permanent assemblies:

- measure IR LED current and transistor switching on a breadboard;
- verify reliable decoding at the desired indoor range and angles;
- test receiver behavior in sunlight and under LED/fluorescent lighting;
- verify that simultaneous receiver-zone signals do not overwhelm firmware;
- measure peak and average current for each node;
- run a full battery-duration test;
- verify USB charging, low-voltage cutoff, connector polarity, and enclosure
  protection for the LiPo battery;
- confirm that Wi-Fi loss cannot create extra hits or corrupt match history.

The IR emitter requires a transistor driver and current-limiting resistor.
Never power it directly from a Pico GPIO. Every receiver should have local
decoupling close to its supply pins. Add reverse-polarity and strain protection
before moving from bench prototypes to wearable hardware.

## 5. Software decomposition

### Firmware

- hardware abstraction for pins, buttons, OLED, and IR;
- deterministic shot encoder and decoder;
- local gun/player state machine;
- Wi-Fi provisioning, reconnect, heartbeat, and event queue;
- over-the-air or USB update path;
- serial diagnostics and self-test.

### Server and web UI

- device registry and player/team assignment;
- configurable game modes and rule engine;
- match lifecycle and authoritative event processor;
- real-time operator dashboard and public scoreboard;
- SQLite schema and migrations;
- match/player history, summaries, export, and backup;
- simulator for developing without complete hardware.

### Testing

- shared protocol vectors used by encoder and decoder tests;
- simulated devices and deterministic match scenarios;
- firmware tests for packet validation and deduplication;
- API, database migration, and scoring tests;
- hardware-in-the-loop range, ambient-light, latency, and endurance tests.

## 6. Delivery milestones

### Milestone 0 — decisions and bench setup

- choose Pico SDK/C++ or MicroPython after a timing proof of concept;
- assign pin maps and stable IDs;
- write protocol version 1 and electrical schematics;
- inventory and smoke-test purchased parts.

Exit: both Pico W boards can connect to Wi-Fi, display status, and exchange a
test event with a development server.

### Milestone 1 — one-way IR link

- build one transistor-driven emitter and one receiver;
- encode/decode IDs with checksum;
- characterize indoor range and false-positive rate.

Exit: at least 100 consecutive valid shots at target range with no duplicate
hits and no unsafe component temperature/current.

### Milestone 2 — one complete player unit

- finish gun controls and display;
- connect four wearable zones to the same Pico W;
- implement zone deduplication and local feedback;
- test battery life and disconnect handling.

Exit: the complete unit transmits valid packets and detects test packets on all
four body zones.

### Milestone 3 — two-player game

- assemble the second one-Pico player unit;
- implement match lifecycle, score rules, and live dashboard;
- persist complete match history in SQLite;
- test crossed shots, rapid fire, retries, and reboots.

Exit: repeatable two-player matches with correct totals after all devices and
the server reconnect.

### Milestone 4 — enclosures and field hardening

- design printable gun, screen, receiver-pod, controller, and battery enclosures;
- add connectors, cable strain relief, battery padding, and protected access to
  the power button and USB charging port;
- perform drop, snag, heat, endurance, and eye-safety reviews.

Exit: wearable prototypes survive normal play without exposed conductors,
sharp edges, loose batteries, or pointing hazards.

### Milestone 5 — scale and deploy

- load-test simulated `N`-player games;
- add portable access-point deployment and automated server backups;
- document assembly, provisioning, calibration, and repair;
- consider optional internet synchronization only after local operation is
  reliable.

## 7. Decisions intentionally deferred

- firmware language and RTOS choice;
- final gun optics and practical maximum range;
- final battery capacity and any future custom charging PCB;
- PCB design;
- enclosure ergonomics and printer/material choice;
- exact game modes, sound, lighting, haptics, and cloud hosting.

These are deferred until the breadboard measurements establish timing, current,
thermal, range, and usability requirements.

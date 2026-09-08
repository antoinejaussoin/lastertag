# What the Raspberry Pi Pico 2 W can do

This project uses the **Raspberry Pi Pico 2 W**. It is the wireless Pico 2:
an RP2350 microcontroller with an Infineon CYW43439 for 2.4 GHz Wi-Fi and
Bluetooth. Header pins must be fitted (or bought pre-soldered) to plug into
the PiCowbell. A Pico 2 without the W has no radio.

The most important distinction is:

> A Pico 2 W is a microcontroller, not a small Linux computer.

It is designed to start one embedded program quickly, interact directly with
buttons, sensors and LEDs, and keep doing that job predictably. A Raspberry Pi
Zero, 4 or 5 is a general-purpose computer designed to run Linux, processes,
desktop applications and large files.

## Short answer

The Pico 2 W:

- does **not** practically run Raspberry Pi OS or ordinary Linux;
- runs a firmware program directly on its RP2350 microcontroller;
- has two Arm Cortex-M33 cores (or two RISC-V Hazard3 cores), typically at
  up to 150 MHz; this project uses the Arm cores;
- has 520 KiB of volatile working memory (SRAM);
- has 4 MiB of persistent flash shared by firmware and saved data;
- has 2.4 GHz Wi-Fi and Bluetooth 5.2;
- can read buttons and sensors and precisely control outputs;
- can run C/C++, MicroPython, Arduino-style code, Rust, or a small RTOS;
- cannot replace the game server, database, or rich web dashboard.

That is ample for one laser-tag player controller.

## Does it run Linux?

Not in the normal or useful sense.

The RP2350 has only 520 KiB of RAM and no memory-management unit (MMU) of the
kind a desktop Linux kernel expects. It has no built-in disk, display
controller, HDMI, Ethernet, keyboard interface or large external RAM. A normal
Linux system expects vastly more memory and usually an MMU for virtual memory
and process isolation.

Experimental Unix-like systems or emulators can be made for very small
microcontrollers, but they are demonstrations rather than a sensible basis for
this project. They would consume resources while making real-time IR handling
less reliable.

The Pico can instead run:

- **bare-metal firmware:** one program directly controls the hardware;
- **MicroPython:** a compact Python interpreter and your Python scripts;
- **a real-time operating system:** for example FreeRTOS, providing tasks,
  queues and timers without becoming a desktop operating system;
- **a custom scheduler/event loop:** often enough for a project this size.

The web server and SQLite database belong on a laptop, desktop computer or
Linux-capable Raspberry Pi. The Pico 2 W communicates with that server over
Wi-Fi.

## What happens when it powers on?

1. Permanent code in the RP2350's internal boot ROM runs.
2. Normally it finds the firmware stored in the external flash chip.
3. The firmware initializes RAM, clocks and peripherals.
4. Its entry function starts and normally continues until power is removed,
   the watchdog resets it, or firmware requests a restart.

There is no login screen and no shell unless the firmware deliberately
provides one. There are no independent applications being launched by an
operating system. The loaded firmware owns the machine.

Holding `BOOTSEL` while connecting USB starts the ROM USB bootloader. The Pico
appears to the computer as a drive named `RP2350`; copying a UF2 firmware file
to it reprograms flash and reboots the board. The apparent USB drive is a
programming interface, not a normal 4 MiB disk.

With MicroPython installed, a computer can also open a USB serial REPL. The
REPL is an interactive Python prompt supplied by MicroPython, not Linux.

## Processor

The RP2350 contains:

- two Arm Cortex-M33 cores, normally clocked at up to 150 MHz, with a
  single-precision floating-point unit (or two Hazard3 RISC-V cores instead;
  this project uses Arm);
- hardware integer divider and interpolator blocks;
- 16 DMA channels for moving data without constant CPU involvement;
- interrupt, timer and watchdog hardware;
- Arm TrustZone, OTP, and optional signed boot;
- a 16 KiB cache in front of execute-in-place flash.

Both cores can run simultaneously, but using two cores requires explicit
coordination. They share RAM, flash and peripherals. Two cores do not turn the
board into a desktop computer and do not automatically make every program
twice as fast.

For this project, one core would already be sufficient. Hardware PWM or PIO
can produce the 38 kHz IR carrier while the CPU handles buttons, received
packets, the OLED and network events.

## Memory: RAM versus persistent flash

### 520 KiB SRAM: temporary working memory

SRAM holds values that change while the program runs:

- variables and object state;
- function call stacks;
- network packet buffers;
- the MicroPython heap, when MicroPython is used;
- queues of pending hits or messages;
- an OLED framebuffer;
- code copied to RAM for time-critical work.

SRAM loses all contents when power is removed or the Pico resets. It is shared
by both CPU cores, the Wi-Fi/network software and the application.

`520 KiB` is 532,480 bytes. The application cannot normally use all of it:
firmware runtime state, stacks, drivers and network buffers take a portion.
MicroPython itself consumes substantially more RAM than a small compiled C
program, and the exact free heap changes between firmware builds.

The 16 KiB flash cache is not additional general-purpose RAM. It automatically
speeds access to code and read-only data held in flash.

### 4 MiB QSPI flash: persistent memory

The Pico 2 W board has a separate 4 MiB flash chip. Its contents remain after
power is removed. It normally contains:

- the executable firmware or MicroPython interpreter;
- constants such as fonts, lookup tables and default configuration;
- MicroPython files, if using its filesystem;
- any explicitly reserved application-data area.

The RP2350 itself has no large internal user flash for application code. It
reads code from the board's external flash through QSPI and can execute that
code in place, known as XIP. A small OTP region exists for keys and factory
data; it is not general application storage.

`4 MiB` is 4,194,304 bytes, but not all of it is available for saved data.
Firmware occupies part of it. A MicroPython build also reserves space for the
interpreter and exposes the remaining portion as a small filesystem. Exact
free space depends on the specific firmware version and partition layout.

In a C/C++ program, there is no filesystem automatically. The program may:

- reserve fixed flash sectors for configuration;
- use a small embedded filesystem such as LittleFS;
- add an external EEPROM, flash chip or microSD card;
- send durable records to the game server instead.

### Flash is not RAM and not an unlimited-write disk

Flash can be read frequently, but changing it is more complicated:

- the common erase unit is a 4 KiB sector;
- programming is performed in aligned pages, commonly 256 bytes;
- an erase changes a whole sector back to its blank state;
- erase/program operations temporarily prevent ordinary XIP flash access;
- flash has finite erase endurance;
- power loss during an update can corrupt an incompletely written record.

Do not rewrite a score counter to the same flash sector after every shot.
Buffer events in RAM, write infrequently, use redundant records or wear
levelling, and let the server provide authoritative long-term persistence.

Appropriate Pico flash data for this project includes:

- device/player ID;
- Wi-Fi provisioning information;
- calibration and hardware revision;
- last acknowledged network sequence number;
- small batches of events awaiting server delivery;
- firmware settings that change rarely.

Complete match history and statistics belong in the server's SQLite database.

### Other storage-related facts

- There is no built-in EEPROM.
- There is no built-in SD-card slot.
- There is no battery-backed storage.
- The boot ROM is permanent factory-programmed code, not application storage.
- The wireless chip's internal memory is managed by its driver and is not
  general application RAM.
- External storage can be added over SPI or I2C if genuinely needed.

## What can fit in 4 MiB?

Exact sizes depend on language, libraries and compiler options, but these are
realistic categories.

Comfortable:

- the complete laser-tag player firmware;
- IR encoding/decoding and test vectors;
- OLED fonts, icons and menus;
- Wi-Fi networking and a compact application protocol;
- configuration and a bounded offline event queue;
- diagnostics and a USB command interface;
- a small HTTP status/configuration page.

Possible but increasingly constrained:

- MicroPython plus several application modules;
- TLS networking, depending on library configuration and certificate sizes;
- a larger set of fonts, sounds or compressed assets;
- over-the-air updates, especially if trying to keep both old and new firmware
  images at once;
- Bluetooth and Wi-Fi features together with large application buffers.

Poor fits:

- a Linux distribution;
- SQLite and a durable match-history database;
- a modern browser or JavaScript application runtime;
- images, music or video collections;
- large machine-learning models;
- a rich web dashboard with extensive static assets;
- unbounded logs.

As a scale example, the 128 by 64 monochrome OLED needs only 1,024 bytes for a
full one-bit framebuffer. A compact hit event might be tens of bytes. The
limitation is therefore not basic laser-tag state; it is large runtimes,
network buffers, rich media and durable bulk storage.

## Programming choices

### C/C++ with the official Pico SDK

The source code is compiled on a normal computer into a UF2 firmware image.
The Pico runs the resulting machine code directly.

Advantages:

- best control over timing and hardware;
- smallest and most predictable RAM usage;
- direct access to PWM, PIO, interrupts, DMA and both cores;
- suitable for production firmware.

Costs:

- longer build/debug cycle;
- memory ownership and concurrency require care;
- more opportunities for low-level bugs.

### MicroPython

The Pico first runs MicroPython firmware. Your `.py` files are then interpreted
on the board.

Advantages:

- simple, interactive development through the REPL;
- readable code and quick experiments;
- suitable for proving OLED, button, receiver and network behavior;
- hardware PWM and PIO remain accessible.

Costs:

- less free RAM and flash;
- slower general Python execution;
- garbage collection can introduce timing pauses;
- some low-level libraries and features differ from the C SDK.

Time-critical IR carrier generation should still use hardware PWM or PIO, not
a Python loop toggling a pin.

### Other options

- **Arduino-Pico:** familiar Arduino APIs on top of RP2350 support.
- **Rust:** strong compile-time safety with community RP2350 libraries
  (`embassy-rp` with the `rp235xa` feature for Pico 2 W).
- **CircuitPython:** beginner-friendly Python variant, but generally uses more
  flash/RAM and has different library trade-offs.
- **FreeRTOS:** task scheduling for C/C++ firmware when a deliberate RTOS
  architecture is useful.

Official Raspberry Pi support is strongest for the C/C++ SDK and MicroPython.

## Input and output hardware

The Pico 2 W exposes 26 multifunction 3.3 V GPIO pins:

- 23 are digital-only;
- three exposed pins can also be analogue inputs;
- digital pins can act as input, output, interrupt, PWM or peripheral signals;
- pin functions can include UART, SPI and I2C.

The RP2350 provides:

- two UART controllers;
- two SPI controllers;
- two I2C controllers;
- 24 PWM channels across twelve slices;
- a 12-bit ADC rated up to 500 ksps;
- one internal temperature-sensor ADC input;
- USB 1.1 device or host hardware;
- twelve PIO state machines across three PIO blocks.

PIO is a distinctive RP2350 feature: tiny deterministic state machines execute
short programs independently of the CPU. They can generate or receive precise
digital waveforms. This is useful for custom protocols and could handle the IR
carrier or packet timing, although ordinary PWM is sufficient for the initial
38 kHz transmitter.

### Electrical limits

GPIO is **3.3 V logic and is not 5 V tolerant**. A GPIO is a signal pin, not a
power supply:

- do not connect a 5 V output directly to it;
- do not power motors, buzzers or high-current LEDs from it;
- use a transistor or driver for the IR LED;
- connect all interacting circuits to a common ground;
- configure unused or boot-sensitive outputs to a safe state.

The current schematic follows these rules by using the BC337-40 transistor and
leaving the IR gate low until firmware deliberately sends a shot.

## Wi-Fi and Bluetooth

The Pico 2 W includes an Infineon CYW43439 radio connected to the RP2350:

- Wi-Fi 4 / IEEE 802.11b/g/n;
- single-band 2.4 GHz only;
- WPA3 capability;
- station mode for joining an existing network;
- SoftAP mode for creating a small access point, officially up to four clients;
- Bluetooth 5.2;
- Bluetooth Low Energy central and peripheral roles;
- Bluetooth Classic support.

The actual features available depend on the firmware, SDK and libraries being
used. Wi-Fi and Bluetooth consume RAM, flash and power, and radio traffic is
not deterministic.

The Pico 2 W can run TCP, UDP, DNS, DHCP, HTTP, WebSocket-like application
protocols and MQTT when suitable libraries are included. TLS is possible but
uses noticeably more memory and processing time.

For laser tag, each Pico should join a private 2.4 GHz network as a client.
Real-time hit detection remains local; Wi-Fi carries events and game state.
Gameplay should tolerate packets being delayed, duplicated or temporarily
lost.

Keep metal, batteries and dense wiring away from the onboard antenna end of
the Pico 2 W. Poor physical placement can reduce range.

## Timing and real-time behavior

A microcontroller is good at predictable hardware timing, but network traffic,
interpreters and flash writes can cause delays. The design should separate
responsibilities:

- PWM/PIO generates the exact 38 kHz carrier;
- GPIO interrupts capture receiver transitions;
- timers enforce packet timing and fire rate;
- the foreground loop updates the display and game state;
- networking runs asynchronously and uses queues;
- flash writes happen outside time-critical receive windows.

The watchdog can reset the firmware if it hangs. On restart, RAM is lost, so
important sequence/configuration data must be recoverable from flash or the
server.

## Clock and time

The RP2350 contains timer and real-time-clock hardware, but the Pico 2 W has no
battery-backed clock. It does not know civil date/time after complete power
loss unless it receives the time from a server or an external RTC.

For IR packets and local event ordering, use monotonic timers that measure time
since boot. Use the game server for authoritative wall-clock timestamps.

## Can it host a web server?

Yes, but only a small embedded one.

It can serve a compact setup or status page and handle a few connections. It
is not suited to storing match history, rendering a rich dashboard, performing
large database queries or serving many simultaneous users.

In this project:

- Pico 2 W: player hardware, immediate rules, display, IR and network client;
- external server: match authority, web UI, user access and SQLite history.

This division keeps the player responsive even when the server or Wi-Fi is
temporarily unavailable.

## Updates and recovery

USB UF2 installation is the simplest and most reliable update path during
development. It can always be reached through the ROM `BOOTSEL` mechanism even
if application firmware is broken.

Network/over-the-air updates are possible. 4 MiB leaves more room than the
older 2 MiB Pico W, but an updater must still authenticate the image, survive
power loss and avoid overwriting its currently executing flash code. OTA
should be added only after the basic firmware is stable.

SWD pads provide low-level programming and debugging with an external debug
probe. They are useful for difficult firmware faults but not required for the
initial prototype.

## Security limitations

RP2350 is stronger than RP2040 (TrustZone, optional signed boot, OTP key
storage), but a player tag is still a physically accessible device:

- application code and credentials usually reside in external flash;
- signed boot and flash protection are optional and must be designed in;
- physical debug/access can expose secrets if those features are unused;
- all firmware code shares one address space without desktop-style process
  isolation.

Use a private Wi-Fi network, per-device credentials, server-side validation and
short-lived/revocable secrets. Do not treat a secret stored on a physically
accessible player unit as impossible to extract.

## Recommended role in this project

One Pico 2 W can comfortably:

- read all four receiver zones independently;
- generate encoded IR shots;
- read trigger, reload and menu controls;
- maintain health, ammunition and short-term game state;
- draw the OLED interface;
- connect to Wi-Fi;
- queue events during a brief disconnection;
- store a small amount of configuration persistently;
- recover safely after resets.

It should not:

- own the only copy of match history;
- run the main SQLite database;
- host the full operator dashboard;
- depend on Wi-Fi for immediate hit detection;
- log every low-level transition permanently to flash;
- directly drive the high-current IR LED.

## Official references

- [Raspberry Pi Pico 2 W datasheet](https://datasheets.raspberrypi.com/picow/pico-2-w-datasheet.pdf)
- [RP2350 datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
- [Raspberry Pi microcontroller documentation](https://www.raspberrypi.com/documentation/microcontrollers/)
- [MicroPython on Pico-series boards](https://www.raspberrypi.com/documentation/microcontrollers/micropython.html)
- [Pico C/C++ SDK](https://www.raspberrypi.com/documentation/microcontrollers/c_sdk.html)
- [Pico SDK networking libraries](https://www.raspberrypi.com/documentation/pico-sdk/networking.html)


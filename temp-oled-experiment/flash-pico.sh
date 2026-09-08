#!/bin/sh
# Copy an RP2040 ELF onto a Pico WH in BOOTSEL mode (macOS volume RPI-RP2).
set -e

ELF="${1:?usage: flash-pico.sh <firmware.elf>}"
HERE="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
UF2="$HERE/temp-oled-experiment.uf2"
ELF2UF2="${ELF2UF2:-$HOME/.cargo/bin/elf2uf2-rs}"

if [ ! -x "$ELF2UF2" ]; then
	echo "Install the UF2 helper with: cargo install elf2uf2-rs --locked" >&2
	exit 1
fi

if [ -d /Volumes/RP2350 ] && [ ! -d /Volumes/RPI-RP2 ]; then
	cat >&2 <<'EOF'
Finder is showing a drive named RP2350. That is a Pico 2 (RP2350 chip).

This firmware is built for Pico W / Pico WH (RP2040). That board appears as
RPI-RP2. Copying this file onto RP2350 will not run.

Hold BOOTSEL on the Pico WH and plug it in instead. If the board you have
really is a Pico 2 W, say so and the experiment can be retargeted.
EOF
	exit 1
fi

if [ ! -d /Volumes/RPI-RP2 ]; then
	cat >&2 <<'EOF'
No RPI-RP2 drive.

1. Unplug USB.
2. Hold BOOTSEL on the Pico WH.
3. Plug USB in, then release BOOTSEL.
4. Wait until Finder shows RPI-RP2, then run this again.

Use a data cable, not a charge-only cable.
EOF
	exit 1
fi

"$ELF2UF2" "$ELF" "$UF2"
cp "$UF2" /Volumes/RPI-RP2/
echo "Copied $UF2 to RPI-RP2. The drive should disappear; that means it worked."

CARGO ?= cargo
TARGET := thumbv6m-none-eabi
BIN := temp-oled-experiment
ELF := target/$(TARGET)/release/$(BIN)
UF2 := $(BIN).uf2
FLASH := ./flash-pico.sh

# Make uses /bin/sh and may not see ~/.cargo/bin even when cargo itself is on PATH.
ELF2UF2 := $(shell command -v elf2uf2-rs 2>/dev/null)
ifeq ($(ELF2UF2),)
  ELF2UF2 := $(HOME)/.cargo/bin/elf2uf2-rs
endif

.PHONY: all build uf2 flash clean help

all: build

# ARM ELF for the Pico. Cargo keeps incremental state in target/.
build:
	$(CARGO) build --release
	@echo
	@echo "Firmware ELF: $(ELF)"

# Optional UF2 for drag-and-drop onto the bootloader drive.
uf2: build
	@if [ ! -x "$(ELF2UF2)" ]; then \
		echo "Install the UF2 helper with: cargo install elf2uf2-rs --locked"; \
		exit 1; \
	fi
	$(ELF2UF2) $(ELF) $(UF2)
	@echo "UF2: $(UF2)"

# Build and copy onto a Pico WH already showing as RPI-RP2.
flash: build
	$(FLASH) $(ELF)

clean:
	$(CARGO) clean
	rm -f $(UF2)

help:
	@echo "make        build release firmware ELF"
	@echo "make uf2    also write $(UF2)"
	@echo "make flash  copy firmware onto a Pico WH in BOOTSEL mode"
	@echo "make clean  remove build output"

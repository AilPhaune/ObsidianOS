BUILD_DIR?=build
SRC_DIR?=src
CARGO?=cargo

CARGO_BUILD_DIR=target/x86-unknown-bare_metal/debug/deps

.PHONY: all stage3asm stage3

all: stage3asm stage3

stage3: stage3asm
	cargo rustc -- -C link-args=-Tlinker.ld --emit obj
	objcopy -O binary $(CARGO_BUILD_DIR)/$$(ls $(CARGO_BUILD_DIR) | grep \\.o) $(BUILD_DIR)/x86/bootloader_stage3.bin

stage3asm: main.asm
BUILD_DIR?=build
SRC_DIR?=src
CARGO?=cargo

ASM?=nasm
ASM_FLAGS?=-f elf32 -F dwarf -g

LD = "ld.lld"

CARGO_CONFIG=release
CARGO_BUILD_DIR=target/x86-unknown-bare_metal/$(CARGO_CONFIG)/deps

.PHONY: all stage3asm stage3 clean

all: stage3asm stage3

stage3: stage3asm
	$(CARGO) rustc --$(CARGO_CONFIG) -- -C link-args=-Tlinker.ld --emit obj
	$(LD) -T linker.ld build/main.o $(CARGO_BUILD_DIR)/stage3-*.o -o build/stage3.o

	objcopy -O binary build/stage3.o $(BUILD_DIR)/x86/bootloader_stage3.bin
	objcopy --only-keep-debug build/stage3.o $(BUILD_DIR)/x86/bootloader_stage3.debug

stage3asm: build/main.o

build/main.o: main.asm
	mkdir -p build
	$(ASM) $(ASM_FLAGS) -o build/main.o main.asm

clean:
	rm -rf target
	rm -rf build
BUILD_DIR?=build

ASM?=nasm
ASM_FLAGS?=-f bin

.PHONY: all clean

all: $(BUILD_DIR)/x86/bootloader.bin

$(BUILD_DIR)/x86/bootloader.bin: bootloader.asm
	$(ASM) $(ASM_FLAGS) -o $(BUILD_DIR)/x86/bootloader.bin bootloader.asm

clean:
	rm -f $(BUILD_DIR)/x86/bootloader.bin
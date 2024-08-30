BUILD_DIR?=build

ASM?=nasm
ASM_FLAGS?=-f bin

.PHONY: all clean

all: $(BUILD_DIR)/x86/bootloader_stage2.bin

$(BUILD_DIR)/x86/bootloader_stage2.bin: stage2.asm
	$(ASM) $(ASM_FLAGS) -o $(BUILD_DIR)/x86/bootloader_stage2.bin stage2.asm

clean:
	rm -f $(BUILD_DIR)/x86/bootloader_stage2.bin
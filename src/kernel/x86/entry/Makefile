BUILD_DIR?=build
SRC_DIR?=src

ASM?=nasm
ASM_FLAGS?=-f bin

.PHONY: all clean

all: kernel_entry

kernel_entry: $(BUILD_DIR)/x86/kernel_entry.bin

$(BUILD_DIR)/x86/kernel_entry.bin: main.asm
	echo "Build dir: $(BUILD_DIR)"
	$(ASM) $(ASM_FLAGS) -o $(BUILD_DIR)/x86/kernel_entry.bin main.asm

clean:
	rm -f $(BUILD_DIR)/x86/kernel_entry.bin
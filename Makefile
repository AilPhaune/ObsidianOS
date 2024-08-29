ASM=nasm            # Assembler command
ASM_FLAGS=-f bin    # Assembler flags for bootloader (flat binary format)

BUILD_DIR=build

# Default target
all: make_dirs bootloader kernel_entry floppy-x86

# Make directories
make_dirs:
	mkdir -p $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)/x86

# Bootloader target
bootloader: $(BUILD_DIR)/x86/bootloader.bin

# Kernel entry target
kernel_entry: $(BUILD_DIR)/x86/kernel_entry.bin

# Compile the ASM file to a flat binary
$(BUILD_DIR)/x86/bootloader.bin: src/bootloader/x86/bootloader.asm
	$(ASM) $(ASM_FLAGS) -o $(BUILD_DIR)/x86/bootloader.bin src/bootloader/x86/bootloader.asm

# Compile the ASM file
$(BUILD_DIR)/x86/kernel_entry.bin: src/kernel/x86/main.asm
	$(ASM) $(ASM_FLAGS) -o $(BUILD_DIR)/x86/kernel_entry.bin src/kernel/x86/main.asm

floppy-x86: bootloader kernel_entry
	dd if=/dev/zero of=$(BUILD_DIR)/x86.img bs=512 count=2880
	mkfs.fat -F 12 -n "AilphauneOS" $(BUILD_DIR)/x86.img
	dd if=$(BUILD_DIR)/x86/bootloader.bin of=$(BUILD_DIR)/x86.img conv=notrunc
	mcopy -i $(BUILD_DIR)/x86.img $(BUILD_DIR)/x86/kernel_entry.bin "::kernel.bin"

# Clean up generated files
clean:
	rm -rf $(BUILD_DIR)

# Phony targets
.PHONY: all clean
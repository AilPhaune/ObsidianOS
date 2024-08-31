ASM=nasm            # Assembler command
ASM_FLAGS=-f bin    # Assembler flags for bootloader (flat binary format)

BUILD_DIR=build
SRC_DIR=src

# Default target
all: make_dirs bootloader kernel floppy-x86

# Make directories
make_dirs:
	mkdir -p $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)/x86

# Bootloader target
bootloader:
	$(MAKE) -B -C $(SRC_DIR)/bootloader/x86 all BUILD_DIR=$(abspath $(BUILD_DIR)) SRC_DIR=$(abspath $(SRC_DIR))

kernel:
	$(MAKE) -B -C $(SRC_DIR)/kernel/x86 all BUILD_DIR=$(abspath $(BUILD_DIR)) SRC_DIR=$(abspath $(SRC_DIR))

floppy-x86: bootloader kernel
	dd if=/dev/zero of=$(BUILD_DIR)/x86.img bs=512 count=2880
	mkfs.fat -F 12 -n "AilphauneOS" $(BUILD_DIR)/x86.img
	dd if=$(BUILD_DIR)/x86/bootloader.bin of=$(BUILD_DIR)/x86.img conv=notrunc
	mcopy -i $(BUILD_DIR)/x86.img $(BUILD_DIR)/x86/bootloader_stage2.bin "::stage2.bin"
	mcopy -i $(BUILD_DIR)/x86.img $(BUILD_DIR)/x86/bootloader_stage3.bin "::stage3.bin"

# Clean up generated files
clean:
	rm -rf $(BUILD_DIR)

# Phony targets
.PHONY: all clean
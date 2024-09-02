ASM=nasm            # Assembler command
ASM_FLAGS=-f bin    # Assembler flags for bootloader (flat binary format)

BUILD_DIR=build
SRC_DIR=src

# Default target
all: make_dirs bootloader kernel

# Make directories
make_dirs:
	mkdir -p $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)/x86

# Bootloader target
bootloader:
	$(MAKE) -B -C $(SRC_DIR)/bootloader/x86 all BUILD_DIR=$(abspath $(BUILD_DIR)) SRC_DIR=$(abspath $(SRC_DIR))

kernel:
	$(MAKE) -B -C $(SRC_DIR)/kernel/x86 all BUILD_DIR=$(abspath $(BUILD_DIR)) SRC_DIR=$(abspath $(SRC_DIR))

x86-disk:
	build_scripts/x86/image.sh

# Clean up generated files
clean:
	rm -rf $(BUILD_DIR)

# Phony targets
.PHONY: all clean
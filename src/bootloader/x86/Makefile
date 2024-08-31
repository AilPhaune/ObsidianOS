BUILD_DIR?=build
SRC_DIR?=src

ASM?=nasm
ASM_FLAGS?=-f bin

.PHONY: all clean

all: stage1 stage2 stage3

stage1:
	$(MAKE) -C $(SRC_DIR)/bootloader/x86/stage1 all BUILD_DIR=$(abspath $(BUILD_DIR)) SRC_DIR=$(abspath $(SRC_DIR))

stage2:
#	$(MAKE) -C $(SRC_DIR)/bootloader/x86/stage2 all BUILD_DIR=$(abspath $(BUILD_DIR)) SRC_DIR=$(abspath $(SRC_DIR))

stage3:
#	$(MAKE) -C $(SRC_DIR)/bootloader/x86/stage3 all BUILD_DIR=$(abspath $(BUILD_DIR)) SRC_DIR=$(abspath $(SRC_DIR))

clean:
	rm -f $(BUILD_DIR)/x86/bootloader.bin
	rm -f $(BUILD_DIR)/x86/bootloader_stage2.bin
	rm -f $(BUILD_DIR)/x86/bootloader_stage3.bin
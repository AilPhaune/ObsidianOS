BUILD_DIR?=build
SRC_DIR?=src

ASM?=nasm
ASM_FLAGS?=-f bin

.PHONY: all clean

all: kernel_entry

kernel_entry:
	$(MAKE) -C $(SRC_DIR)/kernel/x86/entry all BUILD_DIR=$(abspath $(BUILD_DIR)) SRC_DIR=$(abspath $(SRC_DIR))

clean:
	
set disassembly-flavor intel
symbol-file build/x86/bootloader_stage3.debug
layout asm
layout regs
target remote | qemu-system-i386 -S -gdb stdio -drive format=raw,file=build/x86.img
ORG 0x0
BITS 16

kernel_entry:
    mov ax, cs
    mov ds, ax
    mov es, ax

    mov si, msg
    call puts

    cli
    hlt

%include "../shared/puts.asm"

msg: db "Hello world from bootloader stage 2 !", 13, 10, 0
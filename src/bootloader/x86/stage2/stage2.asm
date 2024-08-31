ORG 0
BITS 16

kernel_entry:
    cli
    mov ax, cs
    mov ds, ax
    mov ss, ax
    mov sp, 0
    sti

    xor dh, dh
    mov [boot_drive], dl

    push word msg_searching_kernel
    call _puts
    add sp, 2

    call start_pmode

kernel_end:
    cli
    hlt
    jmp $

load_gdt:
    [bits 16]
    lgdt [GDT_descriptor]
    ret

%include "gdt.asm"

;
; Start 32 bit protected mode
;
start_pmode:
    ; Preparations for protected mode
    cli
    call enable_a20
    call load_gdt

    ; Switch to protected mode
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; Far jump to 32-bit code segment
    jmp dword GDT_32bit_CODE_SEG:STAGE2_LOAD_SEGMENT*16+.pmode

.pmode:
    [bits 32]
    ; Protected mode code

    mov ax, GDT_32bit_DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov ebp, 0x90000
    mov esp, ebp

    hlt
    jmp $


%include "stdio.asm"

boot_drive:             db 0
ENDL:                   db 13, 10, 0

msg_searching_kernel:   db "Searching for kernel.", 13, 10, 0

enable_a20:
    [bits 16]
    ; Disable keyboard
    call a20_wait_input
    mov al, PS2_DISABLE_KEYBOARD
    out PS2_COMMAND_PORT, al

    ; Enable A20
    call a20_wait_input
    mov al, PS2_READ_OUTPUT
    out PS2_COMMAND_PORT, al

    call a20_wait_output
    in al, PS2_DATA_PORT
    push eax

    call a20_wait_input
    mov al, PS2_WRITE_OUTPUT
    out PS2_COMMAND_PORT, al

    call a20_wait_input
    pop eax
    or al, 2                    ; Enable A20 bit
    out PS2_DATA_PORT, al

    ; Enable keyboard
    call a20_wait_input
    mov al, PS2_ENABLE_KEYBOARD
    out PS2_COMMAND_PORT, al

    call a20_wait_input
    ret

a20_wait_input:
    [bits 16]
    ; We must wait until status bit 2 is 0
    in al, PS2_COMMAND_PORT     ; Read status byte
    test al, 2                  ; Test bit 2
    jnz a20_wait_input
    ret

a20_wait_output:
    [bits 16]
    ; We must wait until status bit 1 is 1 so it can be read
    in al, PS2_COMMAND_PORT
    test al, 1
    jz a20_wait_output
    ret

PS2_DATA_PORT           equ 0x60
PS2_COMMAND_PORT        equ 0x64
PS2_ENABLE_KEYBOARD     equ 0xAE
PS2_DISABLE_KEYBOARD    equ 0xAD
PS2_READ_OUTPUT         equ 0xD0
PS2_WRITE_OUTPUT        equ 0xD1

%include "../stage1/stage2_info.asm"

;
; `memcpy`: Copies x bytes of memory from a source pointer into a destination pointer
;
; Parameters:
; fs:di: dest
; gs:si: source
; cx: len
memcpy:
    ;push bx
    ;push cx
    ;push si
    ;push di
.loop:
    test cx, cx
    jz .end
    mov bl, [gs:si]
    mov [fs:di], bl
    inc si
    inc di
    dec cx
    jmp .loop
.end:
    ;pop di
    ;pop si
    ;pop cx
    ;pop bx
    ret
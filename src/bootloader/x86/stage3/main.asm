BITS 32
EXTERN rust_entry

SECTION .text

GLOBAL stage3_entry
stage3_entry:
    call rust_entry
    cli
    hlt
    jmp $

; Write a byte to an I/O port
; Parameters:
;   [esp + 4]: Port address (16-bit)
;   [esp + 8]: Value to write (8-bit)
GLOBAL outb
outb:
    movzx edx, word [esp + 4]   ; Load port address into DX (16-bit)
    movzx eax, byte [esp + 8]   ; Load value to write into AL (8-bit)
    out dx, al                  ; Output the byte in AL to the port in DX
    ret                         ; Return from function

; Read a byte from an I/O port
; Returns:
;   AL: The value read from the port
;   [esp + 4]: Port address (16-bit)
GLOBAL inb
inb:
    xor eax, eax                ; Clear eax
    movzx edx, word [esp + 4]   ; Load port address into DX (16-bit)
    in al, dx                   ; Input a byte from the port in DX into AL
    ret                         ; Return from function

%include "asm/interrupts.asm"
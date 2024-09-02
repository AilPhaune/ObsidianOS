EXTERN stage3_isr_handler

GLOBAL load_idt
load_idt:
    ; Make new call frame
    push ebp
    mov ebp, esp

    ; Load idt
    mov eax, [ebp + 8]
    lidt [eax]
    
    ; Exit call frame
    mov esp, ebp
    pop ebp
    ret

%macro ISR_NO_ERROR 1

GLOBAL isr%1
isr%1:
    push 0          ; Dummy error code
    push %1         ; Interrupt number
    jmp isr_common

%endmacro

%macro ISR_ERROR 1

GLOBAL isr%1
isr%1:
    push %1         ; Interrupt number
    jmp isr_common

%endmacro

isr_common:
    pusha

    ; Push ds
    xor eax, eax
    mov ax, ds
    push eax

    ; Use stage3 segments
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Pass a pointer to the current stack to rust function   
    push esp
    call stage3_isr_handler
    add esp, 4

    ; Restore segments
    pop eax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    popa
    ; Remove error code and interrupt number
    add esp, 8
    iret

GLOBAL clear_interrupts
clear_interrupts:
    cli
    ret

GLOBAL enable_interrupts
enable_interrupts:
    sti
    ret

%include "asm/isr.asm"
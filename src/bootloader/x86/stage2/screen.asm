BITS 16

;
; Writes a teletype character using BIOS function
;
; Parmeters:
; character
; page number
_x86_WriteTeletypeCharacter:
    ; Make new call frame
    push bp
    mov bp, sp

    push bx                     ; In CDECL, only A C and D registers are saved by the caller

    mov al, [bp + 4]            ; The character
    mov bh, [bp + 6]            ; The page number

    mov ah, 0x0E
    int 0x10                    ; Call BIOS function using interrupt

    pop bx                      ; Restore B register

    ; Restore old call frame
    mov sp, bp
    pop bp
    ret
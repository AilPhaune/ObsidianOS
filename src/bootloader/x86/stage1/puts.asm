;
; `puts`: Writes a string to the screen
;
; Parameters:
; ds:si: Address of null terminated string
puts:
    pusha

.loop:
    ; Load byte at DS:SI into AL and increment SI
    lodsb
    ; Check if null character
    test al, al
    ; End if null character
    jz .end

    ; Call BIOS function
    mov ah, 0x0E
    xor bx, bx
    int 0x10

    ; Continue loop
    jmp .loop

.end:
    popa
    ret

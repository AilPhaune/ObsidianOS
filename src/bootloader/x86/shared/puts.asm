;
; `puts`: Writes a string to the screen
;
; Parameters:
; ds:si: Address of null terminated string
puts:
    push ax
    push bx
    push si

.loop:
    mov al, [ds:si]

    ; Check if null character
    cmp al, 0
    jz .end

    ; Call BIOS function
    xor bx, bx
    mov ah, 0x0E
    int 0x10

    ; Increment pointer
    inc si
    jmp .loop

.end:
    pop si
    pop bx
    pop ax
    ret

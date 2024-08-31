BITS 16

INT_PRINT_BUFFER_SIZE           equ 32

;
; `_putc`: Prints a character to the screen
;
; Parameters:
; character
_putc:
    ; Make new call frame
    push bp
    mov bp, sp

    ; Save registers
    push ax
    push cx
    push dx

    push word 0
    push word [bp + 4]
    call _x86_WriteTeletypeCharacter
    add sp, 4

    ; Restore registers
    pop dx
    pop cx
    pop ax

    ; Restore old call frame
    mov sp, bp
    pop bp
    ret

;
; `_puts`: Prints a string to the screen
;
; Parameters:
; pointer to null terminated string
_puts:
    ; Make new call frame
    push bp
    mov bp, sp

    ; Save registers
    push si

    mov si, [bp + 4]            ; Read the pointer passed as parameter
.loop:
    xor ax, ax                  ; Clear ax
    mov al, [ds:si]             ; Read the character in al
    test al, al
    jz .end                     ; Exit loop if null character
    push ax                     ; Push the character for _putc
    call _putc                  ; Print the character
    pop ax                      ; Pop back ax
    inc si                      ; Increment pointer
    jmp .loop                   ; Continue loop

.end:
    ; Restore register
    pop si

    ; Restore old call frame
    mov sp, bp
    pop bp
    ret

;
; `_write_u8`: Write an unsigned 8-bit integer to a buffer and returns an address to that buffer
;
; Parameters:
; the u8 int
; Returns:
; ax = pointer to null terminated string
_write_u8:
    ; Make new call frame
    push bp
    mov bp, sp

    mov al, [bp + 4]            ; al = the u8
    xor ah, ah                  ; Clear the high bits that might contain garbage
    push ax                     ; Argument to _write_u16

    call _write_u16             ; Return value is already in ax

    ; Restore old call frame
    mov sp, bp
    pop bp
    ret

;
; `_write_u16`: Write an unsigned 16-bit integer to a buffer and returns an address to that buffer
;
; Parameters:
; the u16 int
; Returns:
; ax = pointer to null terminated string
_write_u16:
    ; Make new call frame
    push bp
    mov bp, sp

    mov ax, [bp + 4]            ; ax = the u16

    push si                     ; Store si
    push bx                     ; Store bx
    mov si, INT_PRINT_BUFFER_SIZE - 1
    mov [cs:si], byte 0

.loop:
    dec si
    mov bx, 10                  ; bx = 10 is the divisor
    xor dx, dx                  ; dx = 0 because div instruction treats dx as the hight 16 bits of dividend
    div bx                      ; ax = ax / 10   bx = ax % 10
    or dl, 48                   ; Because or is faster than addition, and that ASCII has been well thought
    mov [cs:si], dl             ; Store the digit in the string
    test ax, ax                 ; Test if ax is 0
    jnz .loop                   ; Continue the loop while it isn't 0
    mov ax, si                  ; Move the pointer to the beginning of the string into ax

    pop bx                      ; Restore bx
    pop si                      ; Restore si

    ; Restore old call frame
    mov sp, bp
    pop bp
    ret

;
; `_write_u8_hex`: Write an unsigned 8-bit integer in hexadecimal to a buffer and returns an address to that buffer
;
; Parameters:
; the u8 int
; Returns:
; ax = pointer to null terminated string
_write_u8_hex:
    ; Make new call frame
    push bp
    mov bp, sp

    mov al, [bp + 4]            ; al = the u8
    xor ah, ah                  ; Clear the high bits that might contain garbage
    push ax                     ; Argument to _write_u16

    call _write_u16_hex         ; Return value is already in ax

    ; Restore old call frame
    mov sp, bp
    pop bp
    ret

;
; `_write_u16_hex`: Write an unsigned 16-bit integer in hexadecimal to a buffer and returns an address to that buffer
;
; Parameters:
; the u16 int
; Returns:
; ax = pointer to null terminated string
_write_u16_hex:
    ; Make new call frame
    push bp
    mov bp, sp

    mov ax, [bp + 4]            ; ax = the u16

    push si                     ; Store si
    push bx                     ; Store bx
    
    mov si, INT_PRINT_BUFFER_SIZE - 1
    mov [cs:si], byte 0

.loop:
    dec si

    mov dl, al                  ; dl = al
    and dl, 0x0F                ; Keep the lower nibble
    cmp dl, 10                  ; Compare dl to 10
    mov cx, '0'
    mov bx, 'A'-10
    cmovb bx, cx                ; If dl < 10, we move the ASCII code of '0' into ax, otherwise it stays to 'A'-10
    add dl, bl                  ; Add the nibble to get the corresponding hex character
    mov [cs:si], dl             ; Add the hex character to the buffer
    shr ax, 4                   ; Go to nex character

    test ax, ax                 ; Test if ax is 0
    jnz .loop                   ; Continue the loop while it isn't 0
    mov ax, si                  ; Move the pointer to the beginning of the string into ax

    pop bx                      ; Restore bx
    pop si                      ; Restore si

    ; Restore old call frame
    mov sp, bp
    pop bp
    ret


integer_print_buffer:           times INT_PRINT_BUFFER_SIZE db 0

%include "screen.asm"

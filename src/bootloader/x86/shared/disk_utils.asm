;
; `lba_to_chs`: Converts an LBA address to CHS, with the registers already setup for the BIOS functions
;
; Parameters: 
; ax: LBA address
;
; Returns:
; cx: [bits 0-5: Sector number] [bits 6-15: Cylinder]
; dh: Head
lba_to_chs:
    push ax
    push dx

    xor dx, dx                       ; dx = 0
    div word [bpb_sectors_per_track] ; ax = lba / sectors_per_track   dx = lba % sectors_per_track
    inc dx                           ; dx = (lba % sectors_per_track) + 1 = sector
    mov cx, dx                       ; cx = sector
    xor dx, dx                       ; dx = 0
    div word [bpb_head_count]        ; ax = (lba / sectors_per_track) / head_count = cylinder   dx = (lba / sectors_per_track) % head_count = head
    mov dh, dl                       ; dl = head
    mov ch, al                       ; ch = lower 8 bits of cylinder
    shl ah, 6                        ; ah = upper 2 bits of cylinder << 6
    or cl, ah                        ; put that upper 2 bits in the right place

    pop ax
    mov dl, al
    pop ax
    ret

;
; `read_disk`
;
; Parameters:
; ax: LBA address
; cl: Number of sectors to read
; dl: Drive number
; es:bx: Memory address where read data goes
disk_read:
    push ax
    push bx
    push cx
    push dx
    push di

    push cx             ; Because `lba_to_chs` modifies cx
    call lba_to_chs     ; Compute CHS
    pop ax              ; al = Number of sectors to read

    mov ah, 0x02
    mov di, 3           ; Retry count
.retry:
    pusha               ; We're unsure what the BIOS could be doing to our registers
    stc                 ; Set the carry flag in case the BIOS doesn't
    int 0x13            ; Carry flag cleared = success
    jnc .done           ; Jump out if carry is not set
    popa                ; Restore the registers
    call disk_reset     ; Reset the disk before retry
    dec di              ; Decrement the the retry count
    test di, di         ; Test if it is 0
    jnz .retry          ; If we still have tries, go back to the begining of the loop
.fail:
    ; After all attemps failed
    mov si, msg_read_failed
    jmp bootloader_error
.done:
    popa                ; Restore the registers
    pop di
    pop dx
    pop cx
    pop bx
    pop ax
    ret

;
; `disk_reset`: Resets a drive controller
;
; Parameters:
; dl: Drive number
disk_reset:
    pusha
    mov ah, 0
    stc
    int 0x13
    mov si, msg_reset_failed
    jc bootloader_error
    popa
    ret
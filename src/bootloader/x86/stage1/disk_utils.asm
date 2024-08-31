;
; `extended_read_disk`
;
; Parameters:
; eax: LBA address
; cx: Number of sectors to read
; dl: Drive number
; es:bx: Memory address where read data goes
extended_read_disk:
    pusha
    mov [disk_address_paket.size], byte 0x10
    mov [disk_address_paket.unused], byte 0
    mov [disk_address_paket.lba], eax
    mov [disk_address_paket.lba+4], dword 0
    mov [disk_address_paket.segment], es
    mov [disk_address_paket.offset], bx
    mov [disk_address_paket.sectors_read_count], cx

    ; Retry count
    mov di, 3
.retry:
    pusha                       ; Save registers
    stc                         ; Set carry flag, gets cleared if success
    mov ah, 0x42                ; Parameter
    mov si, disk_address_paket  ; Parameter
    int 0x13                    ; Call BIOS function
    popa                        ; Restore registers
    jnc .done                   ; If carry flag got cleared -> success
    call disk_reset             ; Reset disk controller

    dec di                      ; Decrement counter
    test di, di                 ; Test counter
    jnz .retry                  ; If counter > 0 -> retry
.fail:
    ; Read from disk failed
    mov si, msg_read_failed
    jmp bootloader_error
.done:
    popa
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
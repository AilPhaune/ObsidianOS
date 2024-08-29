BITS 16
ORG 0x7c00

; FAT Headers
bpb_start:
    jmp short start
    nop

bpb_oem:                     db "MSWIN4.1"
bpb_bytes_per_sector:        dw 512
bpb_sectors_per_cluster:     db 1
bpb_reserved_sectors:        dw 1
bpb_fat_count:               db 2
bpb_root_directory_entries:  dw 0xE0
bpb_total_sector_count:      dw 2880
bpb_media_descriptor:        db 0xF0
bpb_sectors_per_fat:         dw 9
bpb_sectors_per_track:       dw 18
bpb_head_count:              dw 2
bpb_hidden_sector_count:     dd 0
bpb_large_sector_count:      dd 0

; Extended boot record
ebr_drive_number:            db 0 ; Will be set from the value of register dl
ebr_reserved:                db 0
ebr_signature:               db 0x29
ebr_volume_id_serial:        db 0x31, 0x41, 0x59, 0x26
ebr_volume_label:            db "AilphauneOS" ; 11 bytes
ebr_system_identifier:       db "FAT12   " ; 8 bytes

start:
    cli
    ; Setup segment registers
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Setup stack
    mov ss, ax
    mov sp, 0x7c00

    mov si, hello_world
    call puts

    sti

end:
    hlt
    jmp $

; Parameters:
;
; si: adress of null terminated string
; ds: data segment
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

hello_world: db "Hello, World !", 13, 10, 0

times 510-($-$$) db 0
db 0x55, 0xAA
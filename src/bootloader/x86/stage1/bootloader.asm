BITS 16
ORG 0

start:
    cli
    ; Setup segment registers
    mov ax, 0x07c0
    mov ds, ax
    mov es, ax
    mov gs, ax

    ; Setup stack
    xor ax, ax
    mov ss, ax
    mov sp, 0x7a00
    sti

    ; Most BIOSes load us into 0x0000:0x7c00, but we need the ORG to be 0, so there won't be problems when we'll relocate to 0x7a00.
    jmp 0x07c0:.bootloader

.bootloader:
    mov [boot_drive], dl

    cmp ecx, ASCII_OBSI
    jne .not_loaded_by_obsidian
    cmp edx, ASCII_DIAN
    jne .not_loaded_by_obsidian

.loaded_by_obsidian:
    mov si, msg_obsi
    call puts
    jmp $

.not_loaded_by_obsidian:
    ; Copy bootloader at 0x7a00
    cli
    mov ax, 0x7a0
    mov fs, ax                          ; Destination segment
    mov si, 0                           ; Source index: 0x07c0:0x0000
    mov di, 0                           ; Destination index: 0x7a0:0x0000
    mov bx, 512                         ; Number of bytes to copy
.copy_loop:
    mov al, [ds:si]                     ; Load byte
    mov [fs:di], al                     ; Store byte
    inc si                              ; Next source byte
    inc di                              ; Next destination byte
    dec bx                              ; Decrement counter
    test bx, bx                         ; Test counter
    jnz .copy_loop                      ; Continue until all bytes copied

    ; Jump to the copied bootloader
    jmp 0x07a0:.after

.after:
    mov ax, 0x7a0
    mov ds, ax
    mov es, ax
    sti

    ; Check extensions present
    mov ah, 0x41
    mov bx, 0x55AA
    stc
    int 0x13
    jc extensions_not_present
    cmp bx, 0xAA55
    jne extensions_not_present

    ; Read drive parameters
    mov ah, 0x48
    stc
    mov si, disk_parameters
    mov [si], byte 0x1E                 ; Set the length field
    int 0x13
    mov si, msg_read_parameters_failed
    jc bootloader_error

    ; Find bootable partitions
    mov si, MBR_Entry1.status
    mov bx, 4
.find_bootable:
    dec bx
    test bx, bx
    jz .found_no_bootable_partitions
    mov ax, [ds:si]
    test ax, 0x80
    jnz .found_bootable
    add si, MBR_Entry_Size
    jmp .find_bootable

.found_bootable:
    ; Found bootable
    push si
    mov si, msg_found_bootable
    call puts
    pop si

    ; Move si to the lba address
    add si, MBR_Entry1.lba_start - MBR_Entry1.status

    ; Get LBA
    mov eax, [si]

    mov cx, 1
    ; Load partition boot sector at 0x7c00 as it expects to be
    xor bx, bx
    mov ds, bx

    mov bx, 512
    call extended_read_disk

    ; Let the partition boot loader know it was loaded by Obsidian Boot Loader
    mov ecx, ASCII_OBSI
    mov edx, ASCII_DIAN

    ; Jump to partition boot sector
    jmp 0x0000:0x7c00

.found_no_bootable_partitions:
    mov si, msg_no_bootable_partitions
    jmp bootloader_error

extensions_not_present:
    mov si, msg_extensions_not_present

bootloader_error:
    call puts

wait_key_and_reboot:
    mov ah, 0
    int 0x16                    ; Wait for keypress
    jmp 0xFFFF:0                ; Jump to beginning of BIOS, should reboot

bootloader_end:
    cli
    hlt
    jmp $

%include "puts.asm"
%include "disk_utils.asm"

%include "stage2_info.asm"

boot_drive:                     db 0

msg_extensions_not_present:     db "ERR: No extensions", 13, 10, 0
msg_read_failed:                db "ERR: Read", 13, 10, 0
msg_read_parameters_failed:     db "ERR: Params", 13, 10, 0
msg_reset_failed:               db "ERR: Reset", 13, 10, 0
msg_no_bootable_partitions:     db "ERR: No boot partition", 13, 10, 0
msg_obsi:                       db "Boot from obsi", 13, 10, 0
msg_found_bootable:             db "Found boot", 13, 10, 0

times 446-($-$$) db 0

MBR_Entry1:
.status:                        db 0
.chs_start:                     db 0, 0, 0
.partition_type:                db 0
.chs_end:                       db 0, 0, 0
.lba_start:                     db 0, 0, 0, 0
.sector_count:                  db 0, 0, 0, 0
MBR_Entry2:
.status:                        db 0
.chs_start:                     db 0, 0, 0
.partition_type:                db 0
.chs_end:                       db 0, 0, 0
.lba_start:                     db 0, 0, 0, 0
.sector_count:                  db 0, 0, 0, 0
MBR_Entry3:
.status:                        db 0
.chs_start:                     db 0, 0, 0
.partition_type:                db 0
.chs_end:                       db 0, 0, 0
.lba_start:                     db 0, 0, 0, 0
.sector_count:                  db 0, 0, 0, 0
MBR_Entry4:
.status:                        db 0
.chs_start:                     db 0, 0, 0
.partition_type:                db 0
.chs_end:                       db 0, 0, 0
.lba_start:                     db 0, 0, 0, 0
.sector_count:                  db 0, 0, 0, 0

MBR_Entry_Size                  equ 16
ASCII_OBSI                      equ 0x4F627369
ASCII_DIAN                      equ 0x6469616E


times 510-($-$$) db 0
db 0x55, 0xAA

disk_parameters:
.size:                          dw 0x1E
.flags:                         dw 0
.cylinders:                     dd 0
.heads:                         dd 0
.sectors_per_track:             dd 0
.sector_count:                  dq 0
.bytes_per_sector:              dw 0
.edd:                           dd 0

disk_address_paket:
.size:                          db 0x10
.unused:                        db 0
.sectors_read_count:            dw 0
.offset:                        dw 0
.segment:                       dw 0
.lba:                           dq 0
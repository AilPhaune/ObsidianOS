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

    ; Setup stack
    mov ss, ax
    mov sp, 0x7c00

    ; Some BIOSes might load the bootloader at 07C0:0000 instead of 0000:7C00
    push es
    push word .after
    retf
.after:

    ; Save disk number
    mov [ebr_drive_number], dl

    ; Get drive parameters: Sectors per track and head count from the disk might not be reliable, so we ask the BIOS
    push es
    mov ah, 0x08
    stc
    int 0x13
    jc bootloader_error
    pop es

    and cl, 0x3F                            ; Get rid of the top 2 bits
    xor ch, ch
    mov [bpb_sectors_per_track], cx         ; Sector count

    inc dh
    mov [bpb_head_count], dh                ; Head count

    ; Read FAT 12 root directory
    ; Start by computing the LBA: lba = reserved + fat_count*sectors_per_fat
    mov ax, [bpb_sectors_per_fat]           ; ax = sectors_per_fat
    mov bl, [bpb_fat_count]         
    xor bh, bh                              ; bx = fat_count
    mul bx                                  ; ax = sectors_per_fat*fat_count
    add ax, [bpb_reserved_sectors]          ; ax = LBA
    push ax
    ; Calculate size of root directory: entry_size * number_of_entries / bytes_per_sector ---- entry_size=32
    mov ax, [bpb_root_directory_entries]
    shl ax, 5                               ; ax = 32*bpb_sectors_per_fat
    xor dx, dx                              ; dx = 0
    div word [bpb_bytes_per_sector]         ; ax = (32 * number_of_entries) / bytes_per_sector   dx = (32 * number_of_entries) % bytes_per_sector
    test dx, dx                             ; if remainder != 0, add 1
    jz .end_route_directory_calculation
    inc ax
.end_route_directory_calculation:
    ; Actually ead the root directory
    mov cl, al                              ; Number of sectors to read = size of the root directory
    pop ax                                  ; LBA of the root directory
    mov dl, [ebr_drive_number]              ; Drive we just booted from
    mov bx, buffer                          ; es:bx = address of root directory buffer
    call disk_read

    ; Search our kernel
    xor bx, bx                              ; bx = 0 will serve as our entry counter
    mov di, buffer
.search_kernel:
    mov si, kernel_file_name                ; Setup string comparison
    mov cx, 11                              ; Length of the string
    push di                                 ; Save the address of the current entry
    repe cmpsb                              ; String comparison
    pop di                                  ; Restore the address of the current entry
    je .found_kernel                        ; If we found the kernel entry, jump to it

    add di, 32                              ; Otherwise, add 32 to the current entry address to get to next entry
    inc bx                                  ; Increment the entry count
    cmp bx, [bpb_root_directory_entries]    ; Compare the current entry count to the total entry count
    jl .search_kernel                       ; If it's less than, there are more entries to search
    
    mov si, msg_kernel_not_found            ; Otherwise we haven't found the kernel
    jmp bootloader_error

.found_kernel:
    ; di still has the address to the directory entry
    mov ax, [di + 26]                       ; Get the kernel's first cluster
    mov [kernel_cluster], ax

    ; Load the FAT from disk to memory
    mov ax, [bpb_reserved_sectors]
    mov bx, buffer
    mov cl, [bpb_sectors_per_fat]
    mov dl, [ebr_drive_number]
    call disk_read

    ; Read the whole kernel by processing the FAT chain
    mov bx, KERNEL_LOAD_SEGMENT
    mov es, bx
    mov bx, KERNEL_LOAD_OFFSET

.load_kernel_fat:
    ; Read next cluster
    mov ax, [kernel_cluster]
    add ax, 31                              ; TODO: Unhardcode this value
    mov cl, 1
    mov dl, [ebr_drive_number]
    call disk_read

    add bx, [bpb_bytes_per_sector]          ; TODO: Fix overflow if kernel size > 64Kb

    ; Calculate location of next cluster
    mov ax, [kernel_cluster]
    mov cx, 3
    mul cx
    mov cx, 2
    div cx                                  ; ax = index of entry in FAT, dx = cluster % 2

    mov si, buffer
    add si, ax
    mov ax, [ds:si]                         ; Read entry from FAT at index ax
    
    or dx, dx
    jz .even

.odd:
    shr ax, 4
.even:
    and ax, 0x0FFF

.next_cluster:
    cmp ax, 0xFF8                           ; If next_cluster >= 0xFF8, then it's the end of the file
    jae .read_kernel_end
    mov [kernel_cluster], ax
    jmp .load_kernel_fat

.read_kernel_end:
    
    ; Setup everything to jump to the kernel
    
    mov dl, [ebr_drive_number]              ; Boot drive number in dl
    mov ax, KERNEL_LOAD_OFFSET              ; Setup segment registers
    mov ds, ax
    mov es, ax

    jmp KERNEL_LOAD_SEGMENT:KERNEL_LOAD_OFFSET

    ; Kernel should never return but maybe ?
    jmp wait_key_and_reboot

bootloader_error:
    call puts

wait_key_and_reboot:
    mov ah, 0
    int 0x16           ; Wait for keypress
    jmp 0xFFFF:0       ; Jump to beginning of BIOS, should reboot

bootloader_end:
    hlt
    jmp $

%include "../shared/disk_utils.asm"
%include "../shared/puts.asm"

msg_read_failed:        db "Read from disk failed!", 13, 10, 0
msg_reset_failed:       db "Reset disk failed !", 13, 10, 0
msg_kernel_not_found:   db "Stage 2 not found !", 13, 10, 0
kernel_file_name:       db "STAGE2  BIN"

kernel_cluster:         dw 0

KERNEL_LOAD_SEGMENT     equ 0x1000
KERNEL_LOAD_OFFSET      equ 0

times 510-($-$$) db 0
db 0x55, 0xAA

buffer:
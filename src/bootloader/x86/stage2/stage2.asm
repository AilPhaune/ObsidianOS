ORG 0
BITS 16

    jmp stage2_entry
    db "Obsidian"

boot_drive:                     db 0
msg_loaded_by_obsidian:         db "Obsidian Loader.", 13, 10, 0
msg_not_loaded_by_obsidian:     db "FATAL: Stage 2 wasn't loaded by Obsidian !", 13, 10, 0
msg_read_failed:                db "FATAL: Read from disk failed !", 13, 10, 0
msg_reset_failed:               db "FATAL: Disk reset failed !", 13, 10, 0
msg_wait_key_and_reboot:        db 13, 10, "Press any key to reboot.", 13, 10, 0

%include "../stage1/disk_utils.asm"

disk_address_paket:
.size:                          db 0x10
.unused:                        db 0
.sectors_read_count:            dw 0
.offset:                        dw 0
.segment:                       dw 0
.lba:                           dq 0

remaining_stage2_sectors:       dw 0

stage2_entry:
    cmp ebx, ASCII_OBSI
    jne not_loaded_by_obsidian
    cmp ecx, ASCII_DIAN
    jne not_loaded_by_obsidian

    ; From OBSIDIAN bootloader: 
    ; REGISTER STATE:
    ;   eax = remaining sectors to load before first cluster)
    ;   ebx = ASCII of "Obsi"
    ;   ecx = ASCII of "dian"
    ;    dl = boot drive
    cli
    mov bx, cs
    mov ds, bx
    mov ss, bx
    mov sp, 0
    sti

    xor dh, dh
    mov [boot_drive], dl

    ; Limit amount of read sectors to 895
    ; https://wiki.osdev.org/Memory_Map_(x86) : We can only use memory from 0x8000 to 0x80000,
    ; and we're currently using 0x10000-0x10200 so usable memory is 0x80000-0x10200 bytes = 895 sectors
    mov ebx, 895
    cmp ebx, eax
    cmovb eax, ebx

    mov [remaining_stage2_sectors], ax
.read_loop:
    mov ax, [remaining_stage2_sectors]                  ; Get the remaining amount of sectors
    test ax, ax                                         ; Check if there are still some sectors to read ?
    jz .loaded_by_obsidian                              ; No more sectors to read
    mov cx, 63                                          ; Read 63 sectors (is the maximum for some BIOSes)
    cmp ax, cx                                          ; Compare ax to cx
    cmovb cx, ax                                        ; If ax<cx then only read ax sectors
    mov eax, 2                                          ; Read from sector 2 (the first 512 bytes of this files are loaded by OBSIDIAN stage 1 from sector 1, so the rest is at sector 2)
    mov bx, 0x1020                                      ; Read at segment 0x1020 (512 bytes after the current one, so just behind in memory)
    mov es, bx                                          ; Move this value to the segment register used by the read function
    xor bx, bx                                          ; Set the offset to this segment to 0
    call extended_read_disk                             ; Read from disk, function only returns if success.
    mov cx, [disk_address_paket.sectors_read_count]     ; Get the real amount of read sectors
    sub [remaining_stage2_sectors], cx                  ; Subtract the read amount of sectors from the remaining count
    shl cx, 5                                           ; Multiply the amount of sectors by 32 to get the number of segments to add (1 segment = 16 bytes offset, 1 disk sector = 512 bytes offset --> 1 disk sector = 512/16=32 segments)
    add bx, cx
    jmp .read_loop

.loaded_by_obsidian:
    push word msg_loaded_by_obsidian
    call _puts
    add sp, 2

    push word msg_searching_kernel
    call _puts
    add sp, 2

    call gather_data_for_stage3

    call start_pmode
    jmp wait_key_and_reboot

not_loaded_by_obsidian:
    mov si, msg_not_loaded_by_obsidian

bootloader_error:
    push si
    call _puts
    add sp, 2

wait_key_and_reboot:
    push word msg_wait_key_and_reboot
    call _puts
    add sp, 2

    mov ah, 0
    int 0x16                    ; Wait for keypress
    jmp 0xFFFF:0                ; Jump to beginning of BIOS, should reboot

kernel_end:
    cli
    hlt
    jmp $

times 512-($-$$) db 0           ; Explicitly throws an error if the above code doesn't fit in 512 bytes

current_partition_data_address:                     dw 0

gather_data_for_stage3:
    [bits 16]
    pusha

    ; Read disk boot sector at 0x7c00 (sounds familiar)
    mov ax, 0x07c0
    mov es, ax

    mov dl, [boot_drive]
    xor eax, eax
    mov cx, 1
    xor bx, bx
    call extended_read_disk

    movzx ebx, word [es:510]
    cmp ebx, 0xAA55
    jne .done_gathering_partition_data

    mov [current_partition_data_address], word 512

    mov si, 430
    mov bx, 4
.read_partition_bootsector_loop:
    dec bx                                          ; Decrement counter
    test bx, bx                                     ; Test counter
    jz .done_gathering_partition_data               ; Jump out if zero
    add si, 16                                      ; Size of an MBR entry

    movzx eax, byte [es:si]                         ; Get partition status
    test ax, 0x80                                   ; Test active bit
    jz .read_partition_bootsector_loop              ; Skip partition

    add si, 8                                       ; Skip to LBA of first sector
    pusha
    mov eax, dword [es:si]                          ; Get LBA
    mov cx, 1                                       ; Read 1 sector
    mov dl, [boot_drive]                            ; Drive number
    mov bx, [current_partition_data_address]        ; Address where to put the data
    call extended_read_disk
    add bx, 512                                     ; Next address where to read
    mov [current_partition_data_address], bx        ; Save next address
    popa
    sub si, 8
    jmp .read_partition_bootsector_loop

.done_gathering_partition_data:
    popa
    ret

load_gdt:
    [bits 16]
    lgdt [GDT_descriptor]
    ret

%include "gdt.asm"

;
; Start 32 bit protected mode
;
start_pmode:
    ; Preparations for protected mode
    cli
    call enable_a20
    call load_gdt

    ; Switch to protected mode
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; Far jump to 32-bit code segment
    jmp dword GDT_32bit_CODE_SEG:STAGE2_LOAD_SEGMENT*16+.pmode

.pmode:
    [bits 32]
    ; Protected mode code

    mov eax, GDT_32bit_DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov ebp, 0x10000
    mov esp, ebp

call_stage3:
    ; Call stage3
    and edx, 0xFF
    push edx
    jmp 0x2000          ; Relative jump

    hlt
    jmp $


%include "stdio.asm"

ENDL:                       db 13, 10, 0

msg_searching_kernel:       db "Searching for kernel.", 13, 10, 0

enable_a20:
    [bits 16]
    ; Disable keyboard
    call a20_wait_input
    mov al, PS2_DISABLE_KEYBOARD
    out PS2_COMMAND_PORT, al

    ; Enable A20
    call a20_wait_input
    mov al, PS2_READ_OUTPUT
    out PS2_COMMAND_PORT, al

    call a20_wait_output
    in al, PS2_DATA_PORT
    push eax

    call a20_wait_input
    mov al, PS2_WRITE_OUTPUT
    out PS2_COMMAND_PORT, al

    call a20_wait_input
    pop eax
    or al, 2                    ; Enable A20 bit
    out PS2_DATA_PORT, al

    ; Enable keyboard
    call a20_wait_input
    mov al, PS2_ENABLE_KEYBOARD
    out PS2_COMMAND_PORT, al

    call a20_wait_input
    ret

a20_wait_input:
    [bits 16]
    ; We must wait until status bit 2 is 0
    in al, PS2_COMMAND_PORT     ; Read status byte
    test al, 2                  ; Test bit 2
    jnz a20_wait_input
    ret

a20_wait_output:
    [bits 16]
    ; We must wait until status bit 1 is 1 so it can be read
    in al, PS2_COMMAND_PORT
    test al, 1
    jz a20_wait_output
    ret

PS2_DATA_PORT           equ 0x60
PS2_COMMAND_PORT        equ 0x64
PS2_ENABLE_KEYBOARD     equ 0xAE
PS2_DISABLE_KEYBOARD    equ 0xAD
PS2_READ_OUTPUT         equ 0xD0
PS2_WRITE_OUTPUT        equ 0xD1

%include "../stage1/stage2_info.asm"
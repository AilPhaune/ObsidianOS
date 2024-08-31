begin_GDT:
    GDT_null:
    ; Null descriptor
    dw 0, 0, 0, 0
    
    GDT_32bit_code:
    ; 32 bit code segment
    dw 0xFFFF                   ; bits  0-15 of `limit`
    dw 0                        ; bits  0-15 of `base address`
    db 0                        ; bits 16-23 of `base_address`
    db 10011010b                ; access flags: present, ring 0, code segment, is executable, direction 0, readable
    db 11001111b                ; granularity: 4K, bits 16-19 of `limit` = 1111
    db 0                        ; bits 24-31 of `base_address`

    GDT_32bit_data:
    ; 32 bit data segment
    dw 0xFFFF                   ; bits  0-15 of `limit`
    dw 0                        ; bits  0-15 of `base address`
    db 0                        ; bits 16-23 of `base_address`
    db 10010010b                ; access flags: present, ring 0, data segment, NOT executable, direction 0, readable
    db 11001111b                ; granularity: 4K, bits 16-19 of `limit` = 1111
    db 0                        ; bits 24-31 of `base_address`

    GDT_16bit_code:
    ; 16 bit code segment
    dw 0xFFFF                   ; bits  0-15 of `limit`
    dw 0                        ; bits  0-15 of `base address`
    db 0                        ; bits 16-23 of `base_address`
    db 10011010b                ; access flags: present, ring 0, code segment, is executable, direction 0, readable
    db 00001111b                ; granularity: 1B, bits 16-19 of `limit` = 1111
    db 0                        ; bits 24-31 of `base_address`

    GDT_16bit_data:
    ; 32 bit data segment
    dw 0xFFFF                   ; bits  0-15 of `limit`
    dw 0                        ; bits  0-15 of `base address`
    db 0                        ; bits 16-23 of `base_address`
    db 10010010b                ; access flags: present, ring 0, data segment, NOT executable, direction 0, readable
    db 00001111b                ; granularity: 1B, bits 16-19 of `limit` = 1111
    db 0                        ; bits 24-31 of `base_address`
end_GDT:

GDT_32bit_CODE_SEG              equ GDT_32bit_code - begin_GDT
GDT_32bit_DATA_SEG              equ GDT_32bit_data - begin_GDT
GDT_16bit_CODE_SEG              equ GDT_16bit_code - begin_GDT
GDT_16bit_DATA_SEG              equ GDT_16bit_data - begin_GDT

GDT_descriptor:
    ; Size of GDT
    dw (end_GDT - begin_GDT - 1)
    ; Address of GDT
    dd begin_GDT + STAGE2_LOAD_SEGMENT*16
end_GDT_descriptor:
BITS 32
EXTERN rust_entry

SECTION text

GLOBAL stage3_entry
stage3_entry:
    xor dh, dh
    push dx
    call rust_entry

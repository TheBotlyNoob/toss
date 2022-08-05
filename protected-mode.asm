[org 0x7c00]

CODE_SEG equ GDT_code - GDT_start
DATA_SEG equ GDT_data - GDT_start

cli
lgdt [GDT_descriptor]
mov eax, cr0
or eax, 1
mov cr0, eax
jmp CODE_SEG:start_protected_mode

jmp $

GDT_start:                          ; must be at the end of real mode code
    GDT_null:
        dd 0x00
        dd 0x00

    GDT_code:
        dw 0xffff
        dw 0x00
        db 0x00
        db 0b10011010
        db 0b11001111
        db 0x00

    GDT_data:
        dw 0xffff
        dw 0x00
        db 0x00
        db 0b10010010
        db 0b11001111
        db 0x00
GDT_end:

GDT_descriptor:
    dw GDT_end - GDT_start - 1
    dd GDT_start


[bits 32]
start_protected_mode:
    mov al, 'A'
    mov ah, 0x0f
    mov [0xb8000], ax
    jmp $

; finish the boot sector
times 510 - ($ - $$) db 0x00
db 0x55, 0xaa

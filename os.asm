mov ah, 0x0e
mov al, 'a'

print_upper:
    sub al, 32
    int 0x10
    jmp print_lower

print_lower:
    add al, 32
    int 0x10
    inc al
    cmp al, 'z' + 1
    jl print_upper
    jmp exit


exit:
    ; halt the os and create the boot sector
    jmp $
    times 510 - ($ - $$) db 0
    db 0x55, 0xaa

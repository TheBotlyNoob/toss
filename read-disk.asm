[org 0x7c00]

; TODO: Error messages

; entrypoint of the OS
main:
    mov [BOOT_DISK], dl                 

    ; setting up the stack

    xor ax, ax                          
    mov es, ax
    mov ds, ax
    mov bp, 0x8000
    mov sp, bp

    mov bx, 0x7e00

    ; reading the disk

    mov ah, 2
    mov al, 1
    mov ch, 0
    mov dh, 0
    mov cl, 2
    mov dl, [BOOT_DISK]
    int 0x13

    cmp ah, 0
    jne error

    call print_string
    jmp $

error:
    mov bx, ERROR_MESSAGE

    call print_string
    jmp $

; prints `NUL` terminated string in `bx` to the screen
print_string:
        mov ah, 0x00e
        l:
            mov al, [bx]

            cmp al, 0
            je after_l

            int 0x10

            inc bx
            jmp l

        after_l:
            ret

BOOT_DISK: db 0x00

ERROR_MESSAGE:
    db "unknown error reading disk.", 0

; finish the boot sector
times 510 - ($ - $$) db 0x00
db 0x55, 0xaa

AFTER_BOOT_SECTOR:
    db "Hello, World!"

    ; finish the sector
    times 512 - ($ - AFTER_BOOT_SECTOR) db 0x00

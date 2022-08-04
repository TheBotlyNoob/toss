[org 0x7c00]


; initialize the stack
mov bp, 0x8000
mov sp, bp

mov cx, 0

; entrypoint of the OS
main:
    mov ah, 0x00
    int 0x16
    cmp al, `\r`
    je on_newline
    mov ah, 0x00e
    int 0x10
    push ax
    inc cx
    call _printAllReg
    jmp main

on_newline:
    call newline
    times 2 add sp, cx ; move to start of string
    call print_stack
    jmp main

print_stack:
    sub sp, 2
    pop ax
    dec cx
    mov ah, 0x00e
    call _printAllReg
    ; int 0x10
    cmp cx, 0
    jne print_stack
    ret

newline:
    pusha
    mov ah, 0x00e
    mov al, `\r`
    int 0x10
    mov al, `\n`
    int 0x10
    popa
    ret

_printAllReg:
    pusha
    mov ah, 0x0e
    mov bx, labelRegs
printLabels:
    mov al, [bx]
    cmp al, 0
    je zeroESI
    int 0x10
    inc bx
    jmp printLabels
zeroESI:
    xor esi, esi
popReg:
    inc esi
    cmp esi, 9
    je end
    pop ax
__printInt:
    mov bx, 10
    xor cx, cx
stackIntDigits:
    xor dx, dx
    div bx
    add dx, 48
    push dx
    inc cx
    cmp ax, 0
    jne stackIntDigits
printStack:
    pop ax
    mov ah, 0x0e
    int 0x10
    dec cx
    cmp cx, 0
    jne printStack
printSpaceThenLoop:
    mov al, 32
    int 0x10
    jmp popReg

end:
    ret

labelRegs:
    db 10, 13, "DI: SI: BP: SP: BX: DX: CX: AX:", 10, 13, 0

; finish the boot sector
times 510 - ($ - $$) db 0x00
db 0x55, 0xaa

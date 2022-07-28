#!/usr/bin/sh

nasm "$1".asm -o os
qemu-system-x86_64 -nodefaults -boot order=c -vga std -drive file=os,format=raw,index=0,media=disk

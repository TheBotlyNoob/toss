/* taken from https://github.com/rust-osdev/bootloader */

ENTRY(_start)

/* FIXME: use this linker script instead of `objcopy` to turn the elf into a flat binary */

SECTIONS {
    . = 0x500;
    _stack_start = .;
    . = 0x7c00;
    _stack_end = .;

    .boot :
    {
        *(.boot .boot.*)
    }
    .text :
    {
        *(.text .text.*)
    }
    .rodata :
    {
        *(.rodata .rodata.*)
    }
    .data :
    {
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.got .got.*)
    }

    . = _stack_end + 510;

    .magic_number :
    {
        SHORT(0xaa55)       /* magic number for bootable disk */
    }

    _end_first_stage = .;

    .after :
    {
        KEEP(*(.after .after.*))
    }

    . = _end_first_stage + 510;
    .end :
    {
        SHORT(0x0000)        /* end of disk */
    }
}

#![no_std]
#![no_main]

use core::arch::asm;

/// Prints the given string to the screen.
fn print_char(c: char) {
    unsafe {
        asm!(
            "int 0x10",
            in("al") c as u8, // AL = character to print
            in("ah") 0x00e_u8, // TTY mode
            options(nostack, nomem)
        )
    }
}
fn print_str(s: &str) {
    for c in s.chars() {
        print_char(c);
    }
}

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    print_str("Ayy, it works!");

    halt()
}

/// Halts CPU execution indefinitely. We cannot continue execution after this point.
fn halt() -> ! {
    unsafe { asm!("hlt", options(noreturn, nomem, nostack)) };
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    halt()
}

#![no_std]
#![no_main]

use core::{arch::asm, hint::unreachable_unchecked};

#[inline(always)]
fn print_char(c: char) {
    unsafe {
        asm!("int 0x10", in("al") c as u8, in("ah") 0x00e_u8, options(nostack));
    }
}

fn get_char() -> char {
    let c: u8;
    unsafe {
        asm!("int 0x16", in("ah") 0x00_u8, out("al") c, options(nostack));
    }
    c as char
}

#[no_mangle]
extern "C" fn _start() -> ! {
    loop {
        let c = get_char();
        if c == '\r' {
            print_char('\r');
            print_char('\n');
        } else if c == '\x08' {
            print_char('\x08');
            print_char(' ');
            print_char('\x08');
        } else {
            print_char(c);
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { unreachable_unchecked() }
}

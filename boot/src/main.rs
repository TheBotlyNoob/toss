#![no_std]
#![no_main]

#[no_mangle]
static mut DISK_NUM: u8 = 0;

use core::{arch::asm, hint::unreachable_unchecked};

fn print_num(mut num: u32) {
    let mut buf = [0; 10];
    for c in buf.iter_mut().rev() {
        *c = (num % 10) as u8 + b'0';

        num /= 10;
        if num == 0 {
            break;
        }
    }
    for c in buf.iter().filter(|c| **c != 0) {
        print_char(*c as char);
    }
}

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
    unsafe {
        asm!("mov {}, dl", out(reg_byte) DISK_NUM, options(nostack));
    }

    print_num(u32::MAX);

    loop {
        unsafe {
            asm!("hlt", options(nostack));
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { unreachable_unchecked() }
}

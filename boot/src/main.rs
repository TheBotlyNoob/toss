#![no_std]
#![no_main]

use core::{arch::asm, hint::unreachable_unchecked};

extern "C" {
    static _stack_end: usize;
    static _stack_start: usize;
}

macro_rules! entry {
    ($fn:expr) => {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            let f: fn() -> ! = $fn;
            f()
        }
    };
}

entry!(main);

fn main() -> ! {
    let disk: u8;
    unsafe {
        asm!("mov dl, {}", out(reg_byte) disk);
    }

    setup_stack();
    print_char('J');

    let ptr = read_disk(disk);
    print_char('K');
    print_char(unsafe { *ptr.add(1) as char });

    unsafe {
        asm!("hlt", options(nostack, noreturn));
    }
}

fn print_char(c: char) {
    unsafe {
        asm!("int 0x10", in("al") c as u8, in("ah") 0x00e_u8, options(nostack));
    }
}

fn setup_stack() {
    unsafe {
        asm!(
            "xor ax, ax",
            "mov es, ax",
            "mov ds, ax",
            "mov sp, {stack_start}",
            "mov bp, sp",
            "mov bx, {stack_end}",
            stack_start = sym _stack_end,
            stack_end = sym _stack_start,
            options(nostack)
        );
    }
}

fn read_disk(disk: u8) -> *mut u8 {
    let ptr: usize;
    unsafe {
        asm!(
            "int 0x13",
            in("ah") 0x02_u8,
            in("al") 0x01_u8,
            in("ch") 0x00_u8,
            in("dh") 0x00_u8,
            in("cl") 0x02_u8,
            in("dl") disk,
            out("bx") ptr,
        );
    }
    ptr as *mut u8
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { unreachable_unchecked() }
}

#[link_section = ".after"]
#[no_mangle]
pub static HELLO: [u8; 12] = *b"Hello, world";

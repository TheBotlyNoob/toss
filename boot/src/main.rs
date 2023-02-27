#![no_std]
#![no_main]

mod disk;

use core::arch::asm;

macro_rules! entry {
    ($fn:expr) => {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            let disk: u8;
            unsafe {
                asm!("", out("dl") disk);
            }

            unsafe {
                asm!("cli", options(nostack)); // disable interrupts

                asm!(
                    "mov ds, {0:x}",
                    "mov es, {0:x}",
                    "mov ss, {0:x}",
                    "mov sp, {1:x}",
                    "mov bp, {1:x}",
                    in(reg) 0x00_u16,
                    in(reg) 0x7C00,
                ); // set up segments

                asm!("sti", options(nostack)); // enable interrupts
            }

            let f: fn(u8) -> ! = $fn;
            f(disk)
        }
    };
}

entry!(main);

fn main(disk: u8) -> ! {
    // unsafe {
    //     asm!(
    //         "int 0x10",
    //         in("ax") 0x0003, // AH=0x00, AL=0x03
    //     ); // clear screen
    // }
    // print_char('C');

    print_num(disk.into());

    // let (heads, sectors) = disk::get_disk_info(disk);

    hlt()
}

fn print_char(c: char) {
    unsafe {
        asm!(
            "int 0x10",
            in("al") c as u8,
            in("ah") 0x00e_u8,
            options(nostack)
        );
    }
}

fn print_num(mut n: u16) {
    // if n == 0 {
    //     print_char('0');
    //     return;
    // }

    let mut buf = [0; 5];
    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
    }
    for c in &buf[i..] {
        print_char(*c as _);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    print_char('P');
    unsafe {
        // emit a breakpoint exception
        asm!("int 0x03", options(nostack, noreturn));
    }
}

#[inline]
fn hlt() -> ! {
    unsafe {
        asm!("hlt", options(nostack, noreturn));
    }
}

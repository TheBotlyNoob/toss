#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{arch::asm, hint::unreachable_unchecked};

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() -> ! {
    asm!("mov al, 'A'", "int 0x10", options(noreturn));
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { unreachable_unchecked() }
}

#![no_main]
#![no_std]
#![feature(naked_functions)]

use core::arch::asm;

#[naked]
#[no_mangle]
unsafe extern "C" fn _start() {
    asm!("mov al, 'A'", "int 0x10", options(noreturn));
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#![no_std]
#![no_main]

use core::arch::asm;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    let disk_id: u8;
    unsafe { asm!("mov {}, dl", out(reg_byte) disk_id) };

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

use core::arch::asm;

/// Gets the disk info.
/// The first tuple element is the number of heads, the second is the number of sectors per track.
pub fn get_disk_info(disk: u8) -> (u8, u8) {
    let heads: u8;
    let sectors: u8;
    unsafe {
        asm!(
            "int 0x13",
            in("ah") 0x08_u8,
            in("dl") disk,
            out("dh") heads,
            out("cl") sectors,
            // options(pure, nostack, nomem)
        );
    };

    (heads + 1, sectors & 0x3f)
}

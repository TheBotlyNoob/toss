use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", env!("KERNEL_BINARY_PATH"));

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.stdout(Stdio::null()).stderr(Stdio::null());

    cmd.arg("-nodefaults");

    cmd.args(["-boot", "order=c"]);

    cmd.args(["-smp", "4"]);
    // Allocate some memory.
    cmd.args(["-m", "256M"]);
    // Map the QEMU exit signal to port f4.
    cmd.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]);
    cmd.args(["-vga", "std"]);
    cmd.args(["-accel", "tcg"]);
    cmd.args(["-serial", "stdio"]);
    cmd.arg("-drive").arg(format!(
        "format=raw,index=0,media=disk,file={}",
        env!("KERNEL_BINARY_PATH")
    ));
    // debugging
    cmd.args(["-s", "-S"]);

    cmd.spawn()?;

    Ok(())
}

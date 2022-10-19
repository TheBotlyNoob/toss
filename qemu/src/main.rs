use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Kernel file: {}", env!("CARGO_BIN_FILE_kernel"));

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit());

    cmd.arg("-nodefaults");

    cmd.args(["-boot", "order=c"]);

    // Use a modern machine.
    cmd.args(["-machine", "q35"]);
    // Multi-processor services protocol test needs exactly 4 CPUs.
    cmd.args(["-smp", "4"]);
    // Allocate some memory.
    cmd.args(["-m", "256M"]);
    // Map the QEMU exit signal to port f4.
    cmd.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]);
    cmd.args(["-vga", "std"]);
    cmd.args([
        "-accel",
        if cfg!(target_os = "windows") {
            "tcg"
        } else {
            "kvm"
        },
    ]);
    cmd.args(["-serial", "stdio"]);
    cmd.arg("-drive").arg(format!(
        "format=raw,index=0,media=disk,file={}",
        env!("CARGO_BIN_FILE_kernel")
    ));

    cmd.spawn()?.wait()?;

    Ok(())
}

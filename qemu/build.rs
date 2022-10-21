use std::{env::var, path::Path, process::Command};

fn main() {
    let binary_path = var("CARGO_BIN_FILE_kernel").unwrap();
    let binary_path = Path::new(&binary_path);

    let flat_binary_path = binary_path.with_extension("bin");

    let llvm_tools = llvm_tools::LlvmTools::new().expect("failed to get llvm tools");
    let objcopy = llvm_tools
        .tool(&llvm_tools::exe("llvm-objcopy"))
        .expect("LlvmObjcopyNotFound");

    // convert first stage to binary
    let mut cmd = Command::new(objcopy);
    cmd.arg("-I").arg("elf64-x86-64");
    cmd.arg("-O").arg("binary");
    cmd.arg("--binary-architecture=i386:x86-64");
    cmd.arg(binary_path);
    cmd.arg(&flat_binary_path);

    let status = cmd.spawn().unwrap().wait().unwrap();
    if !status.success() {
        panic!("objcopy failed");
    }

    println!(
        "cargo:rustc-env=KERNEL_BINARY_PATH={}",
        flat_binary_path.display()
    );
}

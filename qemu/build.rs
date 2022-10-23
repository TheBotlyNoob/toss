use std::{env::var, path::Path, process::Command};

fn main() {
    let workspace_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = Path::new(&workspace_dir).parent().unwrap();

    println!(
        "cargo:rerun-if-changed={}",
        workspace_dir.join("boot").display()
    );

    let cargo = var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let out_dir = var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let mut cmd = Command::new(cargo);

    cmd.env("CARGO_TARGET_DIR", out_dir.join("target"));
    cmd.arg("build");
    cmd.arg("--target")
        .arg(workspace_dir.join("boot-sector.json"));
    cmd.arg("--release");
    cmd.arg("--manifest-path")
        .arg(workspace_dir.join("boot").join("Cargo.toml"));
    cmd.arg("-Zbuild-std=core,panic_abort")
        .arg("-Zbuild-std-features=compiler-builtins-mem");

    cmd.env_remove("RUSTFLAGS");
    cmd.env_remove("CARGO_ENCODED_RUSTFLAGS");
    cmd.env_remove("RUSTC_WORKSPACE_WRAPPER"); // used by clippy

    println!("{:?}\n{}", cmd, out_dir.display());

    assert!(cmd.status().unwrap().success());

    let binary_path = out_dir
        .join("target")
        .join("boot-sector")
        .join("release")
        .join("boot");

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

    assert!(cmd.status().unwrap().success());

    println!(
        "cargo:rustc-env=KERNEL_BINARY_PATH={}",
        flat_binary_path.display()
    );
}

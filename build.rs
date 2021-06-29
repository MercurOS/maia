// Original source:
// https://github.com/rust-osdev/bootloader/blob/c09f94f1fe96e21909437c7cfec33f5b4bb449fa/build.rs
//
// Copyright (c) 2018 Philipp Oppermann
// Copyright (c) 2021 Henry Carlson

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::{self, Command},
};

fn main() {
    let out_dir = PathBuf::from(
        env::var("OUT_DIR").expect("OUT_DIR not set")
    );
    let kernel_path = PathBuf::from(
        env::var("KERNEL").expect("set env variable KERNEL to kernel path")
    );

    let kernel_file_name = kernel_path
        .file_name().expect("KERNEL has no valid file name")
        .to_str().expect("kernel path is not valid utf8");

    println!("cargo:rerun-if-changed=src/arch/riscv/riscv64-efi.ld");
    println!(
        "cargo:rerun-if-changed={}",
        kernel_path.clone().into_os_string().into_string()
            .expect("kernel path is not valid utf8")
    );

    // get access to llvm tools shipped in the llvm-tools-preview rustup component
    let llvm_tools = match llvm_tools::LlvmTools::new() {
        Ok(tools) => tools,
        Err(llvm_tools::Error::NotFound) => {
            eprintln!("Error: llvm-tools not found");
            eprintln!("Maybe the rustup component `llvm-tools-preview` is missing?");
            eprintln!("  Install it through: `rustup component add llvm-tools-preview`");
            process::exit(1);
        }
        Err(err) => {
            eprintln!("Failed to retrieve llvm-tools component: {:?}", err);
            process::exit(1);
        }
    };

    // strip debug symbols from kernel for faster loading
    let stripped_kernel_file_name = format!("kernel_stripped-{}", kernel_file_name);
    let stripped_kernel = out_dir.join(&stripped_kernel_file_name);
    let objcopy = llvm_tools
        .tool(&llvm_tools::exe("llvm-objcopy"))
        .expect("llvm-objcopy not found in llvm-tools");
    let mut cmd = Command::new(&objcopy);
    cmd.arg("--strip-debug");
    cmd.arg(&kernel_path);
    cmd.arg(&stripped_kernel);
    let exit_status = cmd
        .status()
        .expect("failed to run objcopy to strip debug symbols");
    if !exit_status.success() {
        eprintln!("Error: Stripping debug symbols failed");
        process::exit(1);
    }

    // write file for including kernel in binary
    let file_path = out_dir.join("kernel_info.rs");
    let mut file = File::create(file_path).expect("failed to create kernel_info.rs");
    let kernel_size = fs::metadata(&stripped_kernel)
        .expect("Failed to read file metadata of stripped kernel")
        .len();
    file.write_all(
        format!(
            "const KERNEL_SIZE: usize = {}; const KERNEL_BYTES: [u8; KERNEL_SIZE] = *include_bytes!(r\"{}\");",
            kernel_size,
            stripped_kernel.display(),
        ).as_bytes(),
    ).expect("write to kernel_info.rs failed");
}

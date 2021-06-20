fn main() {
    println!("cargo:rerun-if-changed=src/arch/riscv/riscv64-efi.ld");
}

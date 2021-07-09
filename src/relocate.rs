use super::EfiStatus;
use super::elf::dynamic::{Dynamic, R_RISCV_RELATIVE};

#[inline(always)]
pub unsafe fn relocate(
    base_address: *const core::ffi::c_void,
    elf_dyn: *const core::ffi::c_void,
) -> EfiStatus {
    let rel_table = Dynamic::find_relocations_inner(base_address, elf_dyn as *const Dynamic);
    match rel_table {
        Ok(Some(rel_table)) => {
            // apply relocations
            rel_table.fold_inner(EfiStatus::success(), |result, entry| {
                match entry.info {
                    R_RISCV_RELATIVE => {
                        let address = base_address.add(entry.offset) as *mut u64;
                        let value = base_address.offset(entry.addend as isize) as u64;

                        address.write(value);

                        result
                    },
                    _ => EfiStatus::load_error(),
                }
            })
        },
        Ok(None) => EfiStatus::success(),
        Err(_) => EfiStatus::load_error(),
    }
}

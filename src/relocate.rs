use super::EfiStatus;
use super::elf::dynamic::{
    Dynamic,
    ElfRela,
    DT_NULL,
    DT_RELA,
    DT_RELASZ,
    DT_RELAENT,
    R_RISCV_RELATIVE,
};

#[inline(always)]
pub unsafe fn relocate(
    base_address: *const core::ffi::c_void,
    elf_dyn: *const core::ffi::c_void,
) -> EfiStatus {
    let elf_dyn = elf_dyn as *const Dynamic;

    let mut rel_addr = core::ptr::null();
    let mut rel_size: usize = 0;
    let mut rel_entry_size: usize = 0;

    // find relocation table
    let mut i: usize = 0;
    loop {
        let entry = & *elf_dyn.add(i);

        match entry.tag {
            DT_NULL => break,
            DT_RELA => {
                rel_addr = base_address.add(entry.val as usize);
            },
            DT_RELASZ => {
                rel_size = entry.val as usize;
            },
            DT_RELAENT => {
                rel_entry_size = entry.val as usize;
            },
            _ => (),
        }

        i += 1;
    }

    if rel_addr.is_null() && rel_size == 0 {
        return EfiStatus::success();
    }
    if rel_addr.is_null() {
        return EfiStatus::load_error();
    }
    if rel_size == 0 {
        return EfiStatus::load_error();
    }

    // apply relocations
    let rel_end = rel_addr.add(rel_size);
    loop {
        if rel_addr >= rel_end {
            break;
        }

        let entry = & *(rel_addr as *const ElfRela);

        match entry.info {
            R_RISCV_RELATIVE => {
                let address = base_address.add(entry.offset) as *mut u64;
                let value = base_address.offset(entry.addend as isize) as u64;

                address.write(value);
            },
            _ => return EfiStatus::load_error(),
        }

        rel_addr = rel_addr.add(rel_entry_size);
    }

    EfiStatus::success()
}

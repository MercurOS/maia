#![allow(dead_code)]

use super::ElfError;

#[repr(C)]
pub struct Dynamic {
    pub tag: DynamicTagType,
    pub val: u64,
}

impl Dynamic {
    // FIXME: limit visibility to elf module and crate::relocate
    #[inline(always)]
    pub unsafe fn find_relocations_inner(
        base_address: *const core::ffi::c_void,
        dynamic: *const Dynamic,
    ) -> Result<Option<RelocationTable>, ElfError> {
        let mut rel_addr = core::ptr::null();
        let mut rel_size: usize = 0;
        let mut rel_entry_size: usize = 0;

        let mut next_entry = dynamic;
        loop {
            let entry = & *next_entry;
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
            };
            next_entry = next_entry.add(1);
        }

        if rel_addr.is_null() && rel_size == 0 {
            return Ok(None);
        }
        if rel_addr.is_null() || rel_size == 0 {
            return Err(ElfError::InvalidFormat);
        }

        Ok(Some(RelocationTable {
            address: rel_addr,
            size: rel_size,
            entry_size: rel_entry_size,
        }))
    }
}

pub type DynamicTagType = i64;
pub const DT_NULL: DynamicTagType = 0;
pub const DT_RELA: DynamicTagType = 7;
pub const DT_RELASZ: DynamicTagType = 8;
pub const DT_RELAENT: DynamicTagType = 9;

pub struct RelocationTable {
    address: *const core::ffi::c_void,
    size: usize,
    entry_size: usize,
}

impl RelocationTable {
    #[inline(always)]
    pub fn fold_inner<'a, B, F>(&'a self, init: B, mut f: F) -> B
    where
        F: FnMut(B, &'a ElfRela) -> B,
    {
        let rela_end = unsafe { self.address.add(self.size) };
        let mut address = self.address;
        let mut acc = init;
        loop {
            if address >= rela_end {
                break;
            }
            acc = f(acc, unsafe { & *(address as *const ElfRela) });
            address = unsafe { address.add(self.entry_size) };
        }

        acc
    }
}

impl<'a> core::iter::IntoIterator for &'a RelocationTable {
    type IntoIter = RelocationTableIterator<'a>;
    type Item = &'a ElfRela;

    fn into_iter(self) -> Self::IntoIter {
        RelocationTableIterator {
            relocation_table: self,
            next_entry: self.address,
        }
    }
}

pub struct RelocationTableIterator<'a> {
    relocation_table: &'a RelocationTable,
    next_entry: *const core::ffi::c_void,
}

impl<'a> core::iter::Iterator for RelocationTableIterator<'a> {
    type Item = &'a ElfRela;

    fn next(&mut self) -> Option<Self::Item> {
        let rela_end = unsafe { self.relocation_table.address.add(self.relocation_table.size) };
        if self.next_entry >= rela_end {
            return None;
        }

        let item = unsafe { & *(self.next_entry as *const ElfRela) };
        self.next_entry = unsafe { self.next_entry.add(self.relocation_table.entry_size) };

        Some(item)
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.relocation_table.fold_inner(init, f)
    }
}

#[repr(C)]
pub struct ElfRela {
    pub offset: usize,
    pub info: RelaInfoType,
    pub addend: i64,
}

pub type RelaInfoType = u64;
pub const R_RISCV_64: RelaInfoType = 2;
pub const R_RISCV_RELATIVE: RelaInfoType = 3;

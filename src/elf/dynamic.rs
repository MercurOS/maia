#![allow(dead_code)]

#[repr(C)]
pub struct Dynamic {
    pub tag: DynamicTagType,
    pub val: u64,
}

pub type DynamicTagType = i64;
pub const DT_NULL: DynamicTagType = 0;
pub const DT_RELA: DynamicTagType = 7;
pub const DT_RELASZ: DynamicTagType = 8;
pub const DT_RELAENT: DynamicTagType = 9;

#[repr(C)]
pub struct ElfRela {
    pub offset: usize,
    pub info: RelaInfoType,
    pub addend: i64,
}

pub type RelaInfoType = u64;
pub const R_RISCV_64: RelaInfoType = 2;
pub const R_RISCV_RELATIVE: RelaInfoType = 3;

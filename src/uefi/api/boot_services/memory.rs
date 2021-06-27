use super::super::{
    EfiStatus,
    EfiPhyiscalAddress,
    EfiVirtualAddress,
};

pub type EfiAllocatePages = extern "efiapi" fn(
    r#type: EfiAllocateType,
    memory_type: EfiMemoryType,
    pages: usize,
    memory: *mut EfiPhyiscalAddress,
) -> EfiStatus;

pub type EfiAllocateType = u32;
pub const ALLOCATE_ANY_PAGES: EfiAllocateType = 0;
pub const ALLOCATE_MAX_ADDRESS: EfiAllocateType = 1;
pub const ALLOCATE_ADDRESS: EfiAllocateType = 2;

pub type EfiMemoryType = u32;
pub const EFI_LOADER_DATA: EfiMemoryType = 2;

pub type EfiAllocatePool = extern "efiapi" fn(
    pool_type: EfiMemoryType,
    size: usize,
    memory: *mut *const core::ffi::c_void,
) -> EfiStatus;

pub type EfiGetMemoryMap = extern "efiapi" fn(
    memory_map_size: *mut usize,
    memory_map: *mut EfiMemoryDescriptor,
    map_key: *mut usize,
    descriptor_size: *mut usize,
    descriptor_version: *mut u32,
) -> EfiStatus;

#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub r#type: u32,
    pub physical_start: EfiPhyiscalAddress,
    pub virtual_start: EfiVirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

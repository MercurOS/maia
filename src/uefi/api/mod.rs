// External interface definitions for the UEFI API

pub mod protocols;
pub mod boot_services;
pub mod system;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq)]
pub struct Boolean(u8);

pub type Char16 = u16;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EfiStatus(usize);

pub type EfiHandle = *mut core::ffi::c_void;

pub type EfiEvent = *mut core::ffi::c_void;

pub type EfiPhyiscalAddress = u64;
pub type EfiVirtualAddress = u64;

pub type EfiGuid = [u8; 16];

#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    _reserved: u32,
}

impl Boolean {
    pub const TRUE: Boolean = Boolean(1u8);
    pub const FALSE: Boolean = Boolean(0u8);
}

impl From<Boolean> for bool {
    fn from(uefi_bool: Boolean) -> Self {
        uefi_bool.0 != 0
    }
}

impl PartialEq for Boolean {
    fn eq(&self, other: &Boolean) -> bool {
        bool::from(*self) == bool::from(*other)
    }
}

impl PartialEq<bool> for Boolean {
    fn eq(&self, other: &bool) -> bool {
        *other == (*self).into()
    }
}

const EFI_ERROR: usize = 0x8000_0000_0000_0000;

impl EfiStatus {
    pub fn success() -> EfiStatus {
        EfiStatus(0)
    }

    pub fn load_error() -> EfiStatus {
        EfiStatus(EFI_ERROR | 1)
    }

    pub fn is_ok(&self) -> bool {
        EFI_ERROR & self.0 == 0
    }
}

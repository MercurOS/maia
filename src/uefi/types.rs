use core::ffi::c_void;

pub type EfiHandle = *const c_void;
pub const EFI_HANDLE_SIZE: usize = 8;

#[repr(C)]
pub struct EfiStatus(pub usize);

const EFI_ERROR: usize = 0x8000_0000_0000_0000;

impl EfiStatus {
    pub fn success() -> EfiStatus {
        EfiStatus(0)
    }

    pub fn load_error() -> EfiStatus {
        EfiStatus(EFI_ERROR | 1)
    }

    pub fn is_error(&self) -> bool {
        0x8000_0000_0000_0000 & self.0 > 0
    }
}
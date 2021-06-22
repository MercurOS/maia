pub const SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;

#[repr(C)]
pub struct EfiSystemTable {
    pub hdr: super::EfiTableHeader,
    pub firmware_vendor: *const super::Char16,
    pub firmware_revision: u32,
    pub console_in_handle: super::EfiHandle,
    pub console_in: *mut super::protocols::SimpleTextInputProtocol,
    pub console_out_handle: super::EfiHandle,
    pub console_out: *mut super::protocols::SimpleTextOutputProtocol,
    pub console_error_handle: super::EfiHandle,
    pub console_error: *mut super::protocols::SimpleTextOutputProtocol,
    _runtime_services: *const core::ffi::c_void,
    pub boot_services: *mut super::boot_services::EfiBootServices,
}

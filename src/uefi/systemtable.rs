use super::{
    types::{EfiHandle, EfiTableHeader},
    protocols::{SimpleTextInputProtocol, SimpleTextOutputProtocol},
};

static SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;

#[repr(C)]
pub struct EfiSystemTable {
    pub header: EfiTableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    pub console_in: &'static mut SimpleTextInputProtocol,
    pub console_out_handle: EfiHandle,
    pub console_out: &'static mut SimpleTextOutputProtocol,
    pub console_error_handle: EfiHandle,
    pub console_error: &'static mut SimpleTextOutputProtocol,
    pub runtime_services: EfiHandle,
    pub boot_services: EfiHandle,
}

impl EfiSystemTable {
    pub fn verify_signature(&self) -> bool {
        self.header.signature == SYSTEM_TABLE_SIGNATURE
    }
}

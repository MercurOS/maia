use crate::uefi::api;

#[repr(C)]
pub struct SimpleTextInputProtocol {
    pub reset: extern "efiapi" fn (
        this: *const SimpleTextInputProtocol,
        extended_verification: api::Boolean,
    ) -> api::EfiStatus,
    pub read_key_stroke: extern "efiapi" fn (
        this: *const SimpleTextInputProtocol,
        key: *mut InputKey,
    ) -> api::EfiStatus,
    _wait_for_key: api::EfiEvent,
}

#[repr(C)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: api::Char16,
}

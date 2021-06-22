use crate::uefi::api;

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: extern "efiapi" fn(
        this: *const SimpleTextOutputProtocol,
        extended_verification: api::Boolean,
    ) -> api::EfiStatus,
    pub output_string: extern "efiapi" fn(
        this: *const SimpleTextOutputProtocol,
        string: *const api::Char16,
    ) -> api::EfiStatus,
    _test_string: *const core::ffi::c_void,
    _query_mode: *const core::ffi::c_void,
    _set_mode: *const core::ffi::c_void,
    _set_attribute: *const core::ffi::c_void,
    pub clear_screen: extern "efiapi" fn(
        this: *const SimpleTextOutputProtocol,
    ) -> api::EfiStatus,
    _set_cursor_position: *const core::ffi::c_void,
    _enable_cursor: *const core::ffi::c_void,
    _mode: *const core::ffi::c_void,
}

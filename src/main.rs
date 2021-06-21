#![no_std]
#![no_main]

#![feature(global_asm)]

use core::panic::PanicInfo;
use core::ffi::c_void;

pub mod assembly;
mod uefi;

use uefi::{
    EfiHandle,
    EfiStatus,
    EfiSystemTable,
};

#[no_mangle]
pub extern "C" fn relocate(
    base_address: *const c_void,
    efi_dyn: *const c_void,
) -> EfiStatus {
    EfiStatus::success()
}

#[no_mangle]
pub extern "C" fn efi_main(
    _image_handle: EfiHandle,
    system_table: &'static mut EfiSystemTable,
) -> EfiStatus {
    system_table.console_out.write_string(
        "/// MercurOS Maia Bootloader ///\r\n"
    );

    EfiStatus::load_error()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

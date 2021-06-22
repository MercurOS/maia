#![no_std]
#![no_main]

#![feature(abi_efiapi)]
#![feature(global_asm)]

use core::panic::PanicInfo;
use core::ffi::c_void;

pub mod assembly;
mod uefi;

use uefi::EfiStatus;

#[no_mangle]
pub extern "C" fn relocate(
    base_address: *const c_void,
    efi_dyn: *const c_void,
) -> EfiStatus {
    EfiStatus::success()
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::EfiHandle,
    system_table: *mut uefi::EfiSystemTable,
) -> EfiStatus {
    let mut uefi = unsafe {
        uefi::Application::from(image_handle, system_table)
    };

    uefi::Console::write_string(
        &mut uefi,
        "/// MercurOS Maia Bootloader ///\r\n"
    );

    EfiStatus::load_error()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

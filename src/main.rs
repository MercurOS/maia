#![no_std]
#![no_main]

#![feature(abi_efiapi)]
#![feature(global_asm)]

use core::panic::PanicInfo;
use core::ffi::c_void;

pub mod assembly;
pub mod kernel;

mod boot;
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
    let uefi = unsafe {
        uefi::Application::from(image_handle, system_table)
    };

    match uefi {
        Some(uefi) => boot::boot(uefi),
        None => EfiStatus::load_error(),
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

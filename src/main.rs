#![no_std]
#![no_main]

#![feature(abi_efiapi)]
#![feature(asm)]
#![feature(global_asm)]

use core::panic::PanicInfo;
use core::ffi::c_void;

pub mod assembly;
pub mod kernel;

mod boot;
mod elf;
mod relocate;
mod uefi;

use uefi::EfiStatus;

#[no_mangle]
pub extern "C" fn relocate(
    base_address: *const c_void,
    elf_dyn: *const c_void,
) -> EfiStatus {
    unsafe { relocate::relocate(base_address, elf_dyn) }
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::EfiHandle,
    system_table: *mut uefi::EfiSystemTable,
) -> EfiStatus {
    let uefi = unsafe {
        uefi::Application::from(image_handle, system_table)
    };

    if let Some(uefi) = uefi {
        boot::boot(uefi)
    } else {
        EfiStatus::load_error()
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

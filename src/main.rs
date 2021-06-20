#![no_std]
#![no_main]

#![feature(global_asm)]

use core::panic::PanicInfo;
use core::ffi::c_void;

pub mod assembly;
mod uefi;

use uefi::types::EfiStatus;

#[no_mangle]
pub extern "C" fn relocate(
    base_address: *const c_void,
    efi_dyn: *const c_void,
) -> usize {
    // EFI_LOAD_ERROR
    0x8000_0000_0000_0001
}

#[no_mangle]
pub extern "C" fn efi_main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

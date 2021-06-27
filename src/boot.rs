use super::uefi;

pub fn boot(mut uefi: uefi::Application) -> uefi::EfiStatus {
    uefi::Console::clear_screen(&mut uefi);
    uefi::Console::write_string(
        &mut uefi,
        "/// MercurOS Maia Bootloader ///\r\n"
    );

    let _memory_map = match uefi::Memory::get_memory_map(&mut uefi) {
        Ok(memory_map) => memory_map,
        Err(uefi::UEFIError::MemoryAllocationFailed) => {
            uefi::Console::write_string(&mut uefi, "MemoryAllocationFailed\r\n");
            return uefi::EfiStatus::load_error();
        },
        Err(uefi::UEFIError::MemoryMapUnavailable) => {
            uefi::Console::write_string(&mut uefi, "MemoryMapUnavailable\r\n");
            return uefi::EfiStatus::load_error();
        },
    };

    uefi::EfiStatus::success()
}

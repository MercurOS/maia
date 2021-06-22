use super::uefi;

pub fn boot(mut uefi: uefi::Application) -> uefi::EfiStatus {
    uefi::Console::write_string(
        &mut uefi,
        "/// MercurOS Maia Bootloader ///\r\n"
    );

    uefi::EfiStatus::success()
}

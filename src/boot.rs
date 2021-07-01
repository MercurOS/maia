use core::fmt::Write;

use super::{elf, kernel, uefi};

pub enum Error {
    MemoryAllocationFailed,
    MemoryMapUnavailable,
    InvalidKernelImage,
}

impl core::convert::From<Error> for uefi::EfiStatus {
    fn from(error: Error) -> uefi::EfiStatus {
        match error {
            Error::MemoryAllocationFailed =>
                uefi::EfiStatus::out_of_resources(),
            _ =>
                uefi::EfiStatus::load_error(),
        }
    }
}

impl core::convert::From<uefi::UEFIError> for Error {
    fn from(error: uefi::UEFIError) -> Error {
        match error {
            uefi::UEFIError::MemoryAllocationFailed =>
                Error::MemoryAllocationFailed,
            _ =>
                Error::MemoryMapUnavailable,
        }
    }
}

impl core::convert::From<elf::ElfError> for Error {
    fn from(error: elf::ElfError) -> Error {
        Error::InvalidKernelImage
    }
}

pub fn boot(mut uefi: uefi::Application) -> Result<(), Error> {
    uefi::Console::clear_screen(&mut uefi);
    uefi::Console::write_string(
        &mut uefi,
        "MercurOS Maia Bootloader\r\n"
    );

    let (entry_point, memory_map) =
         load_kernel(&mut uefi)
            .and_then(|entry_point| {
                if entry_point.is_null() {
                    uefi::Console::write_string(
                        &mut uefi,
                        "Unable to determine entry point!\r\n"
                    );
                    return Err(Error::InvalidKernelImage);
                }

                uefi::Memory::get_memory_map(&mut uefi)
                    .map(|memory_map| (entry_point, memory_map))
                    .map_err(|error| error.into())
            })
            .map_err(|error| {
                uefi::Console::write_string(
                    &mut uefi,
                    match error {
                        Error::MemoryAllocationFailed =>
                            "Memory allocation failed!\r\n",
                        Error::MemoryMapUnavailable =>
                            "Memory map unavailable!\r\n",
                        Error::InvalidKernelImage =>
                            "Invalid kernel image!\r\n",
                    }
                );
                error
            })?;

    uefi::Console::write_string(&mut uefi, "\r\nBooting to OS\r\n");
    if uefi::Image::exit_boot_services(uefi, &memory_map).is_err() {
        // Unfortunately, we currently cannot handle errors here.
        // UEFI boot services are in an indeterminate state so we cannot
        // return either...
        loop {}
    }

    // Jump to kernel
    unsafe {
        asm!(
            "jalr ra, 0({0})",
            in(reg) entry_point,
            out("ra") _,
        );
    }

    loop {}
}

#[inline(always)]
fn load_kernel(uefi: &mut uefi::Application)
    -> Result<*const core::ffi::c_void, Error>
{
    // TODO: Currently segment.virtual_address values are ignored,
    // and segments are placed in random memory locations.
    // Essentially, the loaded kernel will be unable to reference
    // anything contained in the ELF image at all (outside of relative
    // offsets within the entry point segment).

    let kernel_elf = unsafe {
        elf::ElfFile::from_buffer(&kernel::KERNEL.borrow()[..])
    };

    let mut entry_point: *const core::ffi::c_void = core::ptr::null();

    match kernel_elf {
        Ok(kernel_elf) => {
            uefi::Console::write_string(uefi, "Loading kernel...\r\n");

            let virtual_entry = kernel_elf.header().get_entry_point();

            #[cfg(debug_kernel)]
            {
                uefi.write_fmt(format_args!(
                    "Entry point (virtual address): {:#018x}\r\n",
                    virtual_entry
                ));

                uefi.write_fmt(format_args!(
                    "Segment count: {}\r\n",
                     kernel_elf.header().get_program_header_info().entry_count,
                ));
            }

            let iter = kernel_elf.program_headers()
                .map_err(|_| Error::InvalidKernelImage)?;

            for program_header in iter {
                #[cfg(debug_kernel)]
                {
                    uefi::Console::write_string(uefi, "Segment:\r\n");
                    uefi.write_fmt(format_args!(
                        "vaddr: {:#018x}\r\n",
                         program_header.get_virtual_address(),
                    ));
                    uefi.write_fmt(format_args!(
                        "filesz: {:#018x}\r\n",
                         program_header.get_file_size(),
                    ));
                }

                let virtual_size = program_header.get_memory_size();
                let mut page_count = virtual_size / 4096;

                // round up
                if virtual_size & 0xFFF > 0 {
                    page_count += 1;
                }

                let segment_data = kernel_elf.segment_data(program_header)
                    .map_err(|err| core::convert::Into::<Error>::into(err))?;
                if let Some(buffer) = uefi::Memory::allocate_pages(uefi, page_count) {
                    // TODO: Zeroing, optimization etc.

                    let (initial, _) = buffer.split_at_mut(segment_data.len());
                    initial.copy_from_slice(segment_data);

                    if virtual_entry >= program_header.get_virtual_address() &&
                            virtual_entry < program_header.get_virtual_address() + virtual_size
                    {
                        let entry_offset = virtual_entry - program_header.get_virtual_address();
                        entry_point = &buffer[entry_offset]
                            as *const u8
                            as *const core::ffi::c_void;
                    }
                } else {
                    return Err(Error::MemoryAllocationFailed);
                }
            }

            Ok(entry_point)
        },
        Err(_) => Err(Error::InvalidKernelImage),
    }
}

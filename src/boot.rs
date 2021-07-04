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
    fn from(_error: elf::ElfError) -> Error {
        Error::InvalidKernelImage
    }
}

pub fn boot(mut uefi: uefi::Application) -> Result<(), Error> {
    uefi::Console::clear_screen(&mut uefi);
    uefi::Console::write_string(
        &mut uefi,
        "MercurOS Maia Bootloader\r\n"
    );

    #[cfg(feature = "debug_mmap")]
    debug_mmap(&mut uefi)?;

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
            uefi::Console::write_string(uefi, "\r\nLoading kernel...\r\n");

            let virtual_entry = kernel_elf.header().get_entry_point();

            #[cfg(feature = "debug_kernel")]
            {
                uefi.write_fmt(format_args!(
                    "Entry point (virtual address): {:#018X}\r\n",
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
                if program_header.get_type() != Some(elf::SegmentType::Load) {
                    continue;
                }

                let virtual_address = program_header.get_virtual_address();
                let virtual_size = program_header.get_memory_size();
                let page_base = virtual_address & !(program_header.get_alignment() - 1);

                let mut page_count = (virtual_address + virtual_size - page_base) / 4096;
                // round up
                if virtual_size & 0xFFF > 0 {
                    page_count += 1;
                }

                #[cfg(feature = "debug_kernel")]
                {
                    uefi::Console::write_string(uefi, "\r\nSegment:\r\n");
                    uefi.write_fmt(format_args!(
                        "offset: {:#018x}\r\n",
                        program_header.get_offset(),
                    ));
                    uefi.write_fmt(format_args!(
                        "vaddr: {:#018x}\r\n",
                        program_header.get_virtual_address(),
                    ));
                    uefi.write_fmt(format_args!(
                        "memsz: {:#018x}\r\n",
                        program_header.get_memory_size(),
                    ));

                    uefi.write_fmt(format_args!(
                        "Allocating {} page(s) at {:#018X}\r\n",
                        page_count,
                        page_base,
                    ));
                }

                let segment_data = kernel_elf.segment_data(program_header)
                    .map_err(|err| core::convert::Into::<Error>::into(err))?;

                let buffer = uefi::Memory::allocate_pages_at(uefi, page_base as u64, page_count);
                if let Some(buffer) = buffer {
                    let (_padding, rest) = buffer.split_at_mut(virtual_address - page_base);
                    let (virtual_data, _padding) = rest.split_at_mut(segment_data.len());
                    virtual_data.copy_from_slice(segment_data);

                    // TODO: Zeroing

                    if virtual_entry >= virtual_address &&
                            virtual_entry < virtual_address + virtual_size
                    {
                        let entry_offset = virtual_entry - page_base;
                        entry_point = &buffer[entry_offset]
                            as *const u8
                            as *const core::ffi::c_void;
                    }
                } else {
                    return Err(Error::MemoryAllocationFailed);
                }
            }

            #[cfg(feature = "debug_kernel")]
            uefi.write_fmt(format_args!(
                "Kernel entry point in memory: {:#018X}\r\n",
                entry_point as usize,
            ));

            Ok(entry_point)
        },
        Err(_) => Err(Error::InvalidKernelImage),
    }
}

#[cfg(feature = "debug_mmap")]
fn debug_mmap(uefi: &mut uefi::Application) -> Result<(), Error> {
    let memory_map = uefi::Memory::get_memory_map(uefi)
        .map_err(|err| core::convert::Into::<Error>::into(err))?;

    uefi::Console::write_string(uefi, "\r\nMemory Map:\r\n");
    for descriptor in &memory_map {
        uefi.write_fmt(format_args!(
            "\r\n{:#018X} - {:#018X}: {}\r\n",
            descriptor.physical_start,
            descriptor.physical_start + descriptor.number_of_pages * 4096,
            match descriptor.r#type {
                uefi::api::boot_services::memory::EFI_RESERVED_MEMORY_TYPE => "EfiReservedMemoryType",
                uefi::api::boot_services::memory::EFI_LOADER_DATA => "EfiLoaderData",
                uefi::api::boot_services::memory::EFI_CONVENTIONAL_MEMORY => "EfiConventionalMemory",
                uefi::api::boot_services::memory::EFI_UNUSABLE_MEMORY => "EfiUnusableMemory",
                uefi::api::boot_services::memory::EFI_MEMORY_MAPPED_IO => "EfiMemoryMappedIO",
                _ => "",
            },
        ));
        uefi.write_fmt(format_args!("type: {:#010x}\r\n", descriptor.r#type));
        uefi.write_fmt(format_args!("virtual_start: {:#018x}\r\n", descriptor.virtual_start));
        uefi.write_fmt(format_args!("attribute: {:#018x}\r\n", descriptor.attribute));
    }

    Ok(())
}

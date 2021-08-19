use core::fmt::Write;

use super::{elf, kernel, uefi};

pub enum Error {
    MemoryAllocationFailed,
    MemoryMapUnavailable,
    DeviceTreeUnavailable,
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

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::MemoryAllocationFailed =>
                    "Memory allocation failed!",
                Error::MemoryMapUnavailable =>
                    "Memory map unavailable!",
                Error::DeviceTreeUnavailable =>
                    "DeviceTree not found!",
                Error::InvalidKernelImage =>
                    "Invalid kernel image!",
            }
        )
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

    let entry_point = load_kernel(&mut uefi, &kernel::KERNEL.borrow()[..])
        .map_err(|error| {
            uefi.write_fmt(format_args!("{}\r\n", error));
            error
        })?;

    if entry_point.is_null() {
        uefi::Console::write_string(
            &mut uefi,
            "Unable to determine entry point!\r\n"
        );
        return Err(Error::InvalidKernelImage);
    }

    let dtb = match uefi::Configuration::get_dtb(&mut uefi) {
        Some(dtb) => dtb,
        None => {
            uefi.write_fmt(format_args!("{}\r\n", Error::DeviceTreeUnavailable));
            return Err(Error::DeviceTreeUnavailable);
        },
    };

    let memory_map = uefi::Memory::get_memory_map(&mut uefi)
        .map_err(|error| {
            let error: Error = error.into();
            uefi.write_fmt(format_args!("{}\r\n", error));
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
            in("a0") dtb,
            in("a1") &memory_map as *const _,
            out("ra") _,
        );
    }

    loop {}
}

/// Load and prepare kernel from ELF image.
fn load_kernel(
    uefi: &mut uefi::Application,
    elf_data: &[u8],
) -> Result<*const core::ffi::c_void, Error> {
    if let Ok(kernel_elf) = unsafe { elf::ElfFile::from_buffer(elf_data) } {
        uefi::Console::write_string(uefi, "\r\nLoading kernel...\r\n");

        let virtual_entry = kernel_elf.header().get_entry_point();
        let (virtual_base, page_count) = get_elf_memory_info(uefi, &kernel_elf)?;
        let relocation_table: Option<elf::RelocationTable> = kernel_elf.relocation_table()?;

        #[cfg(feature = "debug_kernel")]
        {
            uefi.write_fmt(format_args!(
                "\r\nEntry point (virtual address): {:#018X}\r\n",
                virtual_entry
            ));

            uefi.write_fmt(format_args!(
                "Segment count: {}\r\n",
                 kernel_elf.header().get_program_header_info().entry_count,
            ));

            if relocation_table.is_some() {
                uefi::Console::write_string(uefi, "Relocation table present\r\n");
            }
        }

        let kernel_buffer = allocate_elf_memory(
            uefi,
            virtual_base,
            page_count,
            relocation_table.is_some(),
        )?;

        // TODO: This is how the ELF base address is defined. I don't see the point, so
        // probably I'm doing something wrong here...
        let base_address = calculate_base_address(
            uefi,
            virtual_base,
            kernel_buffer,
        );
        let physical_base = (virtual_base as i64 + base_address) as *const core::ffi::c_void;

        copy_elf_memory(&kernel_elf, virtual_base, kernel_buffer)?;

        // apply relocations
        if let Some(relocations) = relocation_table.as_ref() {
            #[cfg(feature = "debug_kernel")]
            uefi::Console::write_string(uefi, "\r\nApplying relocations:\r\n");

            for rela in relocations {
                #[cfg(feature = "debug_kernel")]
                uefi.write_fmt(format_args!(
                    "RELA [{:#x}] {:#018x}, {:#018x}\r\n",
                    rela.info,
                    rela.offset,
                    rela.addend,
                ));

                match rela.info {
                    elf::dynamic::R_RISCV_RELATIVE => {
                        unsafe {
                            let address = physical_base.add(rela.offset) as *mut u64;
                            let value = physical_base.offset(rela.addend as isize) as u64;

                            address.write(value);
                        }
                    },
                    _ => return Err(Error::InvalidKernelImage),
                }
            }
        };

        let entry_point = (virtual_entry as i64 + base_address) as *const core::ffi::c_void;

        #[cfg(feature = "debug_kernel")]
        uefi.write_fmt(format_args!(
            "Kernel entry point in memory: {:#018X}\r\n",
            entry_point as usize,
        ));

        Ok(entry_point)
    } else {
        Err(Error::InvalidKernelImage)
    }
}

fn get_elf_memory_info(
    _uefi: &mut uefi::Application,
    kernel_elf: &elf::ElfFile,
) -> Result<(usize, usize), Error> {
    let program_headers = kernel_elf.program_headers()
        .map_err(|_| Error::InvalidKernelImage)?;

    let mut memory_limits: Option<(usize, usize, usize)> = None;
    for program_header in program_headers {
        if program_header.get_type() != Some(elf::SegmentType::Load) {
            continue;
        }

        let base = program_header.get_page_base();
        let address = program_header.get_virtual_address();
        let size = program_header.get_memory_size();

        #[cfg(feature = "debug_kernel")]
        {
            uefi::Console::write_string(_uefi, "\r\nSegment:\r\n");
            _uefi.write_fmt(format_args!("offset: {:#018x}\r\n", program_header.get_offset()));
            _uefi.write_fmt(format_args!("vaddr: {:#018x}\r\n", address));
            _uefi.write_fmt(format_args!("memsz: {:#018x}\r\n", size));
        }

        if let Some((lowest_base, highest_address, highest_size)) = memory_limits {
            if base < lowest_base {
                memory_limits = Some((base, highest_address, highest_size));
            } else if address > highest_address {
                memory_limits = Some((lowest_base, address, size));
            }
        } else {
            memory_limits = Some((base, address, size));
        }
    }

    if let Some((lowest_base, highest_address, highest_size)) = memory_limits {
        let mut page_count = (highest_address + highest_size - lowest_base) / 4096;
        // round up
        if highest_size & 0xFFF > 0 {
            page_count += 1;
        }

        #[cfg(feature = "debug_kernel")]
        _uefi.write_fmt(format_args!("\r\nvirtual_base: {:#018x}\r\n", lowest_base));

        Ok((lowest_base, page_count))
    } else {
        Err(Error::InvalidKernelImage)
    }
}

/// Allocate memory for the ELF file.
fn allocate_elf_memory(
    uefi: &mut uefi::Application,
    virtual_base: usize,
    page_count: usize,
    dynamic: bool,
) -> Result<&'static mut [u8], Error> {
    #[cfg(feature = "debug_kernel")]
    {
        if dynamic {
            uefi.write_fmt(format_args!("\r\nAllocating {} page(s)\r\n", page_count));
        } else {
            uefi.write_fmt(format_args!(
                "\r\nAllocating {} page(s) at {:#018X}\r\n",
                page_count,
                virtual_base,
            ));
        }
    }

    let buffer = {
        if dynamic {
            uefi::Memory::allocate_pages(uefi, page_count)
        } else {
            uefi::Memory::allocate_pages_at(uefi, virtual_base as u64, page_count)
        }
    };

    match buffer {
        Some(buffer) => Ok(buffer),
        None => Err(Error::MemoryAllocationFailed),
    }
}

fn calculate_base_address(
    _uefi: &mut uefi::Application,
    virtual_base: usize,
    buffer: &[u8],
) -> i64 {
    let physical_base = &buffer[0] as *const u8 as u64;
    let base_address = physical_base as i64 - virtual_base as i64;

    #[cfg(feature = "debug_kernel")]
    _uefi.write_fmt(format_args!("\r\nELF base address: {:#018X}\r\n", base_address));

    base_address
}

/// Copy ELF loadable segments into memory.
fn copy_elf_memory(
    kernel_elf: &elf::ElfFile,
    virtual_base: usize,
    target_buffer: &mut [u8],
) -> Result<(), Error> {
    let program_headers = kernel_elf.program_headers()
        .map_err(|_| Error::InvalidKernelImage)?;

    for program_header in program_headers {
        if program_header.get_type() != Some(elf::SegmentType::Load) {
            continue;
        }

        let offset = program_header.get_virtual_address() - virtual_base;

        let file_buffer = kernel_elf.segment_data(program_header)
            .map_err(|err| core::convert::Into::<Error>::into(err))?;

        let target_buffer = &mut target_buffer[offset..(offset + file_buffer.len())];
        target_buffer.copy_from_slice(file_buffer);
    }

    Ok(())
}

#[cfg(feature = "debug_mmap")]
fn debug_mmap(uefi: &mut uefi::Application) -> Result<(), Error> {
    use uefi::api::boot_services::memory;

    let memory_map = uefi::Memory::get_memory_map(uefi)
        .map_err(|err| core::convert::Into::<Error>::into(err))?;

    uefi::Console::write_string(uefi, "\r\nMemory Map:\r\n");
    for descriptor in &memory_map {
        uefi.write_fmt(format_args!(
            "\r\n{:#018X} - {:#018X}: {}\r\n",
            descriptor.physical_start,
            descriptor.physical_start + descriptor.number_of_pages * 4096,
            match descriptor.r#type {
                memory::EFI_RESERVED_MEMORY_TYPE => "EfiReservedMemoryType",
                memory::EFI_LOADER_DATA => "EfiLoaderData",
                memory::EFI_BOOT_SERVICES_CODE => "EfiBootServicesCode",
                memory::EFI_BOOT_SERVICES_DATA => "EfiBootServicesData",
                memory::EFI_CONVENTIONAL_MEMORY => "EfiConventionalMemory",
                memory::EFI_UNUSABLE_MEMORY => "EfiUnusableMemory",
                memory::EFI_MEMORY_MAPPED_IO => "EfiMemoryMappedIO",
                _ => "",
            },
        ));
        uefi.write_fmt(format_args!("type: {:#010x}\r\n", descriptor.r#type));
        uefi.write_fmt(format_args!("virtual_start: {:#018x}\r\n", descriptor.virtual_start));
        uefi.write_fmt(format_args!("attribute: {:#018x}\r\n", descriptor.attribute));
    }

    Ok(())
}

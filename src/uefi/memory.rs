use super::UEFIError;
use super::api::boot_services::memory;

pub trait Memory: super::UEFI {
    fn allocate(
        &mut self,
        size: usize,
    ) -> Option<*mut core::ffi::c_void> {
        let boot_services = unsafe {
            &mut *(self.borrow_system().boot_services)
        };

        let mut ptr = core::ptr::null();
        let status = (boot_services.allocate_pool)(
            memory::EFI_LOADER_DATA,
            size,
            (&mut ptr) as *mut *const core::ffi::c_void
        );

        if status.is_ok() {
            Some(ptr as *mut core::ffi::c_void)
        } else {
            None
        }
    }

    fn get_memory_map(&mut self) -> Result<MemoryMap, UEFIError> {
        let mut memory_map_size = 0usize;
        let mut memory_map = core::ptr::null();
        let mut map_key = 0usize;
        let mut descriptor_size = 0usize;
        let mut descriptor_ver = 0u32;

        {
            // call get_memory_map once to get the memory map size
            let boot_services = unsafe {
                &mut *(self.borrow_system().boot_services)
            };
            (boot_services.get_memory_map)(
                (&mut memory_map_size) as *mut usize,
                memory_map as *mut memory::EfiMemoryDescriptor,
                (&mut map_key) as *mut usize,
                (&mut descriptor_size) as *mut usize,
                (&mut descriptor_ver) as *mut u32,
            );
            if memory_map_size == 0 {
                return Err(UEFIError::MemoryMapUnavailable);
            }
        }

        // account for the memory map pool allocation
        memory_map_size += descriptor_size;

        // allocate memory for the memory map
        match self.allocate(memory_map_size) {
            Some(ptr) => {
                memory_map = ptr;
            },
            None => {
                return Err(UEFIError::MemoryAllocationFailed);
            },
        }

        {
            // call get_memory_map a second time to get the actual memory map
            let boot_services = unsafe {
                &mut *(self.borrow_system().boot_services)
            };
            let status = (boot_services.get_memory_map)(
                (&mut memory_map_size) as *mut usize,
                memory_map as *mut memory::EfiMemoryDescriptor,
                (&mut map_key) as *mut usize,
                (&mut descriptor_size) as *mut usize,
                (&mut descriptor_ver) as *mut u32,
            );
            if !status.is_ok() {
                return Err(UEFIError::MemoryMapUnavailable);
            }
        }

        Ok(MemoryMap {
            map_key: map_key,
            descriptor_size: descriptor_size,
            descriptor_version: descriptor_ver,
            _map_handle: memory_map as *mut memory::EfiMemoryDescriptor,
        })
    }
}

impl Memory for super::Application {}

pub struct MemoryMap {
    pub map_key: usize,
    pub descriptor_size: usize,
    pub descriptor_version: u32,
    _map_handle: *mut memory::EfiMemoryDescriptor,
}

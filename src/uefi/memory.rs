use super::{
    Application, UEFIError,
    api::{
        EfiPhyiscalAddress,
        boot_services::memory,
    },
};

pub trait Memory {
    fn allocate(&mut self, size: usize) -> Option<*mut core::ffi::c_void>;
    /// Attempt to allocate `count` continuous 4KiB pages.
    fn allocate_pages_at(&mut self, addr: u64, count: usize) -> Option<&'static mut [u8]>;
    fn get_memory_map(&mut self) -> Result<MemoryMap, UEFIError>;

    fn allocate_pages(&mut self, count: usize) -> Option<&'static mut [u8]> {
        self.allocate_pages_at(0, count)
    }
}

impl Memory for Application {
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

    fn allocate_pages_at(
        &mut self,
        address: u64,
        count: usize,
    ) -> Option<&'static mut [u8]> {
        let boot_services = unsafe {
            &mut *(self.borrow_system().boot_services)
        };

        let mut ptr: EfiPhyiscalAddress = address;
        let allocate_type = match address {
            0 => memory::ALLOCATE_ANY_PAGES,
            _ => memory::ALLOCATE_ADDRESS,
        };
        let status = (boot_services.allocate_pages)(
            allocate_type,
            memory::EFI_LOADER_DATA,
            count,
            (&mut ptr) as *mut EfiPhyiscalAddress
        );

        if status.is_ok() {
            let slice = unsafe {
                core::slice::from_raw_parts_mut(
                    ptr as *mut u8,
                    4096 * count
                )
            };
            Some(slice)
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
                (boot_services.free_pool)(memory_map);
                return Err(UEFIError::MemoryMapUnavailable);
            }
        }

        Ok(MemoryMap {
            map_key: map_key,
            map_size: memory_map_size,
            descriptor_size: descriptor_size,
            _descriptor_version: descriptor_ver,
            map_handle: memory_map as *const memory::EfiMemoryDescriptor,
        })
    }
}

/// Memory safety: Dropping MemoryMap without freeing the allocated
/// `_map_handle` will leak memory
pub struct MemoryMap {
    pub map_key: usize,
    map_size: usize,
    descriptor_size: usize,
    _descriptor_version: u32,
    map_handle: *const memory::EfiMemoryDescriptor,
}

impl MemoryMap {
    pub fn iter<'a>(&'a self) -> MemoryMapIterator<'a> {
        MemoryMapIterator::new(&self)
    }
}

impl<'a> core::iter::IntoIterator for &'a MemoryMap {
    type Item = &'a memory::EfiMemoryDescriptor;
    type IntoIter = MemoryMapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct MemoryMapIterator<'a> {
    memory_map: &'a MemoryMap,
    next_index: usize,
}

impl<'a> MemoryMapIterator<'a> {
    fn new(memory_map: &'a MemoryMap) -> Self {
        MemoryMapIterator { memory_map, next_index: 0 }
    }
}

impl<'a> core::iter::Iterator for MemoryMapIterator<'a> {
    type Item = &'a memory::EfiMemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.memory_map.descriptor_size * self.next_index;
        if offset + self.memory_map.descriptor_size > self.memory_map.map_size {
            return None;
        }

        self.next_index += 1;

        unsafe {
            let descriptor_ptr = (self.memory_map.map_handle as *const u8).add(offset)
                as *const memory::EfiMemoryDescriptor;

            Some(& *(descriptor_ptr))
        }
    }
}

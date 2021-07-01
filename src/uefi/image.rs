use super::{
    Application,
    UEFIError,
    MemoryMap,
};

pub trait Image {
    fn exit_boot_services(self, memory_map: &MemoryMap)
        -> Result<(), UEFIError>
    where
        Self: Sized;
}

impl Image for Application {
    fn exit_boot_services(mut self, memory_map: &MemoryMap)
        -> Result<(), UEFIError>
    where
        Self: Sized
    {
        // FIXME: Wrap memory map in a custom type, and abstract out `map_key`
        // FIXME: An error for this call consumes the UEFI object
        // Errors from exit_boot_services may be recoverable e.g. when the
        // cause is an outdated memory map.
        // After a failed call to exit_boot_services, only calls to
        // MemoryAllocationServices (and exit_boot_services) are permitted.
        let image_handle = self.borrow_handle().clone();
        let boot_services = unsafe {
            &mut *(self.borrow_system().boot_services)
        };

        let status = (boot_services.exit_boot_services)(
            image_handle,
            memory_map.map_key
        );
        if status.is_ok() {
            Ok(())
        } else {
            Err(UEFIError::MemoryMapOutdated)
        }
    }
}

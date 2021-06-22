pub struct Application {
    image_handle: super::EfiHandle,
    system_table: *mut super::EfiSystemTable,
}

pub trait UEFI {
    fn borrow_handle(&self) -> &super::EfiHandle;
    fn borrow_system(&mut self) -> &mut super::EfiSystemTable;
}

impl Application {
    pub unsafe fn from(
        image_handle: super::EfiHandle,
        system_table: *mut super::EfiSystemTable,
    ) -> Self {
        Self { image_handle, system_table }
    }
}

impl UEFI for Application {
    fn borrow_handle(&self) -> &super::EfiHandle {
        &self.image_handle
    }

    fn borrow_system(&mut self) -> &mut super::EfiSystemTable {
        unsafe { &mut *self.system_table }
    }
}

pub struct Application {
    image_handle: super::EfiHandle,
    system_table: *mut super::EfiSystemTable,
}

impl Application {
    pub unsafe fn from(
        image_handle: super::EfiHandle,
        system_table: *mut super::EfiSystemTable,
    ) -> Option<Self> {
        if (*system_table).hdr.signature == super::api::system::SYSTEM_TABLE_SIGNATURE {
            Some(Self { image_handle, system_table })
        } else {
            None
        }
    }

    pub(super) fn borrow_handle(&self) -> &super::EfiHandle {
        &self.image_handle
    }

    pub(super) fn borrow_system(&mut self) -> &mut super::EfiSystemTable {
        unsafe { &mut *self.system_table }
    }
}

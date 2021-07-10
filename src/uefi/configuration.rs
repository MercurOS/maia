use super::{
    Application,
    api::configuration::EFI_DTB_TABLE_GUID,
};

pub trait Configuration {
    fn get_dtb(&mut self) -> Option<*const core::ffi::c_void>;
}

impl Configuration for Application {
    fn get_dtb(&mut self) -> Option<*const core::ffi::c_void> {
        let system_table = self.borrow_system();

        let configuration_table = system_table.configuration_table;
        for i in 0..system_table.number_of_table_entries {
            let entry = unsafe { & *configuration_table.add(i) };
            if entry.vendor_guid == EFI_DTB_TABLE_GUID {
                return Some(entry.vendor_table);
            }
        }

        None
    }
}

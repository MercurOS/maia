use core::ffi::c_void;

use super::super::{
    EfiStatus,
    EfiTableHeader,
    EfiPhyiscalAddress,
    EfiVirtualAddress,
};

#[repr(C)]
pub struct EfiBootServices {
    pub hdr: EfiTableHeader,

    // Task Priority Services
    _raise_tpl: *const c_void,
    _restore_tpl: *const c_void,

    // Memory Services
    _allocate_pages: *const c_void,
    _free_pages: *const c_void,
    pub get_memory_map: extern "efiapi" fn(
        memory_map_size: *mut usize,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,
    ) -> EfiStatus,
    _allocate_pool: *const c_void,
    _free_pool: *const c_void,

    // Event & Timer Services
    _create_event: *const c_void,
    _set_timer: *const c_void,
    _wait_for_event: *const c_void,
    _signal_event: *const c_void,
    _close_event: *const c_void,
    _check_event: *const c_void,

    // Protocol Handler Services
    _install_protocol_interface: *const c_void,
    _reinstall_protocol_interface: *const c_void,
    _uninstall_protocol_interface: *const c_void,
    _handle_protocol: *const c_void,
    _reserved: *const c_void,
    _register_protocol_notify: *const c_void,
    _locate_handle: *const c_void,
    _locate_device_path: *const c_void,
    _install_configuration_table: *const c_void,

    // Image Services
    _load_image: *const c_void,
    _start_image: *const c_void,
    _exit: *const c_void,
    _unload_image: *const c_void,
    _exit_boot_services: *const c_void,

    // Miscellaneous Services
    _get_next_monotonic_count: *const c_void,
    _stall: *const c_void,
    _set_watchdog_timer: *const c_void,

    // DriverSupport Services
    _connect_controller: *const c_void,
    _disconnect_controller: *const c_void,

    // Open and Close Protocol Services
    _open_protocol: *const c_void,
    _close_protocol: *const c_void,
    _open_protocol_information: *const c_void,

    // Library Services
    _protocols_per_handle: *const c_void,
    _locate_handle_buffer: *const c_void,
    _locate_protocol: *const c_void,
    _install_multiple_protocol_interfaces: *const c_void,
    _uninstall_multiple_protocol_interfaces: *const c_void,

    // 32-bit CRC Services
    _calculate_crc32: *const c_void,

    // Miscellaneous Services
    _copy_mem: *const c_void,
    _set_mem: *const c_void,
    _create_event_ex: *const c_void,
}

#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub r#type: u32,
    pub physical_start: EfiPhyiscalAddress,
    pub virtual_start: EfiVirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

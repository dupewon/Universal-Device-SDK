use uds_plugin::abi::exports::RegistryHandle;
use uds_plugin::abi::UDS_PLUGIN_ABI_VERSION;

#[no_mangle]
pub extern "C" fn uds_plugin_abi_version() -> u32 {
    UDS_PLUGIN_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn uds_plugin_name() -> *const std::os::raw::c_char {
    c"example-plugin\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn uds_plugin_version() -> *const std::os::raw::c_char {
    c"0.1.0\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn uds_plugin_register(_registry: RegistryHandle) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn uds_plugin_unregister() -> i32 {
    0
}

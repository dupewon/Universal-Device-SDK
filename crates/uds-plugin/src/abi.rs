pub const UDS_PLUGIN_ABI_VERSION: u32 = 1;

/// Plugin ABI entry points that every plugin must export.
/// All functions use C ABI for cross-language compatibility.
pub mod exports {
    use std::ffi::CStr;

    pub type RegistryHandle = *mut std::ffi::c_void;
    pub type PluginRegisterFn = extern "C" fn(RegistryHandle) -> i32;
    pub type PluginUnregisterFn = extern "C" fn() -> i32;

    /// Validate a plugin's ABI version against the host.
    pub fn validate_abi_version(plugin_version: u32) -> Result<(), String> {
        if plugin_version == UDS_PLUGIN_ABI_VERSION {
            Ok(())
        } else if plugin_version > UDS_PLUGIN_ABI_VERSION {
            Err(format!(
                "Plugin ABI v{} is newer than host ABI v{}",
                plugin_version, UDS_PLUGIN_ABI_VERSION
            ))
        } else {
            Err(format!(
                "Plugin ABI v{} is too old. Host requires v{}",
                plugin_version, UDS_PLUGIN_ABI_VERSION
            ))
        }
    }

    /// Safely convert a C string pointer to a Rust string.
    pub fn c_str_to_string(ptr: *const std::os::raw::c_char) -> Result<String, String> {
        if ptr.is_null() {
            return Err("null pointer".into());
        }
        unsafe {
            CStr::from_ptr(ptr)
                .to_str()
                .map(|s| s.to_string())
                .map_err(|e| format!("invalid UTF-8: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::exports;

    #[test]
    fn test_abi_version_match() {
        assert!(exports::validate_abi_version(1).is_ok());
    }

    #[test]
    fn test_abi_version_too_new() {
        assert!(exports::validate_abi_version(2).is_err());
    }
}

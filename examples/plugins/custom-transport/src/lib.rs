use uds_core::error::Error;
use uds_plugin::abi::exports::{RegistryHandle, validate_abi_version};
use uds_plugin::abi::UDS_PLUGIN_ABI_VERSION;
use uds_transport::traits::{Transport, TransportConnection, TransportConfig, TransportError};
use std::sync::Mutex;

static REGISTRY: Mutex<Option<RegistryHandle>> = Mutex::new(None);

pub struct CanTransport;

impl Transport for CanTransport {
    fn open(&self, _config: TransportConfig) -> Result<Box<dyn TransportConnection>, TransportError> {
        Err(TransportError::Unavailable("CAN transport not yet implemented".into()))
    }

    fn name(&self) -> &'static str {
        "can"
    }

    fn is_available(&self) -> bool {
        cfg!(target_os = "linux")
    }
}

#[no_mangle]
pub extern "C" fn uds_plugin_abi_version() -> u32 {
    UDS_PLUGIN_ABI_VERSION
}

#[no_mangle]
pub extern "C" fn uds_plugin_name() -> *const std::os::raw::c_char {
    c"can-transport\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn uds_plugin_version() -> *const std::os::raw::c_char {
    c"0.1.0\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn uds_plugin_init() -> i32 {
    tracing::info!("CAN transport plugin initializing");
    0
}

#[no_mangle]
pub extern "C" fn uds_plugin_register(registry: RegistryHandle) -> i32 {
    tracing::info!("CAN transport plugin registered");
    if let Ok(mut reg) = REGISTRY.lock() {
        *reg = Some(registry);
    }
    0
}

#[no_mangle]
pub extern "C" fn uds_plugin_unregister() -> i32 {
    tracing::info!("CAN transport plugin unregistering");
    if let Ok(mut reg) = REGISTRY.lock() {
        *reg = None;
    }
    0
}

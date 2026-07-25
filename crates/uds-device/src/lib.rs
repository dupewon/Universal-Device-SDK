pub mod traits;
pub mod discovery;
pub mod pairing;
pub mod manager;
pub mod esp32;
pub mod stm32;
pub mod rp2040;
pub mod arduino;

pub use traits::{Device, DeviceDiscovery, DeviceManager, DeviceError, DeviceEvent};
pub use discovery::DiscoveryService;
pub use pairing::PairingManager;
pub use manager::DeviceManagerImpl;

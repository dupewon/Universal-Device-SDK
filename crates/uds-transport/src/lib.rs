pub mod traits;
pub mod registry;
pub mod serial;
pub mod tcp;
pub mod udp;
pub mod websocket;
pub mod ble;
pub mod usb;
pub mod mock;

pub use traits::{Transport, TransportConnection, TransportError};
pub use registry::TransportRegistry;
pub use mock::MockTransport;

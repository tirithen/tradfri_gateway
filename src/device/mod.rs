mod light;
pub use light::*;

mod parse;
pub use parse::*;

mod update;
pub use update::*;

use crate::{TradfriGateway, TradfriGatewayError};

#[derive(Debug)]
pub enum Device {
    RemoteControl,
    Light(Box<Light>),
}

impl Device {
    pub fn new(gateway: TradfriGateway, bytes: &[u8]) -> Result<Device, DeviceError> {
        let device_type: DeviceTypeParsed = match serde_json::from_slice(bytes) {
            Ok(d) => d,
            Err(error) => {
                return Err(DeviceError::SerdeError(
                    error.to_string(),
                    String::from_utf8_lossy(bytes).to_string(),
                ))
            }
        };

        match device_type.device_type {
            0 => Ok(Device::RemoteControl),
            2 => {
                let light = Light::new(gateway, bytes)?;
                let device = Device::Light(Box::new(light));
                Ok(device)
            }
            _ => Err(DeviceError::UnsupportedDevice(device_type.device_type)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("Unsupported device type: {0}")]
    UnsupportedDevice(u32),

    #[error("Expected device type {0}")]
    ExpectedDeviceType(String),

    #[error("Serde error: {0}, raw data: {1}")]
    SerdeError(String, String),

    #[error("Tradfri gateway error: {0}")]
    TradfriGatewayError(#[from] TradfriGatewayError),
}

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use crate::{
    device::parse::DeviceTypeParsed, TradfriGateway, TradfriGatewayError, TradfriGatewayState,
    TradfriGatewayStateConnected,
};

use self::parse::{BulbParsed, BulbRgbXYParsed, DeviceInfoParsed, LightParsed};

mod parse;

mod color;
pub use color::*;

mod update;
pub use update::*;

#[derive(Debug)]
pub enum Device<S: TradfriGatewayState> {
    RemoteControl,
    Light(Light<S>),
}

impl<S: TradfriGatewayState> Device<S> {
    pub fn new(gateway: TradfriGateway<S>, bytes: &[u8]) -> Result<Device<S>, DeviceError> {
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
                let light = Light::<S>::new(gateway, bytes)?;
                let device = Device::Light(light);
                Ok(device)
            }
            _ => Err(DeviceError::UnsupportedDevice(device_type.device_type)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    manufacturer: String,
    product: String,
    firmware: String,
}

impl From<DeviceInfoParsed> for DeviceInfo {
    fn from(value: DeviceInfoParsed) -> Self {
        Self {
            manufacturer: value.manufacturer,
            product: value.product,
            firmware: value.firmware,
        }
    }
}

#[derive(Debug)]
pub struct Light<S: TradfriGatewayState> {
    gateway: TradfriGateway<S>,
    info: DeviceInfo,
    id: u32,
    name: String,
    creation_date: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    reachable: bool,
    bulbs: Vec<BulbParsed>,
}

impl<S: TradfriGatewayState> Light<S> {
    pub fn new(gateway: TradfriGateway<S>, bytes: &[u8]) -> Result<Self, DeviceError> {
        let parsed: LightParsed = match serde_json::from_slice(bytes) {
            Ok(p) => p,
            Err(error) => {
                return Err(DeviceError::SerdeError(
                    error.to_string(),
                    String::from_utf8_lossy(bytes).to_string(),
                ))
            }
        };

        Ok(Self {
            gateway,
            info: parsed.info.into(),
            id: parsed.id,
            name: parsed.name,
            creation_date: Utc.from_utc_datetime(
                &NaiveDateTime::from_timestamp_opt(parsed.creation_date.into(), 0).unwrap(),
            ),
            last_seen: Utc.from_utc_datetime(
                &NaiveDateTime::from_timestamp_opt(parsed.last_seen.into(), 0).unwrap(),
            ),
            reachable: parsed.reachable,
            bulbs: parsed.bulbs,
        })
    }
}

impl Light<TradfriGatewayStateConnected> {
    pub fn on(&mut self) -> Result<(), DeviceError> {
        let update = Update::BulbUpdate {
            bulbs: self
                .bulbs
                .iter()
                .map(|bulb| match bulb {
                    BulbParsed::DriverParsed(_) => BulbUpdate::DriverUpdate(DriverUpdate {
                        on: Some(true),
                        ..Default::default()
                    }),
                    BulbParsed::BulbColdWarmHexParsed(_) => {
                        BulbUpdate::BulbColdWarmHexUpdate(BulbColdWarmHexUpdate {
                            on: Some(true),
                            ..Default::default()
                        })
                    }
                    BulbParsed::BulbRgbXYParsed(_) => {
                        BulbUpdate::BulbRgbXYUpdate(BulbRgbXYUpdate {
                            on: Some(true),
                            ..Default::default()
                        })
                    }
                })
                .collect(),
        };
        self.gateway.update_device(self.id, update)?;
        self.update()?;

        Ok(())
    }

    pub fn off(&mut self) -> Result<(), DeviceError> {
        let update = Update::BulbUpdate {
            bulbs: self
                .bulbs
                .iter()
                .map(|bulb| match bulb {
                    BulbParsed::DriverParsed(_) => BulbUpdate::DriverUpdate(DriverUpdate {
                        on: Some(false),
                        ..Default::default()
                    }),
                    BulbParsed::BulbColdWarmHexParsed(_) => {
                        BulbUpdate::BulbColdWarmHexUpdate(BulbColdWarmHexUpdate {
                            on: Some(false),
                            ..Default::default()
                        })
                    }
                    BulbParsed::BulbRgbXYParsed(_) => {
                        BulbUpdate::BulbRgbXYUpdate(BulbRgbXYUpdate {
                            on: Some(false),
                            ..Default::default()
                        })
                    }
                })
                .collect(),
        };
        self.gateway.update_device(self.id, update)?;
        self.update()?;

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), DeviceError> {
        let device = self.gateway.device(self.id)?;

        if let Device::Light(light) = device {
            self.info = light.info;
            self.id = light.id;
            self.name = light.name;
            self.creation_date = light.creation_date;
            self.last_seen = light.last_seen;
            self.reachable = light.reachable;
            self.bulbs = light.bulbs;
        } else {
            return Err(DeviceError::ExpectedDeviceType("Light".to_string()));
        }

        Ok(())
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

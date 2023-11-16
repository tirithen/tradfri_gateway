use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use crate::{
    tradfri_coap::TradfriConnection, BulbColdWarmHexUpdate, BulbParsed, BulbRgbXYUpdate,
    BulbUpdate, Device, DeviceError, DeviceInfoParsed, DriverUpdate, LightDeviceParsed,
    TradfriGateway, Update,
};

#[derive(Debug)]
pub struct Light {
    gateway: TradfriGateway,
    info: DeviceInfoParsed,
    id: u32,
    name: String,
    creation_date: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    reachable: bool,
    bulbs: Vec<BulbParsed>,
}

impl Light {
    pub fn new(gateway: TradfriGateway, bytes: &[u8]) -> Result<Self, DeviceError> {
        let parsed: LightDeviceParsed = match serde_json::from_slice(bytes) {
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
            info: parsed.info,
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

    pub fn on(&mut self) -> Result<(), DeviceError> {
        let update = Update::BulbUpdate {
            bulbs: self
                .bulbs
                .iter()
                .map(|bulb| match bulb {
                    BulbParsed::LedDriver(_) => BulbUpdate::DriverUpdate(DriverUpdate {
                        on: Some(true),
                        ..Default::default()
                    }),
                    BulbParsed::BulbColdWarmHex(_) => {
                        BulbUpdate::BulbColdWarmHexUpdate(BulbColdWarmHexUpdate {
                            on: Some(true),
                            ..Default::default()
                        })
                    }
                    BulbParsed::BulbRgbXY(_) => BulbUpdate::BulbRgbXYUpdate(BulbRgbXYUpdate {
                        on: Some(true),
                        ..Default::default()
                    }),
                })
                .collect(),
        };

        let mut connection = self.gateway.create_connection()?;
        self.gateway
            .update_device(self.id, &update, Some(&mut connection))?;
        self.update_with_connection(&mut connection)?;

        Ok(())
    }

    pub fn off(&mut self) -> Result<(), DeviceError> {
        let update = Update::BulbUpdate {
            bulbs: self
                .bulbs
                .iter()
                .map(|bulb| match bulb {
                    BulbParsed::LedDriver(_) => BulbUpdate::DriverUpdate(DriverUpdate {
                        on: Some(false),
                        ..Default::default()
                    }),
                    BulbParsed::BulbColdWarmHex(_) => {
                        BulbUpdate::BulbColdWarmHexUpdate(BulbColdWarmHexUpdate {
                            on: Some(false),
                            ..Default::default()
                        })
                    }
                    BulbParsed::BulbRgbXY(_) => BulbUpdate::BulbRgbXYUpdate(BulbRgbXYUpdate {
                        on: Some(false),
                        ..Default::default()
                    }),
                })
                .collect(),
        };

        let mut connection = self.gateway.create_connection()?;
        self.gateway
            .update_device(self.id, &update, Some(&mut connection))?;
        self.update_with_connection(&mut connection)?;

        Ok(())
    }

    pub fn is_on(&self) -> bool {
        self.bulbs
            .iter()
            .fold(0, |acc, b| acc + if b.is_on() { 1 } else { 0 })
            > 0
    }

    pub fn update(&mut self) -> Result<(), DeviceError> {
        let mut connection = self.gateway.create_connection()?;
        self.update_with_connection(&mut connection)
    }

    fn update_with_connection(
        &mut self,
        connection: &mut TradfriConnection,
    ) -> Result<(), DeviceError> {
        let device = self.gateway.device_with_connection(self.id, connection)?;

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

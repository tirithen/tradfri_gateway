use crate::{tradfri_coap::TradfriConnection, TradfriGateway, TradfriGatewayError};

mod parse;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
pub use parse::*;

mod update;
pub use update::*;

#[derive(Debug, Clone)]
pub struct Group {
    gateway: TradfriGateway,
    on: bool,
    brightness: u8,
    name: String,
    creation_date: DateTime<Utc>,
    id: u32,
    items: Items,
}

#[derive(Debug, Clone)]
pub struct Items {
    // #[serde(rename = "15001")]
    // pub items: Vec<Device>,
}

impl Group {
    pub fn new(gateway: TradfriGateway, bytes: &[u8]) -> Result<Group, GroupError> {
        let parsed: GroupParsed = match serde_json::from_slice(bytes) {
            Ok(d) => d,
            Err(error) => {
                return Err(GroupError::SerdeError(
                    error.to_string(),
                    String::from_utf8_lossy(bytes).to_string(),
                ))
            }
        };

        Ok(Group {
            gateway,
            on: parsed.on,
            brightness: parsed.brightness,
            name: parsed.name,
            creation_date: Utc.from_utc_datetime(
                &NaiveDateTime::from_timestamp_opt(parsed.creation_date.into(), 0).unwrap(),
            ),
            id: parsed.id,
            items: Items {
                // items: parsed.items,
            },
        })
    }

    pub fn on(&mut self) -> Result<(), GroupError> {
        let update = GroupUpdate {
            on: Some(true),
            ..Default::default()
        };

        let mut connection = self.gateway.create_connection()?;
        self.gateway
            .update_group(self.id, &update, Some(&mut connection))?;
        self.update_with_connection(&mut connection)?;

        Ok(())
    }

    pub fn off(&mut self) -> Result<(), GroupError> {
        let update = GroupUpdate {
            on: Some(false),
            ..Default::default()
        };

        let mut connection = self.gateway.create_connection()?;
        self.gateway
            .update_group(self.id, &update, Some(&mut connection))?;
        self.update_with_connection(&mut connection)?;

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), GroupError> {
        let mut connection = self.gateway.create_connection()?;
        self.update_with_connection(&mut connection)
    }

    fn update_with_connection(
        &mut self,
        connection: &mut TradfriConnection,
    ) -> Result<(), GroupError> {
        let group = self.gateway.group_with_connection(self.id, connection)?;

        self.on = group.on;
        self.brightness = group.brightness;
        self.name = group.name;
        self.creation_date = group.creation_date;
        self.id = group.id;
        self.items = group.items;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GroupError {
    #[error("Serde error: {0}, raw data: {1}")]
    SerdeError(String, String),

    #[error("Tradfri gateway error: {0}")]
    TradfriGatewayError(#[from] TradfriGatewayError),
}

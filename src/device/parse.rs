use serde::{de, Deserialize, Deserializer, Serialize};

use crate::{ColdWarmColor, RgbColor};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceTypeParsed {
    #[serde(rename = "5750")]
    pub device_type: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceInfoParsed {
    #[serde(rename = "0")]
    pub manufacturer: String,
    #[serde(rename = "1")]
    pub product: String,
    #[serde(rename = "3")]
    pub firmware: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceInfoWithBatteryParsed {
    #[serde(rename = "0")]
    pub manufacturer: String,
    #[serde(rename = "1")]
    pub product: String,
    #[serde(rename = "2")]
    pub firmware: String,
    #[serde(rename = "9")]
    pub battery: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LightParsed {
    #[serde(rename = "3")]
    pub info: DeviceInfoParsed,
    #[serde(rename = "3311")]
    pub bulbs: Vec<BulbParsed>,
    #[serde(rename = "9001")]
    pub name: String,
    #[serde(rename = "9002")]
    pub creation_date: u32,
    #[serde(rename = "9003")]
    pub id: u32,
    #[serde(rename = "9019", deserialize_with = "bool_from_int")]
    pub reachable: bool,
    #[serde(rename = "9020")]
    pub last_seen: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BulbParsed {
    DriverParsed(DriverParsed),
    BulbColdWarmHexParsed(BulbColdWarmHexParsed),
    // BulbRgbHexParsed(BulbRgbHexParsed),
    BulbRgbXYParsed(BulbRgbXYParsed),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriverParsed {
    #[serde(rename = "5850", deserialize_with = "bool_from_int")]
    pub on: bool,
    #[serde(rename = "5851")]
    pub brightness: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulbColdWarmHexParsed {
    #[serde(rename = "5706")]
    pub color_hex: ColdWarmColor,
    #[serde(rename = "5707")]
    pub hue: Option<u32>,
    #[serde(rename = "5708")]
    pub saturation: Option<u32>,
    #[serde(rename = "5711")]
    pub color_temperature: Option<u32>,
    #[serde(rename = "5850", deserialize_with = "bool_from_int")]
    pub on: bool,
    #[serde(rename = "5851")]
    pub brightness: u8,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct BulbRgbHexParsed {
//     #[serde(rename = "5706")]
//     pub color_hex: RgbColorParsed,
//     #[serde(rename = "5707")]
//     pub hue: Option<u32>,
//     #[serde(rename = "5708")]
//     pub saturation: Option<u32>,
//     #[serde(rename = "5711")]
//     pub color_temperature: Option<u32>,
//     #[serde(rename = "5712")]
//     pub transition_time: u32,
//     #[serde(rename = "5850", deserialize_with = "bool_from_int")]
//     pub on: bool,
//     #[serde(rename = "5851")]
//     pub brightness: u8,
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulbRgbXYParsed {
    #[serde(rename = "5706")]
    pub color_hex: RgbColor,
    #[serde(rename = "5707")]
    pub hue: Option<u32>,
    #[serde(rename = "5708")]
    pub saturation: Option<u32>,
    #[serde(rename = "5709")]
    pub color_x: u32,
    #[serde(rename = "5710")]
    pub color_y: u32,
    #[serde(rename = "5711")]
    pub color_temperature: Option<u32>,
    // #[serde(rename = "5712")]
    // pub transition_time: u32,
    #[serde(rename = "5850", deserialize_with = "bool_from_int")]
    pub on: bool,
    #[serde(rename = "5851")]
    pub brightness: u8,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

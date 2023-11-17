use serde::{Deserialize, Serialize};

use crate::{ColdWarmColor, RgbColor, serialization::{option_bool_from_int, option_int_from_bool}};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DeviceUpdate {
    BulbUpdate {
        #[serde(rename = "3311")]
        bulbs: Vec<BulbUpdate>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BulbUpdate {
    DriverUpdate(DriverUpdate),
    BulbColdWarmHexUpdate(BulbColdWarmHexUpdate),
    // BulbRgbHexParsedUpdate(BulbRgbHexParsedUpdate),
    BulbRgbXYUpdate(BulbRgbXYUpdate),
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DriverUpdate {
    #[serde(rename = "5712", skip_serializing_if = "Option::is_none")]
    pub transition_time: Option<u32>,
    #[serde(
        rename = "5850",
        deserialize_with = "option_bool_from_int",
        serialize_with = "option_int_from_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub on: Option<bool>,
    #[serde(rename = "5851", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BulbColdWarmHexUpdate {
    #[serde(rename = "5706", skip_serializing_if = "Option::is_none")]
    pub color_hex: Option<ColdWarmColor>,
    #[serde(rename = "5707", skip_serializing_if = "Option::is_none")]
    pub hue: Option<u32>,
    #[serde(rename = "5708", skip_serializing_if = "Option::is_none")]
    pub saturation: Option<u32>,
    #[serde(rename = "5711", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<u32>,
    #[serde(rename = "5712", skip_serializing_if = "Option::is_none")]
    pub transition_time: Option<u32>,
    #[serde(
        rename = "5850",
        deserialize_with = "option_bool_from_int",
        serialize_with = "option_int_from_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub on: Option<bool>,
    #[serde(rename = "5851", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
}

// #[derive(Default, Debug, Clone, Deserialize, Serialize)]
// pub struct BulbRgbHexParsedUpdate {
//     #[serde(rename = "5706", skip_serializing_if = "Option::is_none")]
//     pub color_hex: Option<RgbColorUpdate>,
//     #[serde(rename = "5707", skip_serializing_if = "Option::is_none")]
//     pub hue: Option<u32>,
//     #[serde(rename = "5708", skip_serializing_if = "Option::is_none")]
//     pub saturation: Option<u32>,
//     #[serde(rename = "5711", skip_serializing_if = "Option::is_none")]
//     pub color_temperature: Option<u32>,
//     #[serde(rename = "5712", skip_serializing_if = "Option::is_none")]
//     pub transition_time: Option<u32>,
//     #[serde(rename = "5850", deserialize_with = "option_bool_from_int", skip_serializing_if = "Option::is_none")]
//     pub on: Option<bool>,
//     #[serde(rename = "5851", skip_serializing_if = "Option::is_none")]
//     pub brightness: Option<u8>,
// }

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BulbRgbXYUpdate {
    #[serde(rename = "5706", skip_serializing_if = "Option::is_none")]
    pub color_hex: Option<RgbColor>,
    #[serde(rename = "5707", skip_serializing_if = "Option::is_none")]
    pub hue: Option<u32>,
    #[serde(rename = "5708", skip_serializing_if = "Option::is_none")]
    pub saturation: Option<u32>,
    #[serde(rename = "5709", skip_serializing_if = "Option::is_none")]
    pub color_x: Option<u32>,
    #[serde(rename = "5710", skip_serializing_if = "Option::is_none")]
    pub color_y: Option<u32>,
    #[serde(rename = "5711", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<u32>,
    #[serde(rename = "5712", skip_serializing_if = "Option::is_none")]
    pub transition_time: Option<u32>,
    #[serde(
        rename = "5850",
        deserialize_with = "option_bool_from_int",
        serialize_with = "option_int_from_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub on: Option<bool>,
    #[serde(rename = "5851", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
}

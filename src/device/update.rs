use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::{ColdWarmColor, RgbColor};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Update {
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
    // #[serde(rename = "5712", skip_serializing_if = "Option::is_none")]
    // pub transition_time: u32,
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

fn option_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(Some(false)),
        1 => Ok(Some(true)),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

fn option_int_from_bool<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(true) => serializer.serialize_u8(1),
        Some(false) => serializer.serialize_u8(0),
        None => serializer.serialize_none(),
    }
}

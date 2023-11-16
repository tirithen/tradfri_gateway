use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{ColdWarmColor, RgbColor};

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
    // BulbRgbHexUpdate(BulbRgbHexUpdate),
    BulbRgbXYUpdate(BulbRgbXYUpdate),
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DriverUpdate {
    #[serde(
        rename = "5850",
        deserialize_with = "option_bool_from_int",
        serialize_with = "option_int_from_bool"
    )]
    pub on: Option<bool>,
    #[serde(rename = "5851")]
    pub brightness: Option<u8>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BulbColdWarmHexUpdate {
    #[serde(rename = "5706")]
    pub color_hex: Option<ColdWarmColor>,
    #[serde(rename = "5707")]
    pub hue: Option<u32>,
    #[serde(rename = "5708")]
    pub saturation: Option<u32>,
    #[serde(rename = "5711")]
    pub color_temperature: Option<u32>,
    #[serde(
        rename = "5850",
        deserialize_with = "option_bool_from_int",
        serialize_with = "option_int_from_bool"
    )]
    pub on: Option<bool>,
    #[serde(rename = "5851")]
    pub brightness: Option<u8>,
}

// #[derive(Default, Debug, Clone, Deserialize, Serialize)]
// pub struct BulbRgbHexUpdate {
//     #[serde(rename = "5706")]
//     pub color_hex: Option<RgbColorUpdate>,
//     #[serde(rename = "5707")]
//     pub hue: Option<u32>,
//     #[serde(rename = "5708")]
//     pub saturation: Option<u32>,
//     #[serde(rename = "5711")]
//     pub color_temperature: Option<u32>,
//     #[serde(rename = "5850", deserialize_with = "option_bool_from_int")]
//     pub on: Option<bool>,
//     #[serde(rename = "5851")]
//     pub brightness: Option<u8>,
// }

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct BulbRgbXYUpdate {
    #[serde(rename = "5706")]
    pub color_hex: Option<RgbColor>,
    #[serde(rename = "5707")]
    pub hue: Option<u32>,
    #[serde(rename = "5708")]
    pub saturation: Option<u32>,
    #[serde(rename = "5709")]
    pub color_x: Option<u32>,
    #[serde(rename = "5710")]
    pub color_y: Option<u32>,
    #[serde(rename = "5711")]
    pub color_temperature: Option<u32>,
    // #[serde(rename = "5712")]
    // pub transition_time: u32,
    #[serde(
        rename = "5850",
        deserialize_with = "option_bool_from_int",
        serialize_with = "option_int_from_bool"
    )]
    pub on: Option<bool>,
    #[serde(rename = "5851")]
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

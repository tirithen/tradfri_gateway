use serde::{Deserialize, Serialize};

use crate::serialization::bool_from_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupParsed {
    // #[serde(rename = "5712")]
    // pub transition_time: u32,
    #[serde(rename = "5850", deserialize_with = "bool_from_int")]
    pub on: bool,
    #[serde(rename = "5851")]
    pub brightness: u8,
    #[serde(rename = "9001")]
    pub name: String,
    #[serde(rename = "9002")]
    pub creation_date: u32,
    #[serde(rename = "9003")]
    pub id: u32,
    #[serde(rename = "9018")]
    pub items: ItemsParsed,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsParsed {
    // #[serde(rename = "15001")]
    // pub items: Vec<Device>,
}
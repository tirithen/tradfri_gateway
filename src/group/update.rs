use serde::{Deserialize, Serialize};

use crate::serialization::option_int_from_bool;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct GroupUpdate {
    #[serde(rename = "5712", skip_serializing_if = "Option::is_none")]
    pub transition_time: Option<u32>,
    #[serde(rename = "5850", serialize_with = "option_int_from_bool", skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(rename = "5851", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    #[serde(rename = "9001", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "9002", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<u32>,
    #[serde(rename = "9003", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(rename = "9039", skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<u32>,
}

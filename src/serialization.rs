use serde::{Deserialize, Deserializer, Serializer, de};

pub(crate) fn option_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
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

pub(crate) fn option_int_from_bool<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(true) => serializer.serialize_u8(1),
        Some(false) => serializer.serialize_u8(0),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
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

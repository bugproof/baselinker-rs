use serde::de;
use serde::de::Deserializer;
use std::fmt;

pub fn inconsistent_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(InconsistentBoolVisitor)
}

struct InconsistentBoolVisitor;

impl<'de> de::Visitor<'de> for InconsistentBoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(value)
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(match value {
            0 => false,
            1 => true,
            _ => return Err(E::invalid_value(de::Unexpected::Unsigned(value), &self)),
        })
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(match value {
            "0" => false,
            "1" => true,
            _ if value.eq_ignore_ascii_case("false") => false,
            _ if value.eq_ignore_ascii_case("true") => true,
            _ => return Err(E::custom("string cannot be converted to bool")),
        })
    }
}

pub fn inconsistent_bool_option<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(OptionInconsistentBoolVisitor)
}

struct OptionInconsistentBoolVisitor;

impl<'de> de::Visitor<'de> for OptionInconsistentBoolVisitor {
    type Value = Option<bool>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean or none")
    }

    fn visit_none<E>(self) -> Result<Option<bool>, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer
            .deserialize_any(InconsistentBoolVisitor)
            .map(Some)
    }

    fn visit_unit<E>(self) -> Result<Option<bool>, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

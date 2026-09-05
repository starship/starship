use serde::{
    Deserialize, Serialize,
    de::{Deserializer, Visitor},
};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub enum NotificationMode {
    None,
    Desktop,
    Osc9,
}

struct NotificationModeVisitor;

impl<'de> Visitor<'de> for NotificationModeVisitor {
    type Value = NotificationMode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an boolean or string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v {
            Ok(NotificationMode::Desktop)
        } else {
            Ok(NotificationMode::None)
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "none" => Ok(NotificationMode::None),
            "desktop" => Ok(NotificationMode::Desktop),
            "osc9" => Ok(NotificationMode::Osc9),
            unknown => Err(serde::de::Error::unknown_variant(
                unknown,
                &["none", "desktop", "osc9"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NotificationModeVisitor)
    }
}

impl Serialize for NotificationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NotificationMode::None => serializer.serialize_bool(false),
            NotificationMode::Desktop => serializer.serialize_bool(true),
            NotificationMode::Osc9 => serializer.serialize_str("osc9"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct CmdDurationConfig<'a> {
    pub min_time: i64,
    pub format: &'a str,
    pub style: &'a str,
    pub show_milliseconds: bool,
    pub disabled: bool,
    pub show_notifications: NotificationMode,
    pub min_time_to_notify: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_timeout: Option<u32>,
}

impl Default for CmdDurationConfig<'_> {
    fn default() -> Self {
        Self {
            min_time: 2_000,
            format: "took [$duration]($style) ",
            show_milliseconds: false,
            style: "yellow bold",
            disabled: false,
            show_notifications: NotificationMode::None,
            min_time_to_notify: 45_000,
            notification_timeout: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_notification_mode_from_bool() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::from_value::<NotificationMode>(serde_json::json!(false))?,
            NotificationMode::None
        );
        assert_eq!(
            serde_json::from_value::<NotificationMode>(serde_json::json!(true))?,
            NotificationMode::Desktop
        );
        Ok(())
    }

    #[test]
    fn deserialize_notification_mode_from_string() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::from_value::<NotificationMode>(serde_json::json!("none"))?,
            NotificationMode::None
        );
        assert_eq!(
            serde_json::from_value::<NotificationMode>(serde_json::json!("desktop"))?,
            NotificationMode::Desktop
        );
        assert_eq!(
            serde_json::from_value::<NotificationMode>(serde_json::json!("osc9"))?,
            NotificationMode::Osc9
        );
        Ok(())
    }
}

use crate::module::ALL_MODULES;
use serde::de::{
    value::{Error as ValueError, MapDeserializer, SeqDeserializer},
    Deserializer, Error, IntoDeserializer, Visitor,
};
use std::{cmp::Ordering, fmt};
use toml::Value;

/// A helper struct for deserializing a TOML value references with serde.
/// This also prints a warning and suggestions if a key is unknown.
#[derive(Debug)]
pub struct ValueDeserializer<'de> {
    value: &'de Value,
    info: Option<StructInfo>,
    current_key: Option<&'de str>,
    error_on_ignored: bool,
}

/// When deserializing a struct, this struct stores information about the struct.
#[derive(Debug, Clone, Copy)]
struct StructInfo {
    fields: &'static [&'static str],
    name: &'static str,
}

impl<'de> ValueDeserializer<'de> {
    pub fn new(value: &'de Value) -> Self {
        ValueDeserializer {
            value,
            info: None,
            current_key: None,
            error_on_ignored: true,
        }
    }

    fn with_info(
        value: &'de Value,
        info: Option<StructInfo>,
        current_key: &'de str,
        ignored: bool,
    ) -> Self {
        ValueDeserializer {
            value,
            info,
            current_key: Some(current_key),
            error_on_ignored: ignored,
        }
    }

    pub fn with_allow_unknown_keys(self) -> Self {
        ValueDeserializer {
            error_on_ignored: false,
            ..self
        }
    }
}

impl ValueDeserializer<'_> {
    /// Prettify an error message by adding the current key and struct name to it.
    fn error<T: fmt::Display>(&self, msg: T) -> ValueError {
        match (self.current_key, self.info) {
            (Some(key), Some(StructInfo { name, .. })) => {
                // Prettify name of struct
                let display_name = name.strip_suffix("Config").unwrap_or(name);
                ValueError::custom(format!("Error in '{display_name}' at '{key}': {msg}",))
            }
            // Handling other cases leads to duplicates in the error message.
            _ => ValueError::custom(msg),
        }
    }
}

impl<'de> IntoDeserializer<'de> for ValueDeserializer<'de> {
    type Deserializer = ValueDeserializer<'de>;

    fn into_deserializer(self) -> ValueDeserializer<'de> {
        self
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = ValueError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Boolean(b) => visitor.visit_bool(*b),
            Value::Integer(i) => visitor.visit_i64(*i),
            Value::Float(f) => visitor.visit_f64(*f),
            Value::String(s) => visitor.visit_borrowed_str(s),
            Value::Array(a) => {
                let seq = SeqDeserializer::new(a.iter().map(ValueDeserializer::new));
                seq.deserialize_seq(visitor)
            }
            Value::Table(t) => {
                let map = MapDeserializer::new(t.iter().map(|(k, v)| {
                    (
                        k.as_str(),
                        ValueDeserializer::with_info(
                            v,
                            self.info,
                            k.as_str(),
                            self.error_on_ignored,
                        ),
                    )
                }));
                map.deserialize_map(visitor)
            }
            Value::Datetime(d) => visitor.visit_string(d.to_string()),
        }
        .map_err(|e| self.error(e))
    }

    // Save a reference to the struct fields and name for later use in error messages.
    fn deserialize_struct<V>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.info = Some(StructInfo { fields, name });
        self.deserialize_any(visitor)
    }

    // Always `Some` because TOML doesn't have a null type.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    // Handle ignored Values. (Values at unknown keys in TOML)
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self
            .info
            .filter(|StructInfo { name, .. }| name == &"StarshipRootConfig")
            .and(self.current_key)
            .map_or(false, |key| {
                ALL_MODULES.contains(&key) || key == "custom" || key == "env_var"
            })
        {
            return visitor.visit_none();
        }

        if !self.error_on_ignored {
            return visitor.visit_none();
        }

        let did_you_mean = match (self.current_key, self.info) {
            (Some(key), Some(StructInfo { fields, .. })) => fields
                .iter()
                .filter_map(|field| {
                    let score = strsim::jaro_winkler(key, field);
                    (score > 0.8).then_some((score, field))
                })
                .max_by(|(score_a, _field_a), (score_b, _field_b)| {
                    score_a.partial_cmp(score_b).unwrap_or(Ordering::Equal)
                }),
            _ => None,
        };
        let did_you_mean = did_you_mean
            .map(|(_score, field)| format!(" (Did you mean '{field}'?)"))
            .unwrap_or_default();

        Err(self.error(format!("Unknown key{did_you_mean}")))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Handle most deserialization cases by deferring to `deserialize_any`.
    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq
        bytes byte_buf map unit_struct tuple_struct enum tuple identifier
    }
}

#[cfg(test)]
mod test {
    use crate::configs::StarshipRootConfig;

    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_deserialize_bool() {
        let value = Value::Boolean(true);
        let deserializer = ValueDeserializer::new(&value);
        let result: bool = bool::deserialize(deserializer).unwrap();
        assert!(result);
    }

    #[test]
    fn test_deserialize_i64() {
        let value = Value::Integer(42);
        let deserializer = ValueDeserializer::new(&value);
        let result: i64 = i64::deserialize(deserializer).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_deserialize_f64() {
        let value = Value::Float(3.14);
        let deserializer = ValueDeserializer::new(&value);
        let result: f64 = f64::deserialize(deserializer).unwrap();
        assert_eq!(result, 3.14);
    }

    #[test]
    fn test_deserialize_string() {
        let value = Value::String("hello".to_string());
        let deserializer = ValueDeserializer::new(&value);
        let result: String = String::deserialize(deserializer).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_deserialize_str() {
        let value = toml::toml! {
            foo = "bar"
        };
        let deserializer = ValueDeserializer::new(&value);

        #[derive(Deserialize)]
        struct StrWrapper<'a> {
            foo: &'a str,
        }

        let result = StrWrapper::deserialize(deserializer).unwrap();
        assert_eq!(result.foo, "bar");
    }

    #[test]
    fn test_deserialize_datetime() {
        let value = toml::toml! {
            foo = 2018-01-01T00:00:00Z
        };
        let deserializer = ValueDeserializer::new(&value);

        #[derive(Deserialize)]
        struct DateWrapper {
            foo: String,
        }

        let result = DateWrapper::deserialize(deserializer).unwrap();
        assert_eq!(result.foo, "2018-01-01T00:00:00Z");
    }

    #[test]
    fn test_deserialize_array() {
        let value = toml::toml! {
            foo = [1, 2, 3]
        };
        let deserializer = ValueDeserializer::new(&value);

        #[derive(Deserialize)]
        struct ArrayWrapper {
            foo: Vec<i32>,
        }

        let result = ArrayWrapper::deserialize(deserializer).unwrap();
        assert_eq!(result.foo, vec![1, 2, 3]);
    }

    #[test]
    fn test_deserialize_map() {
        let value = toml::toml! {
            [foo]
            a = 1
            b = 2
        };
        let deserializer = ValueDeserializer::new(&value);

        #[derive(Deserialize)]
        struct MapWrapper {
            foo: std::collections::HashMap<String, i32>,
        }

        let result = MapWrapper::deserialize(deserializer).unwrap();
        assert_eq!(
            result.foo,
            std::collections::HashMap::from_iter(vec![("a".to_string(), 1), ("b".to_string(), 2)])
        );
    }

    #[test]
    fn test_deserialize_newtype_struct() {
        let value = toml::toml! {
            foo = "bar"
        };

        #[derive(Deserialize)]
        struct NewtypeWrapper(String);
        #[derive(Deserialize)]
        struct Sample {
            foo: NewtypeWrapper,
        }

        let deserializer = ValueDeserializer::new(&value);

        let result = Sample::deserialize(deserializer).unwrap();
        assert_eq!(result.foo.0, "bar".to_owned());
    }

    #[test]
    fn test_deserialize_unknown() {
        let value = toml::toml! {
            foo = "bar"
            unknown_key = 1
        };
        let deserializer = ValueDeserializer::new(&value);

        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Sample {
            foo: String,
        }

        let result = Sample::deserialize(deserializer).unwrap_err();
        assert_eq!(
            format!("{result}"),
            "Error in 'Sample' at 'unknown_key': Unknown key"
        );
    }

    #[test]
    fn test_deserialize_unknown_root_config() {
        let value = toml::toml! {
            unknown_key = "foo"
        };

        let deserializer = ValueDeserializer::new(&value);
        let result = StarshipRootConfig::deserialize(deserializer).unwrap_err();
        assert_eq!(
            format!("{result}"),
            "Error in 'StarshipRoot' at 'unknown_key': Unknown key"
        );

        let deserializer2 = ValueDeserializer::new(&value).with_allow_unknown_keys();
        let result = StarshipRootConfig::deserialize(deserializer2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialize_unknown_root_module() {
        let value = toml::toml! {
            [rust]
            disabled = true
        };
        let deserializer = ValueDeserializer::new(&value);

        let result = StarshipRootConfig::deserialize(deserializer);
        assert!(result.is_ok())
    }

    #[test]
    fn test_deserialize_unknown_typo() {
        let value = toml::toml! {
            food = "bar"
        };
        let deserializer = ValueDeserializer::new(&value);

        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Sample {
            foo: String,
        }

        let result = Sample::deserialize(deserializer).unwrap_err();
        assert_eq!(
            format!("{result}"),
            "Error in 'Sample' at 'food': Unknown key (Did you mean 'foo'?)"
        );
    }

    #[test]
    fn test_deserialize_wrong_type() {
        let value = toml::toml! {
            foo = 1
        };
        let deserializer = ValueDeserializer::new(&value);

        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Sample {
            foo: String,
        }

        let result = Sample::deserialize(deserializer).unwrap_err();
        assert_eq!(
            format!("{result}"),
            "Error in 'Sample' at 'foo': invalid type: integer `1`, expected a string"
        );
    }
}

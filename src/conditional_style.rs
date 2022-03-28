use crate::context::Context;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq)]
pub enum StarshipConditionalStyleOperator {
    Equal,
    Exists,
}

impl Serialize for StarshipConditionalStyleOperator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            StarshipConditionalStyleOperator::Equal => "equal",
            StarshipConditionalStyleOperator::Exists => "exists",
        })
    }
}

impl<'de> Deserialize<'de> for StarshipConditionalStyleOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.to_lowercase().as_str() {
            "equal" => StarshipConditionalStyleOperator::Equal,
            "exists" => StarshipConditionalStyleOperator::Exists,
            _ => StarshipConditionalStyleOperator::Equal,
        })
    }
}

impl StarshipConditionalStyleOperator {
    fn invoke(&self, left_hand_side: Option<String>, right_hand_side: Option<String>) -> bool {
        match self {
            StarshipConditionalStyleOperator::Equal => left_hand_side == right_hand_side,
            StarshipConditionalStyleOperator::Exists => left_hand_side.is_some(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct StarshipConditionalStyle<'a> {
    pub env: Option<&'a str>,
    pub operator: Option<StarshipConditionalStyleOperator>,
    pub expected_value: Option<&'a str>,
    pub style: &'a str,
}

impl<'a> From<&'a str> for StarshipConditionalStyle<'a> {
    fn from(value: &'a str) -> Self {
        StarshipConditionalStyle {
            env: None,
            operator: None,
            expected_value: None,
            style: value,
        }
    }
}

impl<'a> StarshipConditionalStyle<'a> {
    fn should_apply(&self, context: &Context) -> bool {
        let env_value = self.env.and_then(|x| context.get_env(x));

        match &self.operator {
            Some(operator) => operator.invoke(env_value, self.expected_value.map(String::from)),
            None => true,
        }
    }
}

pub fn get_conditional_style<'a>(
    context: &Context,
    items: &[StarshipConditionalStyle<'a>],
) -> &'a str {
    let matching_style = items.iter().find(|s| s.should_apply(context));
    let last = items.iter().last();

    matching_style.or(last).map(|x| x.style).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{Context, Shell, Target};
    use crate::serde_utils::ValueDeserializer;
    use std::path::PathBuf;

    fn create_context<'a>() -> Context<'a> {
        Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            PathBuf::new(),
            PathBuf::new(),
        )
    }

    #[test]
    fn test_exists_operator() {
        let mut context = create_context();
        let style = StarshipConditionalStyle {
            env: Some("test"),
            operator: Some(StarshipConditionalStyleOperator::Exists),
            expected_value: None,
            style: "",
        };
        assert_eq!(style.should_apply(&context), false);
        context.env.insert("test", String::default());
        assert_eq!(style.should_apply(&context), true);
    }

    #[test]
    fn test_equal_operator() {
        let mut context = create_context();
        let style = StarshipConditionalStyle {
            env: Some("test"),
            operator: Some(StarshipConditionalStyleOperator::Equal),
            expected_value: Some("expected"),
            style: "",
        };

        assert_eq!(style.should_apply(&context), false);
        context.env.insert("test", String::from("not_expected"));
        assert_eq!(style.should_apply(&context), false);
        context.env.insert("test", String::from("expected"));
        assert_eq!(style.should_apply(&context), true);
    }

    #[test]
    fn should_deserialize_from_table_value() {
        let config = toml::toml! {
            env = "HOSTNAME"
            operator = "equal"
            expected_value = "home"
            style = "bold dimmed red"
        };
        let deserializer = ValueDeserializer::new(&config);

        let result = StarshipConditionalStyle::deserialize(deserializer);

        assert_eq!(
            result,
            Ok(StarshipConditionalStyle {
                env: Some("HOSTNAME"),
                operator: Some(StarshipConditionalStyleOperator::Equal),
                expected_value: Some("home"),
                style: "bold dimmed red"
            })
        );
    }

    #[test]
    fn get_conditional_style_fallback() {
        let context = create_context();
        let items: Vec<StarshipConditionalStyle> = vec![];
        assert_eq!(get_conditional_style(&context, &items), "");
    }

    #[test]
    fn get_conditional_style_no_match() {
        let context = create_context();
        let items: Vec<StarshipConditionalStyle> = vec![
            StarshipConditionalStyle {
                env: Some("env"),
                operator: Some(StarshipConditionalStyleOperator::Equal),
                expected_value: Some("value"),
                style: "red",
            },
            StarshipConditionalStyle::from("red bold"),
        ];
        assert_eq!(get_conditional_style(&context, &items), "red bold");
    }

    #[test]
    fn get_conditional_style_match_operator() {
        let items: Vec<StarshipConditionalStyle> = vec![
            StarshipConditionalStyle {
                env: Some("env"),
                operator: Some(StarshipConditionalStyleOperator::Exists),
                expected_value: None,
                style: "red",
            },
            StarshipConditionalStyle::from("style"),
        ];
        let mut context = create_context();
        context.env.insert("env", "value".into());
        assert_eq!(get_conditional_style(&context, &items), "red");
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct LineBreakConfig {
    pub disabled: bool,
}

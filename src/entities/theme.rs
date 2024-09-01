use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

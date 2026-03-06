use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemPriority {
    Low,
    Medium,
    High,
}

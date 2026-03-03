use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemPriority {
    Low,
    Medium,
    High,
}

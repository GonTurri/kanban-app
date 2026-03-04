use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ColumnType {
    Todo { limit: Option<usize> },
    Wip { limit: Option<usize> },
    Done,
}

use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ColumnType {
    Todo { limit: Option<usize> },
    Wip { limit: Option<usize> },
    Done,
}

use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BoardRole {
    Owner,
    Viewer,
    Editor,
}
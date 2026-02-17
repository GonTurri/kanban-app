#[derive(Debug, PartialEq, Clone)]
pub enum BoardRole {
    Owner,
    Viewer,
    Editor,
}
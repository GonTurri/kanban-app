#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BoardRole {
    Owner,
    Viewer,
    Editor,
}
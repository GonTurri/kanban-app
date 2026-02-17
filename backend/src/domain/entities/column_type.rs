#[derive(Debug, PartialEq, Clone)]
pub enum ColumnType {
    Todo {limit: Option<usize>},
    Wip {limit: Option<usize>},
    Done
}
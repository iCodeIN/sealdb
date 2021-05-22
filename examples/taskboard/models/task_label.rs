use sealdb::Record;

use crate::models::{TaskId, LabelId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskLabelId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct TaskLabel {
    pub id: TaskLabelId,
    pub task: TaskId,
    pub label: LabelId,
}

impl Record for TaskLabel {}

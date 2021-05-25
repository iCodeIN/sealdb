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

#[derive(Debug, Clone, PartialEq)]
pub struct InsertTaskLabel {
    pub task: TaskId,
    pub label: LabelId,
}

impl Record for TaskLabel {
    type PrimaryKey = TaskLabelId;
    type Fields = (); //TODO
    type Insert = InsertTaskLabel;

    fn create_primary_key(key: usize) -> Self::PrimaryKey {
        TaskLabelId(key)
    }

    fn from_insert(record: Self::Insert, primary_key: Self::PrimaryKey) -> Self {
        let InsertTaskLabel {task, label} = record;

        Self {id: primary_key, task, label}
    }
}

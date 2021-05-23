use chrono::{Date, Utc};
use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub title: FixedLenStr<256>,
    pub description: FixedLenStr<4096>,
    pub created: Date<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertTask {
    pub title: FixedLenStr<256>,
    pub description: FixedLenStr<4096>,
}

impl Record for Task {
    type PrimaryKey = TaskId;
    type Insert = InsertTask;

    fn create_primary_key(key: usize) -> Self::PrimaryKey {
        TaskId(key)
    }

    fn from_insert(record: Self::Insert, primary_key: Self::PrimaryKey) -> Self {
        let InsertTask {title, description} = record;
        let created = Utc::now().date();

        Self {id: primary_key, title, description, created}
    }
}

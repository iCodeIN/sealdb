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

impl Record for Task {}

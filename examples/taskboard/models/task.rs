use chrono::{Date, Utc};
use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub title: FixedLenStr<256>,
    pub description: FixedLenStr<4096>,
    pub completed: bool,
    pub created: Date<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertTask {
    pub title: FixedLenStr<256>,
    pub description: FixedLenStr<4096>,
}

impl Record for Task {
    type PrimaryKey = TaskId;
    type Fields<const ARG_INDEX: usize> = TaskFields<ARG_INDEX>;
    type Insert = InsertTask;

    fn create_primary_key(key: usize) -> Self::PrimaryKey {
        TaskId(key)
    }

    fn from_insert(record: Self::Insert, primary_key: Self::PrimaryKey) -> Self {
        let InsertTask {title, description} = record;

        let completed = false;
        let created = Utc::now().date();

        Self {id: primary_key, title, description, completed, created}
    }
}

//TODO: Complete these impls (autogenerate them)
impl sealdb::FieldAccess<3> for Task {
    type FieldType = bool;

    fn get(&self) -> &Self::FieldType {
        &self.completed
    }

    fn get_mut(&mut self) -> &mut Self::FieldType {
        &mut self.completed
    }
}

#[derive(Debug, Default)]
pub struct TaskFields<const ARG_INDEX: usize> {
    //TODO: other fields
    pub completed: sealdb::Field<Task, 3, ARG_INDEX>,
}

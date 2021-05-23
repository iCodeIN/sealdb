use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub id: ListId,
    pub title: FixedLenStr<256>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertList {
    pub title: FixedLenStr<256>,
}

impl Record for List {
    type PrimaryKey = ListId;
    type Insert = InsertList;

    fn create_primary_key(key: usize) -> Self::PrimaryKey {
        ListId(key)
    }

    fn from_insert(record: Self::Insert, primary_key: Self::PrimaryKey) -> Self {
        let InsertList {title} = record;

        Self {id: primary_key, title}
    }
}

use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub id: BoardId,
    pub title: FixedLenStr<256>,
    pub description: FixedLenStr<2048>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertBoard {
    pub title: FixedLenStr<256>,
    pub description: FixedLenStr<2048>,
}

impl Record for Board {
    type PrimaryKey = BoardId;
    type Insert = InsertBoard;

    fn create_primary_key(key: usize) -> Self::PrimaryKey {
        BoardId(key)
    }

    fn from_insert(record: Self::Insert, primary_key: Self::PrimaryKey) -> Self {
        let InsertBoard {title, description} = record;

        Self {id: primary_key, title, description}
    }
}

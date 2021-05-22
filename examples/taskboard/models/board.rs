use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub id: BoardId,
    pub title: FixedLenStr<256>,
    pub description: FixedLenStr<2048>,
}

impl Record for Board {}

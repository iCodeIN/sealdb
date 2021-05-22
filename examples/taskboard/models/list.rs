use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub id: ListId,
    pub title: FixedLenStr<256>,
}

impl Record for List {}

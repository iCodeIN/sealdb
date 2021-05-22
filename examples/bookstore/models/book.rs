use chrono::{Date, Utc};
use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub id: BookId,
    pub title: FixedLenStr<256>,
    pub genre: FixedLenStr<64>,
    // See: https://stackoverflow.com/a/66837719/551904
    pub isbn: FixedLenStr<17>,
    pub published: Date<Utc>,
}

impl Record for Book {}

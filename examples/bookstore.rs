use sealdb::{Table, Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookId(usize);

#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: FixedLenStr<256>,
    pub author: AuthorId,
    // See: https://stackoverflow.com/a/66837719/551904
    pub isbn: FixedLenStr<17>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthorId(usize);

fn main() {}

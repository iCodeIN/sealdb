use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthorId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Author {
    pub id: AuthorId,
    pub name: FixedLenStr<256>,
    pub bio: FixedLenStr<2048>,
}

impl Record for Author {}

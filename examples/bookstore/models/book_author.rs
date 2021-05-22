use sealdb::Record;

use crate::models::{BookId, AuthorId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookAuthorId(usize);

/// Books may have many authors and authors may publish many books
#[derive(Debug, Clone, PartialEq)]
pub struct BookAuthor {
    pub id: BookAuthorId,
    pub book: BookId,
    pub author: AuthorId,
}

impl Record for BookAuthor {}

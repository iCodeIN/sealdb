use sealdb::Record;

use crate::models::BookId;

/// A number of cents in USD
/// e.g. USD$12.75 is represented as 1275
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Usd(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListingId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Listing {
    pub id: ListingId,
    pub book: BookId,
    pub price: Usd,
}

impl Record for Listing {}

use chrono::{Date, Utc};
use sealdb::{Table, Record, types::FixedLenStr};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthorId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Author {
    pub id: AuthorId,
    pub name: FixedLenStr<256>,
    pub bio: FixedLenStr<2048>,
}

impl Record for Author {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListingId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Listing {
    pub id: ListingId,
    pub book: BookId,
    pub price: Usd,
}

impl Record for Listing {}

/// A number of cents in USD
/// e.g. USD$12.75 is represented as 1275
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Usd(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListingReviewId(usize);

/// A review for a book associated with a specific listing
#[derive(Debug, Clone, PartialEq)]
pub struct ListingReview {
    pub id: ListingReviewId,
    /// The listing that was reviewed
    pub listing: ListingId,
    /// The name of the person who left the review
    pub name: FixedLenStr<256>,
    /// The email of the person who left the review
    pub email: FixedLenStr<256>,
    /// The star rating of the review (1-5)
    pub rating: Rating,
    /// The text comment left with the review
    pub text: FixedLenStr<2048>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rating {
    OneStar,
    TwoStars,
    ThreeStars,
    FourStars,
    FiveStars,
}

fn main() {}

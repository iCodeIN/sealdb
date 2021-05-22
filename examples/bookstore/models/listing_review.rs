use sealdb::{Record, types::FixedLenStr};

use crate::models::ListingId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rating {
    OneStar,
    TwoStars,
    ThreeStars,
    FourStars,
    FiveStars,
}

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

impl Record for ListingReview {}

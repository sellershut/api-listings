mod error;
pub use std::fmt::Debug;

use crate::{Listing, ListingCondition};

pub use error::*;
pub use uuid::Uuid;

#[trait_variant::make(QueryListings: Send)]
pub trait LocalQueryListings {
    async fn get_listings(&self) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn get_listing_by_id(&self, listing_id: &Uuid) -> Result<Option<Listing>, CoreError>;
    async fn get_listings_from_user(
        &self,
        user_id: &Uuid,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn get_listings_in_category(
        &self,
        category_id: &Uuid,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn get_listings_with_tags(
        &self,
        tags: &[&Uuid],
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn get_listings_in_price_range(
        &self,
        min: f64,
        max: f64,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn search(
        &self,
        query: impl AsRef<str> + Send + Debug,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
}

#[trait_variant::make(QueryListingCondition: Send)]
pub trait LocalQueryListingCondition {
    async fn get_conditions(
        &self,
    ) -> Result<impl ExactSizeIterator<Item = ListingCondition>, CoreError>;
}

#[trait_variant::make(MutateListings: Send)]
pub trait LocalMutateListings {
    async fn create_listing(
        &self,
        listing: &Listing,
        user_id: &Uuid,
        category_id: &Uuid,
        condition_id: &Uuid,
        quantity: usize,
    ) -> Result<Listing, CoreError>;
    async fn update_listing(
        &self,
        id: &Uuid,
        data: &Listing,
        user_id: &Uuid,
        category_id: &Uuid,
        condition_id: &Uuid,
        quantity: usize,
    ) -> Result<Option<Listing>, CoreError>;
    async fn delete_listing(&self, id: &Uuid, user_id: &Uuid)
        -> Result<Option<Listing>, CoreError>;
    async fn upload_images(&self, files: &[&[u8]]) -> Result<Vec<(String, String)>, CoreError>;
}

mod error;
pub use std::fmt::Debug;

use crate::{Listing, Tag};

pub use error::*;
pub use uuid::Uuid;

#[trait_variant::make(QueryListings: Send)]
pub trait LocalQueryListings {
    async fn get_listings(&self) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn get_listing_by_id(
        &self,
        listing_id: impl Into<Uuid> + Send,
    ) -> Result<Option<Listing>, CoreError>;
    async fn get_listings_from_user(
        &self,
        user_id: impl Into<Uuid> + Send,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn get_listings_from_category(
        &self,
        category_id: impl Into<Uuid> + Send,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn get_listings_in_price_range(
        &self,
        min: f32,
        max: f32,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
    async fn search(
        &self,
        query: impl AsRef<str> + Send + Debug,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError>;
}

#[trait_variant::make(MutateListings: Send)]
pub trait LocalMutateListings {
    async fn create_listing(&self, category: &Listing) -> Result<Listing, CoreError>;
    async fn update_listing(&self, id: &Uuid, data: &Listing)
        -> Result<Option<Listing>, CoreError>;
    async fn delete_listing(&self, id: &Uuid) -> Result<Option<Listing>, CoreError>;
}

#[trait_variant::make(QueryTags: Send)]
pub trait LocalQueryTags {
    async fn get_tags(&self) -> Result<impl ExactSizeIterator<Item = Tag>, CoreError>;
    async fn get_tag_by_id(&self, id: impl Into<Uuid>) -> Result<Tag, CoreError>;
}

#[trait_variant::make(MutateTags: Send)]
pub trait LocalMutateTag {
    async fn create_tag(&self, category: &Tag) -> Result<Tag, CoreError>;
    async fn update_tag(&self, id: &Uuid, data: &Tag) -> Result<Option<Tag>, CoreError>;
    async fn delete_tag(&self, id: &Uuid) -> Result<Option<Tag>, CoreError>;
}

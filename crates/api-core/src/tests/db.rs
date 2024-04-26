use std::fmt::Debug;

use uuid::Uuid;

use crate::{
    api::{CoreError, LocalMutateListings, LocalQueryListings, MutateListings, QueryListings},
    Listing,
};

pub struct SampleDb;
pub struct SampleDbSend;

impl LocalQueryListings for SampleDb {
    async fn get_listings(&self) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listing_by_id(&self, _listing_id: &Uuid) -> Result<Option<Listing>, CoreError> {
        Ok(None)
    }

    async fn get_listings_from_user(
        &self,
        _user_id: &Uuid,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listings_in_category(
        &self,
        _category_id: &Uuid,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listings_in_price_range(
        &self,
        _min: f32,
        _max: f32,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn search(
        &self,
        _query: impl AsRef<str> + Send + Debug,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listings_with_tags(
        &self,
        _tags: &[&Uuid],
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }
}

impl LocalMutateListings for SampleDb {
    async fn create_listing(
        &self,
        listing: &Listing,
        _user_id: &Uuid,
    ) -> Result<Listing, CoreError> {
        Ok(listing.to_owned())
    }

    async fn update_listing(
        &self,
        _id: &Uuid,
        data: &Listing,
        _user_id: &Uuid,
    ) -> Result<Option<Listing>, CoreError> {
        Ok(Some(data.to_owned()))
    }

    async fn delete_listing(
        &self,
        _id: &Uuid,
        _user_id: &Uuid,
    ) -> Result<Option<Listing>, CoreError> {
        Ok(None)
    }

    async fn upload_images(&self, _files: &[&[u8]]) -> Result<Vec<(String, String)>, CoreError> {
        Ok(Default::default())
    }
}

impl MutateListings for SampleDbSend {
    async fn create_listing(
        &self,
        listing: &Listing,
        _user_id: &Uuid,
    ) -> Result<Listing, CoreError> {
        Ok(listing.to_owned())
    }

    async fn update_listing(
        &self,
        _id: &Uuid,
        data: &Listing,
        _user_id: &Uuid,
    ) -> Result<Option<Listing>, CoreError> {
        Ok(Some(data.to_owned()))
    }

    async fn delete_listing(
        &self,
        _id: &Uuid,
        _user_id: &Uuid,
    ) -> Result<Option<Listing>, CoreError> {
        Ok(None)
    }

    async fn upload_images(&self, _files: &[&[u8]]) -> Result<Vec<(String, String)>, CoreError> {
        Ok(Default::default())
    }
}

impl QueryListings for SampleDbSend {
    async fn get_listings_with_tags(
        &self,
        _tags: &[&Uuid],
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listings(&self) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listing_by_id(&self, _listing_id: &Uuid) -> Result<Option<Listing>, CoreError> {
        Ok(None)
    }

    async fn get_listings_from_user(
        &self,
        _user_id: &Uuid,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listings_in_category(
        &self,
        _category_id: &Uuid,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listings_in_price_range(
        &self,
        _min: f32,
        _max: f32,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn search(
        &self,
        _query: impl AsRef<str> + Send + Debug,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }
}

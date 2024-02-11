use std::{fmt::Debug, str::FromStr};

use uuid::Uuid;

use crate::{
    api::{CoreError, LocalMutateListings, LocalQueryListings, MutateListings, QueryListings},
    Listing,
};

pub struct SampleDb;
pub struct SampleDbSend;

impl LocalQueryListings for SampleDb {
    async fn search(
        &self,
        _query: impl AsRef<str> + Debug + Send,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_listings(&self) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        todo!()
    }

    async fn get_listing_by_id(&self, listing_id: impl Into<Uuid>) -> Result<Listing, CoreError> {
        todo!()
    }

    async fn get_listings_from_user(
        &self,
        user_id: impl Into<Uuid>,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        todo!()
    }

    async fn get_listings_from_category(
        &self,
        category_id: impl Into<Uuid>,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        todo!()
    }

    async fn get_listings_in_price_range(
        &self,
        min: f32,
        max: f32,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        todo!()
    }
}

impl LocalMutateCategories for SampleDb {
    async fn create_category(&self, category: &Listing) -> Result<Listing, CoreError> {
        Ok(category.to_owned())
    }

    async fn update_category(
        &self,
        id: &Uuid,
        data: &Listing,
    ) -> Result<Option<Listing>, CoreError> {
        if id.as_ref().is_empty() {
            Err(CoreError::from_str("Id cannot be empty")?)
        } else {
            Ok(Some(data.to_owned()))
        }
    }

    async fn delete_category(&self, _id: &Uuid) -> Result<Option<Listing>, CoreError> {
        Ok(None)
    }
}

impl MutateCategories for SampleDbSend {
    async fn create_category(&self, category: &Listing) -> Result<Listing, CoreError> {
        Ok(category.to_owned())
    }

    async fn update_category(
        &self,
        _id: &Uuid,
        data: &Listing,
    ) -> Result<Option<Listing>, CoreError> {
        Ok(Some(data.to_owned()))
    }

    async fn delete_category(&self, _id: &Uuid) -> Result<Option<Listing>, CoreError> {
        Ok(None)
    }
}

impl QueryCategories for SampleDbSend {
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_sub_categories(
        &self,
        _id: Option<&Uuid>,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_category_by_id(&self, _id: &Uuid) -> Result<Option<Listing>, CoreError> {
        Ok(None)
    }

    async fn search(
        &self,
        _query: impl AsRef<str> + Debug + Send,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok([].into_iter())
    }
}

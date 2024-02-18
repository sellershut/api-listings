use crate::{collections::Collections, entity::listing::DatabaseListing};
use api_core::{
    api::{CoreError, MutateListings},
    reexports::uuid::Uuid,
    Listing,
};
use surrealdb::opt::RecordId;
use time::OffsetDateTime;
use tracing::instrument;

use crate::{map_db_error, Client};

impl MutateListings for Client {
    #[instrument(skip(self), err(Debug))]
    async fn create_listing(&self, listing: &Listing) -> Result<Listing, CoreError> {
        // check if user exists,
        // cheeck if category exists
        // check if provided tag exists
        let input = InputListing::from(listing);
        let id = Uuid::now_v7();
        let item: Option<DatabaseListing> = self
            .client
            .create((Collections::Listing.to_string(), id.to_string()))
            .content(input)
            .await
            .map_err(map_db_error)?;

        match item {
            Some(e) => Listing::try_from(e),
            None => Err(CoreError::Unreachable),
        }
    }

    async fn update_listing(
        &self,
        id: &Uuid,
        data: &Listing,
    ) -> Result<Option<Listing>, CoreError> {
        todo!()
    }

    async fn delete_listing(&self, id: &Uuid) -> Result<Option<Listing>, CoreError> {
        todo!()
    }
}

#[derive(serde::Serialize)]
struct InputListing<'a> {
    user_id: RecordId,
    title: &'a str,
    description: &'a str,
    price: f32,
    category_id: RecordId,
    image_url: &'a str,
    other_images: &'a [String],
    active: bool,
    tags: Vec<RecordId>,
    location: &'a str,
    created_at: &'a OffsetDateTime,
    updated_at: Option<&'a OffsetDateTime>,
    deleted_at: Option<&'a OffsetDateTime>,
}

impl<'a> From<&'a Listing> for InputListing<'a> {
    fn from(value: &'a Listing) -> Self {
        let record =
            |collection: &str, uuid: &Uuid| RecordId::from((collection, uuid.to_string().as_str()));
        Self {
            title: &value.title,
            tags: value
                .tags
                .iter()
                .map(|str| RecordId::from((Collections::Tag.to_string(), str.to_string())))
                .collect(),
            image_url: &value.image_url,
            description: &value.description,
            user_id: record("user", &value.user_id),
            price: value.price,
            category_id: record("category", &value.category_id),
            other_images: &value.other_images,
            active: value.active,
            location: &value.location,
            created_at: &value.created_at,
            deleted_at: value.deleted_at.as_ref(),
            updated_at: value.updated_at.as_ref(),
        }
    }
}

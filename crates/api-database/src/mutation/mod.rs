use api_core::{
    api::{CoreError, MutateListings},
    reexports::uuid::Uuid,
    Listing,
};
use surrealdb::opt::RecordId;
use time::OffsetDateTime;

use crate::Client;

impl MutateListings for Client {
    async fn create_listing(&self, category: &Listing) -> Result<Listing, CoreError> {
        todo!()
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
    other_images: Option<&'a [String]>,
    active: bool,
    tags: Option<Vec<RecordId>>,
    location: &'a str,
    created_at: &'a OffsetDateTime,
    deleted_at: Option<&'a OffsetDateTime>,
}

impl<'a> From<&'a Listing> for InputListing<'a> {
    fn from(value: &'a Listing) -> Self {
        let record =
            |collection: &str, uuid: &Uuid| RecordId::from((collection, uuid.to_string().as_str()));
        Self {
            title: &value.title,
            tags: value.tags.as_ref().map(|f| {
                f.iter()
                    .map(|str| RecordId::from(("tag", str.to_string().as_str())))
                    .collect()
            }),
            image_url: &value.image_url,
            description: &value.description,
            user_id: record("user", &value.user_id),
            price: value.price,
            category_id: record("category", &value.category_id),
            other_images: value.other_images.as_deref(),
            active: value.active,
            location: &value.location,
            created_at: &value.created_at,
            deleted_at: value.deleted_at.as_ref(),
        }
    }
}

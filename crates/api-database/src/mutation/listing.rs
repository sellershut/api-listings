use crate::{
    collections::Collection,
    entity::{create_thing_from_id, listing::DatabaseEntityListing, tag::DatabaseEntityTag},
    graphql_requests::{find_category_by_id, find_user_by_id},
};
use api_core::{
    api::{CoreError, MutateListings},
    reexports::uuid::Uuid,
    Listing,
};
use surrealdb::opt::RecordId;
use time::OffsetDateTime;
use tracing::{instrument, Instrument};

use crate::{map_db_error, Client};

impl MutateListings for Client {
    #[instrument(skip(self), err(Debug))]
    async fn create_listing(&self, listing: &Listing) -> Result<Listing, CoreError> {
        let category_result = find_category_by_id(
            &self.http_client,
            &self.categories_api,
            crate::graphql_requests::category_by_id::Variables {
                id: listing.category_id,
            },
        );

        let user_result = find_user_by_id(
            &self.http_client,
            &self.users_api,
            crate::graphql_requests::user_by_id::Variables {
                id: listing.user_id,
            },
        );

        let (category_ok, user_ok) =
            futures_util::future::try_join(category_result, user_result).await?;

        let mut futs = Vec::with_capacity(listing.tags.len());
        for i in listing.tags.iter() {
            futs.push(async {
                self.client
                    .select::<Option<DatabaseEntityTag>>(create_thing_from_id(Collection::Tag, i))
                    .await
                    .map_err(map_db_error)
            });
        }

        let tags_exist = futures_util::future::try_join_all(futs)
            .instrument(tracing::info_span!("verifying tags"))
            .await?;

        if tags_exist.iter().any(|f| f.is_none()) {
            return Err(CoreError::Database(String::from(
                "One or more of your tags does not exist",
            )));
        }

        if !category_ok {
            return Err(CoreError::Database(format!(
                "category: {} does not exist",
                listing.category_id
            )));
        }

        if !user_ok {
            return Err(CoreError::Database(format!(
                "user: {} does not exist",
                listing.user_id
            )));
        }

        // check if user exists,
        // cheeck if category exists
        // check if provided tag exists
        let input = InputListing::from(listing);
        let id = Uuid::now_v7();
        let item: Option<DatabaseEntityListing> = self
            .client
            .create((Collection::Listing.to_string(), id.to_string()))
            .content(input)
            .await
            .map_err(map_db_error)?;

        match item {
            Some(e) => Listing::try_from(e),
            None => Err(CoreError::Unreachable),
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn update_listing(
        &self,
        id: &Uuid,
        data: &Listing,
    ) -> Result<Option<Listing>, CoreError> {
        todo!()
    }

    #[instrument(skip(self), err(Debug))]
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
    likes: Vec<RecordId>,
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
                .map(|str| RecordId::from((Collection::Tag.to_string(), str.to_string())))
                .collect(),
            image_url: &value.image_url,
            description: &value.description,
            user_id: record("user", &value.user_id),
            price: value.price,
            category_id: record("category", &value.category_id),
            other_images: &value.other_images,
            active: value.active,
            likes: vec![],
            location: &value.location,
            created_at: &value.created_at,
            deleted_at: value.deleted_at.as_ref(),
            updated_at: value.updated_at.as_ref(),
        }
    }
}

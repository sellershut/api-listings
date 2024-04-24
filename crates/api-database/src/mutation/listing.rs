use crate::{
    collections::Collection,
    entity::{create_thing_from_id, listing::DatabaseEntityListing, tag::DatabaseEntityTag},
    graphql_requests::{find_category_by_id, find_user_by_id},
    redis::{cache_keys::CacheKey, PoolLike, PooledConnectionLike, RedisPool},
};
use api_core::{
    api::{CoreError, MutateListings},
    reexports::uuid::Uuid,
    Listing,
};
use futures_util::TryFutureExt;
use surrealdb::opt::RecordId;
use time::OffsetDateTime;
use tracing::{debug, error, instrument, trace, Instrument};

use crate::{map_db_error, Client};

async fn check_listing_validity(
    client: &Client,
    category_id: &Uuid,
    user_id: &Uuid,
    tags: &[Uuid],
) -> Result<(), CoreError> {
    // check if category exists
    let category_result = find_category_by_id(
        &client.http_client,
        &client.categories_api,
        crate::graphql_requests::category_by_id::Variables { id: *category_id },
    );

    // check if user exists,
    let user_result = find_user_by_id(
        &client.http_client,
        &client.users_api,
        crate::graphql_requests::user_by_id::Variables { id: *user_id },
    );

    let (category_ok, user_ok) =
        futures_util::future::try_join(category_result, user_result).await?;

    let mut futs = Vec::with_capacity(tags.len());
    for i in tags.iter() {
        futs.push(async {
            client
                .client
                .select::<Option<DatabaseEntityTag>>(create_thing_from_id(Collection::Tag, i))
                .await
                .map_err(map_db_error)
        });
    }

    let tags_exist = futures_util::future::try_join_all(futs)
        .instrument(tracing::info_span!("verifying tags"))
        .await?;

    if !category_ok {
        return Err(CoreError::Database(format!(
            "category: {} does not exist",
            category_id
        )));
    }

    if !user_ok {
        return Err(CoreError::Database(format!(
            "user: {} does not exist",
            user_id
        )));
    }

    // check if provided tags exist
    if !tags_exist.is_empty() && tags_exist.iter().any(|f| f.is_none()) {
        return Err(CoreError::Database(String::from(
            "One or more of your tags does not exist",
        )));
    }

    Ok(())
}

async fn clear_listing_cache(redis: &RedisPool, user_id: &Uuid) {
    match redis.get().await {
        Ok(mut pool) => {
            let mut pipe = redis::Pipeline::new();
            pipe.del(CacheKey::AllListings)
                .del(CacheKey::UserListing { user_id });

            if let Err(e) = pool.query_async_pipeline::<()>(pipe).await {
                error!("{e}");
            }
        }
        Err(e) => {
            error!("{e}");
        }
    }
}

impl MutateListings for Client {
    #[instrument(skip(self), err(Debug))]
    async fn create_listing(
        &self,
        listing: &Listing,
        user_id: &Uuid,
    ) -> Result<Listing, CoreError> {
        check_listing_validity(self, &listing.category_id, user_id, &listing.tags).await?;

        let input = InputListing::from(listing);
        let id = Uuid::now_v7();
        let item: Option<DatabaseEntityListing> = self
            .client
            .create((Collection::Listing.to_string(), id.to_string()))
            .content(input)
            .await
            .map_err(map_db_error)?;

        match item {
            Some(e) => {
                let listing = Listing::try_from(e)?;
                if let Some((ref redis, _ttl)) = self.redis {
                    clear_listing_cache(redis, user_id).await;
                };
                debug!("listing created");
                Ok(listing)
            }
            None => Err(CoreError::Unreachable),
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn update_listing(
        &self,
        id: &Uuid,
        data: &Listing,
        user_id: &Uuid,
    ) -> Result<Option<Listing>, CoreError> {
        check_listing_validity(self, &data.category_id, user_id, &data.tags).await?;

        let input = InputListing::from(data);

        let item: Option<DatabaseEntityListing> = self
            .client
            .update((Collection::Listing.to_string(), id.to_string()))
            .merge(input)
            .await
            .map_err(map_db_error)?;

        match item {
            Some(e) => {
                let listing = Listing::try_from(e)?;
                if let Some((ref redis, _ttl)) = self.redis {
                    clear_listing_cache(redis, user_id).await;
                };
                debug!("listing updated");
                Ok(Some(listing))
            }
            None => Err(CoreError::Unreachable),
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn delete_listing(
        &self,
        id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Option<Listing>, CoreError> {
        let listing: Option<DatabaseEntityListing> = self
            .client
            .delete((Collection::Listing.to_string(), id.to_string()))
            .await
            .map_err(map_db_error)?;

        match listing.map(Listing::try_from) {
            Some(Ok(listing)) => {
                if let Some((ref redis, _ttl)) = self.redis {
                    clear_listing_cache(redis, user_id).await;
                };
                Ok(Some(listing))
            }
            Some(Err(e)) => {
                error!("{e}");
                Err(CoreError::Database(String::from(
                    "Listing could not be deserialised properly",
                )))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self, files), err(Debug))]
    async fn upload_images(&self, files: &[&[u8]]) -> Result<Vec<(String, String)>, CoreError> {
        let bucket = &self.storage_bucket;

        let futs = files.iter().map(|value| {
            let uid = Uuid::now_v7();
            let id = format!("/{}", uid);
            bucket
                .put_object(id.to_owned(), value)
                .and_then(move |resp| async move {
                    if let Ok(s) = resp.as_str() {
                        trace!("{s}");
                    }
                    Ok((uid.to_string(), uid.to_string()))
                })
        });

        futures_util::future::try_join_all(futs)
            .await
            .map_err(|e| CoreError::Other(e.to_string()))
    }
}

#[derive(serde::Serialize)]
struct InputListing<'a> {
    title: &'a str,
    description: &'a str,
    price: f32,
    category_id: RecordId,
    image_url: &'a str,
    other_images: &'a [String],
    active: bool,
    negotiable: bool,
    tags: Vec<RecordId>,
    location_id: RecordId,
    condition_id: RecordId,
    created_at: &'a OffsetDateTime,
    updated_at: Option<&'a OffsetDateTime>,
    expires_at: Option<&'a OffsetDateTime>,
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
            price: value.price,
            category_id: record("category", &value.category_id),
            other_images: &value.other_images,
            active: value.published,
            negotiable: value.negotiable,
            condition_id: record("listing_condition", &value.condition_id),
            location_id: record("region", &value.location_id),
            created_at: &value.created_at,
            deleted_at: value.deleted_at.as_ref(),
            updated_at: value.updated_at.as_ref(),
            expires_at: value.expires_at.as_ref(),
        }
    }
}

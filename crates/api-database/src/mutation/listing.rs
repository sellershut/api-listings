use crate::{
    collections::Collection,
    entity::listing::DatabaseEntityListing,
    graphql_requests::{find_category_by_id, find_user_by_id},
    redis::{cache_keys::CacheKey, PoolLike, PooledConnectionLike, RedisPool},
};
use api_core::{
    api::{CoreError, MutateListings},
    reexports::uuid::Uuid,
    Listing,
};
use futures_util::TryFutureExt;
use rust_decimal::Decimal;
use time::OffsetDateTime;
use tracing::{debug, error, instrument, trace};

use crate::{map_db_error, Client};

async fn check_listing_validity(
    client: &Client,
    category_id: &Uuid,
    user_id: &Uuid,
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
        category_id: &Uuid,
        condition_id: &Uuid,
        quantity: usize,
    ) -> Result<Listing, CoreError> {
        check_listing_validity(self, category_id, user_id).await?;

        let input = InputListing::from(listing);
        trace!("creating listing");

        // TODO: fix location
        let mut item = self
            .client
            .query(
                "BEGIN TRANSACTION;
            LET $listing = (CREATE ONLY listing:uuid() CONTENT {
                title: type::string($title),
                description: type::string($description),
                image_url: type::string($img_url),
                price: type::decimal($price),
                other_images: [],
                active: type::bool($active),
                negotiable: type::bool($negotiable),
                updated: time::now(),
                deleted: NULL,
                expires: IF type::is::none($expires) OR type::is::null($expires) THEN
                            NULL
                         ELSE
                            type::datetime($expires)
                         END
            });
            LET $condition_id = type::thing($condition_tbl, $condition_id);
            LET $category_id = type::thing($category_tbl, $category_id);
            LET $listing_id = $listing.id;
            LET $user_node = type::thing($user_tbl, $user_id);
            RELATE $user_node->sells->$listing_id CONTENT {
                 in: $user_node,
                 quantity: type::int($quantity),
                 out: $listing_id
            };
            RELATE $listing_id->inCategory->$category_id CONTENT {
                 in: $listing_id,
                 out: $category_id
            };
            RELATE $listing_id->withCondition->$condition_id CONTENT {
                 in: $listing_id,
                 out: $condition_id
            };
            RETURN $listing;
            COMMIT TRANSACTION;",
            )
            .bind(("title", input.title))
            .bind(("description", input.description))
            .bind(("img_url", input.image_url))
            .bind(("price", input.price))
            .bind(("category_tbl", Collection::Category))
            .bind(("category_id", category_id.to_string()))
            // .bind(("other_img", input.title))
            .bind(("active", input.active))
            .bind(("negotiable", input.negotiable))
            .bind(("condition_id", condition_id.to_string()))
            .bind(("condition_tbl", Collection::ListingCondition))
            .bind(("expires", input.expires_at))
            .bind(("user_id", user_id.to_string()))
            .bind(("region_tbl", "region"))
            .bind(("quantity", quantity))
            .bind(("user_tbl", Collection::User))
            .await
            .map_err(map_db_error)?;

        let resp: Option<DatabaseEntityListing> = item.take(0).map_err(map_db_error)?;

        match resp {
            Some(e) => {
                let listing = Listing::try_from(e)?;
                if let Some((ref redis, _ttl)) = self.redis {
                    clear_listing_cache(redis, user_id).await;
                };
                trace!("listing created");
                debug!("listing content: {:?}", listing);
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
        category_id: &Uuid,
        condition_id: &Uuid,
        quantity: usize,
    ) -> Result<Option<Listing>, CoreError> {
        check_listing_validity(self, category_id, user_id).await?;

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
    price: &'a Decimal,
    image_url: &'a str,
    other_images: &'a [String],
    active: bool,
    negotiable: bool,
    created_at: &'a OffsetDateTime,
    updated_at: &'a OffsetDateTime,
    expires_at: Option<&'a OffsetDateTime>,
    deleted_at: Option<&'a OffsetDateTime>,
}

impl<'a> From<&'a Listing> for InputListing<'a> {
    fn from(value: &'a Listing) -> Self {
        Self {
            title: &value.title,
            image_url: &value.image_url,
            description: &value.description,
            price: &value.price,
            other_images: &value.other_images,
            active: value.published,
            negotiable: value.negotiable,
            created_at: &value.created,
            deleted_at: value.deleted.as_ref(),
            updated_at: &value.updated,
            expires_at: value.expires.as_ref(),
        }
    }
}

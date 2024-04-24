mod condition;
mod tags;

use api_core::{
    api::{CoreError, QueryListings},
    reexports::uuid::Uuid,
    Listing,
};
use meilisearch_sdk::{SearchQuery, SearchResults};
use tracing::{debug, error, instrument};

use crate::{
    collections::Collection,
    entity::{create_thing_from_id, listing::DatabaseEntityListing},
    map_db_error,
    redis::{cache_keys::CacheKey, redis_query},
    Client,
};

async fn db_get_listings(
    db: &Client,
    wait_for_indexing: bool,
) -> Result<std::vec::IntoIter<Listing>, CoreError> {
    let listings = if let Some((ref redis, ttl)) = db.redis {
        let cache_key = CacheKey::AllListings;
        let listings = redis_query::query::<Vec<Listing>>(cache_key, redis).await;

        if let Some(listings) = listings {
            listings
        } else {
            let listings: Vec<DatabaseEntityListing> = db
                .client
                .select(Collection::Listing)
                .await
                .map_err(map_db_error)?;

            let listings = listings
                .into_iter()
                .map(Listing::try_from)
                .collect::<Result<Vec<Listing>, CoreError>>()?;

            if let Err(e) = redis_query::update(cache_key, redis, &listings, ttl).await {
                error!(key = %cache_key, "[redis update]: {e}");
            }

            listings
        }
    } else {
        let listings: Vec<DatabaseEntityListing> = db
            .client
            .select(Collection::Listing)
            .await
            .map_err(map_db_error)?;
        listings
            .into_iter()
            .map(Listing::try_from)
            .collect::<Result<Vec<Listing>, CoreError>>()?
    };

    if let Some(ref client) = db.search_client {
        debug!("indexing listings for search");
        let index = client.index("listings");
        let res = index
            .add_documents(&listings, Some("id"))
            .await
            .map_err(|e| CoreError::Other(e.to_string()))?;

        if wait_for_indexing {
            debug!("waiting for completion");
            let _res = res.wait_for_completion(client, None, None).await;
        }
    }

    Ok(listings.into_iter())
}

async fn get_listings_by_field(
    db: &Client,
    field: &str,
    id: &Uuid,
) -> Result<std::vec::IntoIter<Listing>, CoreError> {
    let collection = Collection::from(field);
    let field_id_value = create_thing_from_id(collection, id);
    let cache_key = match collection {
        Collection::Listing => todo!(),
        Collection::User => CacheKey::UserListing { user_id: id },
        Collection::Tag => todo!(),
        Collection::ListingCondition => todo!(),
    };

    if let Some((ref redis, ttl)) = db.redis {
        let listings = redis_query::query::<Vec<Listing>>(cache_key, redis).await;

        if let Some(listings) = listings {
            Ok(listings.into_iter())
        } else {
            let mut listings = db
                .client
                .query(format!(
                    "SELECT * FROM type::table($table) WHERE {field} = type::string($value)"
                ))
                .bind(("table", Collection::Listing))
                .bind(("value", field_id_value))
                .await
                .map_err(map_db_error)?;

            let listings: Vec<DatabaseEntityListing> = listings.take(0).map_err(map_db_error)?;

            let items: Result<Vec<Listing>, _> =
                listings.into_iter().map(Listing::try_from).collect();

            let listings = items?;

            if let Err(e) = redis_query::update(cache_key, redis, &listings, ttl).await {
                error!(key = %cache_key, "[redis update]: {e}");
            }
            Ok(listings.into_iter())
        }
    } else {
        let mut listings = db
            .client
            .query(format!(
                "SELECT * FROM type::table($table) WHERE {field} = type::string($value)"
            ))
            .bind(("table", Collection::Listing))
            .bind(("value", field_id_value))
            .await
            .map_err(map_db_error)?;

        let listings: Vec<DatabaseEntityListing> = listings.take(0).map_err(map_db_error)?;

        let items: Result<Vec<Listing>, _> = listings.into_iter().map(Listing::try_from).collect();

        let listings = items?;

        Ok(listings.into_iter())
    }
}

impl QueryListings for Client {
    #[instrument(skip(self), err(Debug))]
    async fn get_listings(&self) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        db_get_listings(self, false).await
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_listing_by_id(&self, listing_id: &Uuid) -> Result<Option<Listing>, CoreError> {
        let id = create_thing_from_id(Collection::Listing, listing_id);
        if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::Listing { id: listing_id };

            let listing = redis_query::query::<Option<Listing>>(cache_key, redis).await;

            if let Some(listing) = listing {
                Ok(listing)
            } else {
                let listing: Option<DatabaseEntityListing> =
                    self.client.select(id).await.map_err(map_db_error)?;
                let listing = listing.and_then(|f| match Listing::try_from(f) {
                    Ok(cat) => Some(cat),
                    Err(e) => {
                        error!("{e}");
                        None
                    }
                });

                if let Err(e) = redis_query::update(cache_key, redis, listing.as_ref(), ttl).await {
                    error!(key = %cache_key, "[redis update]: {e}");
                }
                Ok(listing)
            }
        } else {
            let listing: Option<DatabaseEntityListing> =
                self.client.select(id).await.map_err(map_db_error)?;
            let listing = listing.and_then(|f| match Listing::try_from(f) {
                Ok(cat) => Some(cat),
                Err(e) => {
                    error!("{e}");
                    None
                }
            });

            Ok(listing)
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_listings_from_user(
        &self,
        user_id: &Uuid,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        get_listings_by_field(self, "user_id", user_id).await
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_listings_in_category(
        &self,
        category_id: &Uuid,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        get_listings_by_field(self, "category_id", category_id).await
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_listings_in_price_range(
        &self,
        min: f32,
        max: f32,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        let listings = if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::AllListings;
            let listings = redis_query::query::<Vec<Listing>>(cache_key, redis).await;

            if let Some(listings) = listings {
                listings
            } else {
                let mut listings = self
                    .client
                    .query( "SELECT * FROM {} where price >= type::decimal($min) and price <= type::decimal($max)").bind(("table", Collection::Listing))
                    .bind(("min", min))
                    .bind(("max", max))
                    .await
                    .map_err(map_db_error)?;

                let listings: Vec<DatabaseEntityListing> =
                    listings.take(0).map_err(map_db_error)?;

                let listings = listings
                    .into_iter()
                    .map(Listing::try_from)
                    .collect::<Result<Vec<Listing>, CoreError>>()?;

                if let Err(e) = redis_query::update(cache_key, redis, &listings, ttl).await {
                    error!(key = %cache_key, "[redis update]: {e}");
                }

                listings
            }
        } else {
            let mut listings = self
                .client
                .query("SELECT * FROM type::table($table) where price >= type::decimal($min) and price <= type::decimal($max)")
                .bind(("table", Collection::Listing))
                .bind(("min", min))
                .bind(("max", max))
                .await
                .map_err(map_db_error)?;

            let listings: Vec<DatabaseEntityListing> = listings.take(0).map_err(map_db_error)?;

            listings
                .into_iter()
                .map(Listing::try_from)
                .collect::<Result<Vec<Listing>, CoreError>>()?
        };

        Ok(listings.into_iter())
    }

    #[instrument(skip(self), err(Debug))]
    async fn search(
        &self,
        query: impl AsRef<str> + Send + std::fmt::Debug,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        if let Some(ref client) = self.search_client {
            let mut index = None;
            for _retries in 0..3 {
                if let Ok(idx) = client.get_index("listings").await {
                    index = Some(idx);
                    break;
                }
                let _categories = db_get_listings(self, true).await?;
            }
            match index {
                Some(index) => {
                    let query = SearchQuery::new(&index).with_query(query.as_ref()).build();

                    let results: SearchResults<Listing> = index
                        .execute_query(&query)
                        .await
                        .map_err(|e| CoreError::Other(e.to_string()))?;

                    let search_results: Vec<Listing> = results
                        .hits
                        .into_iter()
                        .map(|hit| Listing {
                            id: hit.result.id,
                            title: hit.result.title,
                            description: hit.result.description,
                            negotiable: hit.result.negotiable,
                            price: hit.result.price,
                            category_id: hit.result.category_id,
                            image_url: hit.result.image_url,
                            condition_id: hit.result.condition_id,
                            expires_at: hit.result.expires_at,
                            other_images: hit.result.other_images,
                            published: hit.result.published,
                            tags: hit.result.tags,
                            location_id: hit.result.location_id,
                            created_at: hit.result.created_at,
                            updated_at: hit.result.updated_at,
                            deleted_at: hit.result.deleted_at,
                        })
                        .collect();

                    Ok(search_results.into_iter())
                }
                None => Err(CoreError::Other(
                    "items could not be indexed for search".into(),
                )),
            }
        } else {
            Err(CoreError::Other(String::from(
                "no client configured for search",
            )))
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_listings_with_tags(
        &self,
        tags: &[&Uuid],
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        let tags_vals: Vec<_> = tags
            .iter()
            .map(|f| create_thing_from_id(Collection::Tag, f))
            .collect();

        let mut query = self
            .client
            .query("SELECT * FROM type::table($table) WHERE tags CONTAINSANY type::array($values)")
            .bind(("table", Collection::Listing))
            .bind(("values", &tags_vals))
            .await
            .map_err(map_db_error)?;

        let result: Vec<DatabaseEntityListing> = query.take(0).map_err(map_db_error)?;
        let result = result
            .into_iter()
            .map(Listing::try_from)
            .collect::<Result<Vec<Listing>, CoreError>>()?;

        Ok(result.into_iter())
    }
}
/*
impl Client {
    pub async fn live_listing(&self) {
        let val: surrealdb::method::Stream<
            '_,
            surrealdb::engine::remote::ws::Client,
            Vec<DatabaseEntityListing>,
        > = self
            .client
            .select(Collection::Listing)
            .live()
            .await
            .unwrap();
        let res = val.map(|item| item.map(|item| Listing::try_from(item.data)));
    }
} */

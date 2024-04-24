use api_core::{
    api::{CoreError, QueryListingCondition},
    ListingCondition,
};
use tracing::{error, instrument};

use crate::{
    collections::Collection,
    entity::condition::DatabaseEntityListingCondition,
    map_db_error,
    redis::{cache_keys::CacheKey, redis_query},
    Client,
};

async fn db_get_conditions(db: &Client) -> Result<std::vec::IntoIter<ListingCondition>, CoreError> {
    let conditions = if let Some((ref redis, ttl)) = db.redis {
        let cache_key = CacheKey::AllConditions;
        let conditions = redis_query::query::<Vec<ListingCondition>>(cache_key, redis).await;

        if let Some(conditions) = conditions {
            conditions
        } else {
            let conditions: Vec<DatabaseEntityListingCondition> = db
                .client
                .select(Collection::ListingCondition)
                .await
                .map_err(map_db_error)?;

            let conditions = conditions
                .into_iter()
                .map(ListingCondition::try_from)
                .collect::<Result<Vec<ListingCondition>, CoreError>>()?;

            if let Err(e) = redis_query::update(cache_key, redis, &conditions, ttl).await {
                error!(key = %cache_key, "[redis update]: {e}");
            }

            conditions
        }
    } else {
        let conditions: Vec<DatabaseEntityListingCondition> = db
            .client
            .select(Collection::ListingCondition)
            .await
            .map_err(map_db_error)?;
        conditions
            .into_iter()
            .map(ListingCondition::try_from)
            .collect::<Result<Vec<ListingCondition>, CoreError>>()?
    };

    Ok(conditions.into_iter())
}

impl QueryListingCondition for Client {
    #[instrument(skip(self), err(Debug))]
    async fn get_conditions(
        &self,
    ) -> Result<impl ExactSizeIterator<Item = ListingCondition>, CoreError> {
        db_get_conditions(self).await
    }
}

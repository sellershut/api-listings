use api_core::{
    api::{CoreError, MutateTags},
    Tag,
};
use tracing::{debug, error, instrument};
use uuid::Uuid;

use crate::{
    collections::Collection,
    entity::tag::DatabaseEntityTag,
    map_db_error,
    redis::{cache_keys::CacheKey, PoolLike, PooledConnectionLike, RedisPool},
    Client,
};

async fn clear_tag_cache(redis: &RedisPool) {
    match redis.get().await {
        Ok(mut pool) => {
            let mut pipe = redis::Pipeline::new();

            pipe.del(CacheKey::AllTags);

            if let Err(e) = pool.query_async_pipeline::<()>(pipe).await {
                error!("{e}");
            }
        }
        Err(e) => {
            error!("{e}");
        }
    }
}

impl MutateTags for Client {
    #[instrument(skip(self), err(Debug))]
    async fn create_tag(&self, tag: &Tag) -> Result<Tag, CoreError> {
        let input = InputTag::from(tag);

        let id = Uuid::now_v7();
        let item: Option<DatabaseEntityTag> = self
            .client
            .create((Collection::Tag.to_string(), id.to_string()))
            .content(input)
            .await
            .map_err(map_db_error)?;

        match item {
            Some(e) => {
                let tag = Tag::try_from(e)?;
                if let Some((ref redis, _ttl)) = self.redis {
                    clear_tag_cache(redis).await;
                };
                debug!("tag created");
                Ok(tag)
            }
            None => Err(CoreError::Unreachable),
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn update_tag(&self, id: &Uuid, data: &Tag) -> Result<Option<Tag>, CoreError> {
        let input = InputTag::from(data);

        let item: Option<DatabaseEntityTag> = self
            .client
            .update((Collection::Tag.to_string(), id.to_string()))
            .merge(input)
            .await
            .map_err(map_db_error)?;

        match item {
            Some(e) => {
                let listing = Tag::try_from(e)?;
                if let Some((ref redis, _ttl)) = self.redis {
                    clear_tag_cache(redis).await;
                };
                debug!("tag updated");
                Ok(Some(listing))
            }
            None => Err(CoreError::Unreachable),
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn delete_tag(&self, id: &Uuid) -> Result<Option<Tag>, CoreError> {
        let tag: Option<DatabaseEntityTag> = self
            .client
            .delete((Collection::Tag.to_string(), id.to_string()))
            .await
            .map_err(map_db_error)?;

        match tag.map(Tag::try_from) {
            Some(Ok(listing)) => {
                if let Some((ref redis, _ttl)) = self.redis {
                    clear_tag_cache(redis).await;
                };
                Ok(Some(listing))
            }
            Some(Err(e)) => {
                error!("{e}");
                Err(CoreError::Database(String::from(
                    "Tag could not be deserialised properly",
                )))
            }
            None => Ok(None),
        }
    }
}

#[derive(serde::Serialize)]
struct InputTag<'a> {
    name: &'a str,
}

impl<'a> From<&'a Tag> for InputTag<'a> {
    fn from(value: &'a Tag) -> Self {
        Self { name: &value.name }
    }
}

use api_core::{
    api::{CoreError, QueryTags},
    Tag,
};
use meilisearch_sdk::{SearchQuery, SearchResults};
use tracing::{debug, error, instrument};
use uuid::Uuid;

use crate::{
    collections::Collection,
    entity::{create_thing_from_id, tag::DatabaseEntityTag},
    map_db_error,
    redis::{cache_keys::CacheKey, redis_query},
    Client,
};

async fn db_get_tags(
    db: &Client,
    wait_for_indexing: bool,
) -> Result<std::vec::IntoIter<Tag>, CoreError> {
    let tags = if let Some((ref redis, ttl)) = db.redis {
        let cache_key = CacheKey::AllTags;
        let tags = redis_query::query::<Vec<Tag>>(cache_key, redis).await;

        if let Some(tags) = tags {
            tags
        } else {
            let tags: Vec<DatabaseEntityTag> = db
                .client
                .select(Collection::Tag)
                .await
                .map_err(map_db_error)?;

            let tags = tags
                .into_iter()
                .map(Tag::try_from)
                .collect::<Result<Vec<Tag>, CoreError>>()?;

            if let Err(e) = redis_query::update(cache_key, redis, &tags, ttl).await {
                error!(key = %cache_key, "[redis update]: {e}");
            }

            tags
        }
    } else {
        let tags: Vec<DatabaseEntityTag> = db
            .client
            .select(Collection::Tag)
            .await
            .map_err(map_db_error)?;
        tags.into_iter()
            .map(Tag::try_from)
            .collect::<Result<Vec<Tag>, CoreError>>()?
    };

    if let Some(ref client) = db.search_client {
        debug!("indexing tags for search");
        let index = client.index("tags");
        let res = index
            .add_documents(&tags, Some("id"))
            .await
            .map_err(|e| CoreError::Other(e.to_string()))?;

        if wait_for_indexing {
            debug!("waiting for completion");
            let _res = res.wait_for_completion(client, None, None).await;
        }
    }

    Ok(tags.into_iter())
}

impl QueryTags for Client {
    #[instrument(skip(self), err(Debug))]
    async fn get_tags(&self) -> Result<impl ExactSizeIterator<Item = Tag>, CoreError> {
        db_get_tags(self, false).await
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_tag_by_id(&self, tag_id: &Uuid) -> Result<Option<Tag>, CoreError> {
        let id = create_thing_from_id(Collection::Tag, tag_id);
        if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::Tag { id: tag_id };

            let tag = redis_query::query::<Option<Tag>>(cache_key, redis).await;

            if let Some(tag) = tag {
                Ok(tag)
            } else {
                let tag: Option<DatabaseEntityTag> =
                    self.client.select(id).await.map_err(map_db_error)?;
                let tag = tag.and_then(|f| match Tag::try_from(f) {
                    Ok(cat) => Some(cat),
                    Err(e) => {
                        error!("{e}");
                        None
                    }
                });

                if let Err(e) = redis_query::update(cache_key, redis, tag.as_ref(), ttl).await {
                    error!(key = %cache_key, "[redis update]: {e}");
                }
                Ok(tag)
            }
        } else {
            let tag: Option<DatabaseEntityTag> =
                self.client.select(id).await.map_err(map_db_error)?;
            let tag = tag.and_then(|f| match Tag::try_from(f) {
                Ok(cat) => Some(cat),
                Err(e) => {
                    error!("{e}");
                    None
                }
            });

            Ok(tag)
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn search(
        &self,
        query: impl AsRef<str> + Send + std::fmt::Debug,
    ) -> Result<impl ExactSizeIterator<Item = Tag>, CoreError> {
        if let Some(ref client) = self.search_client {
            let mut index = None;
            for _retries in 0..3 {
                if let Ok(idx) = client.get_index("tags").await {
                    index = Some(idx);
                    break;
                }
                let _tags = db_get_tags(self, true).await?;
            }
            match index {
                Some(index) => {
                    let query = SearchQuery::new(&index).with_query(query.as_ref()).build();

                    let results: SearchResults<Tag> = index
                        .execute_query(&query)
                        .await
                        .map_err(|e| CoreError::Other(e.to_string()))?;

                    let search_results: Vec<Tag> = results
                        .hits
                        .into_iter()
                        .map(|hit| Tag {
                            id: hit.result.id,
                            name: hit.result.name,
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
}

use std::sync::Arc;

use api_core::api::CoreError;
use s3::Bucket;
use thiserror::Error;

mod collections;
pub(crate) mod entity;
mod file_storage;
mod graphql_requests;
mod mutation;
mod query;
mod redis;
pub use file_storage::S3Config;

use surrealdb::{
    engine::remote::ws::{Client as SurrealClient, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::{instrument, trace};

use self::redis::RedisPool;

pub(crate) fn map_db_error(error: surrealdb::Error) -> CoreError {
    CoreError::Database(error.to_string())
}

pub struct Client {
    client: Surreal<SurrealClient>,
    redis: Option<(RedisPool, u64)>,
    search_client: Option<meilisearch_sdk::Client>,
    http_client: reqwest::Client,
    users_api: Arc<str>,
    categories_api: Arc<str>,
    storage_bucket: Bucket,
}

impl Client {
    #[instrument(skip_all)]
    pub async fn with_redis(&mut self, dsn: &str, is_cluster: bool, pool_size: u16, ttl: u64) {
        trace!("connecting to redis");
        self.redis = Some((
            if is_cluster {
                redis::new_redis_pool_clustered(dsn, pool_size).await
            } else {
                redis::new_redis_pool(dsn, pool_size).await
            },
            ttl,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(skip_all)]
    pub async fn try_new(
        dsn: &str,
        username: &str,
        password: &str,
        namespace: &str,
        database: &str,
        users_api: &str,
        categories_api: &str,
        s3_config: &S3Config,
    ) -> Result<Self, ClientError> {
        let bucket = file_storage::start(s3_config).await?;
        trace!("connecting to database");
        let db = Surreal::new::<Ws>(dsn).await?;

        // Signin as a namespace, database, or root user
        db.signin(Root { username, password }).await?;

        db.use_ns(namespace).use_db(database).await?;

        let http_client = reqwest::Client::new();

        Ok(Client {
            client: db,
            search_client: None,
            redis: None,
            http_client,
            users_api: users_api.into(),
            categories_api: categories_api.into(),
            storage_bucket: bucket,
        })
    }

    #[instrument(skip_all)]
    pub fn with_meilisearch(&mut self, host: &str, api_key: Option<impl Into<String>>) {
        trace!("connecting to meilisearch");
        self.search_client = Some(meilisearch_sdk::Client::new(host, api_key));
    }
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("database engine error")]
    Engine(#[from] surrealdb::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error(transparent)]
    Bucket(#[from] s3::error::S3Error),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[cfg(test)]
mod tests;

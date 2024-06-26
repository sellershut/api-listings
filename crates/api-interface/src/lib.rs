use api_database::Client;
use async_graphql::{extensions::ExtensionFactory, Schema, SchemaBuilder};
use thiserror::Error;
use tracing::{info, instrument, trace};

use self::graphql::{mutation::Mutation, query::Query, subscription::Subscription};

pub mod graphql;

pub use api_database::S3Config;

#[derive(Debug, Clone, Copy)]
pub struct DatabaseCredentials<'a> {
    pub db_dsn: &'a str,
    pub db_user: &'a str,
    pub db_pass: &'a str,
    pub db_ns: &'a str,
    pub db: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct RedisConfig<'a> {
    pub redis_dsn: &'a str,
    pub clustered: bool,
    pub pool_size: u16,
    pub ttl: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Apis<'a> {
    pub users: &'a str,
    pub categories: &'a str,
}

pub struct ApiSchemaBuilder {
    builder: SchemaBuilder<Query, Mutation, Subscription>,
}

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error(transparent)]
    DatabaseError(#[from] api_database::ClientError),
}

impl ApiSchemaBuilder {
    #[instrument(skip_all, fields(db.url = %database.db_dsn), name = "schema.init")]
    pub async fn new(
        database: DatabaseCredentials<'_>,
        redis: Option<RedisConfig<'_>>,
        meilisearch: Option<(&str, Option<&str>)>,
        apis: Apis<'_>,
        s3_config: &S3Config,
    ) -> Result<Self, SchemaError> {
        trace!("creating database client");

        let mut db_client = Client::try_new(
            database.db_dsn,
            database.db_user,
            database.db_pass,
            database.db_ns,
            database.db,
            apis.users,
            apis.categories,
            s3_config,
        )
        .await?;

        if let Some((host, api_key)) = meilisearch {
            db_client.with_meilisearch(host, api_key);
        }

        if let Some(redis) = redis {
            db_client
                .with_redis(redis.redis_dsn, redis.clustered, redis.pool_size, redis.ttl)
                .await;
        }

        info!("database database client created");

        let builder = Self {
            builder: Schema::build(
                Query::default(),
                Mutation::default(),
                Subscription::default(),
            )
            .data(db_client),
        };

        Ok(builder)
    }

    #[instrument(skip(self, extension), name = "schema.ext")]
    pub fn with_extension(self, extension: impl ExtensionFactory) -> Self {
        trace!("attaching extension to schema");
        Self {
            builder: self.builder.extension(extension),
        }
    }

    #[instrument(skip(self), name = "schema.build")]
    pub fn build(self) -> Schema<Query, Mutation, Subscription> {
        trace!("building schema");
        self.builder.finish()
    }
}

#[cfg(test)]
mod tests;

pub mod env;

use anyhow::{Ok, Result};
use api_interface::{Apis, DatabaseCredentials, RedisConfig, S3Config};
use metrics_exporter_prometheus::PrometheusHandle;
use tracing::{error, instrument, warn};

use crate::telemetry::metrics::setup_metrics_recorder;

pub struct AppState {
    pub port: u16,
    database_dsn: String,
    database_username: String,
    database_password: String,
    database_namespace: String,
    database_name: String,
    pub frontend_url: String,
    pub metrics_handle: PrometheusHandle,
    redis_dsn: String,
    redis_clustered: bool,
    db_pool_size: u16,
    cache_ttl: u64,
    meilisearch_host: String,
    meilisearch_api_key: Option<String>,
    api_users: String,
    api_categories: String,
    s3_config: S3Config,
}

impl AppState {
    #[instrument(name = "env.cfg")]
    pub fn try_from_env() -> Result<AppState> {
        let port: u16 = env::extract_variable("PORT", "3001").parse()?;

        let (dsn, db_name, db_user, db_pass, db_ns, redis_host, redis_is_cluster) = {
            if cfg!(test) {
                (
                    "TEST_DATABASE_URL",
                    "TEST_DATABASE_NAME",
                    "TEST_DATABASE_USERNAME",
                    "TEST_DATABASE_PASSWORD",
                    "TEST_DATABASE_NAMESPACE",
                    "TEST_REDIS_HOST",
                    "TEST_REDIS_CLUSTER",
                )
            } else {
                (
                    "DATABASE_DSN",
                    "DATABASE_NAME",
                    "DATABASE_USERNAME",
                    "DATABASE_PASSWORD",
                    "DATABASE_NAMESPACE",
                    "REDIS_HOST",
                    "REDIS_CLUSTER",
                )
            }
        };

        let database_dsn = env::extract_variable(dsn, "localhost:8000");

        #[cfg(test)]
        let database_dsn = database_dsn.replace("http://", "");

        let database_username = env::extract_variable(db_user, "");
        let database_password = env::extract_variable(db_pass, "");
        let database_namespace = env::extract_variable(db_ns, "");
        let database_name = env::extract_variable(db_name, "");
        let frontend_url = env::extract_variable("FRONTEND_URL", "http://localhost:5173");
        let redis_dsn = env::extract_variable(redis_host, "redis://localhost:6379");
        let redis_clustered = env::extract_variable(redis_is_cluster, "false");
        let pool_size = env::extract_variable("DB_POOL_SIZE", "10");
        let cache_ttl = env::extract_variable("CACHE_TTL_MS", "5000");

        let meilisearch_host = env::extract_variable("MEILISEARCH_HOST", "http://localhost:7700");
        let meilisearch_api_key = env::extract_variable("MEILISEARCH_API_KEY", "");
        let meilisearch_api_key = if meilisearch_api_key.is_empty() {
            None
        } else {
            Some(meilisearch_api_key)
        };

        let api_users = env::extract_variable("API_SELLERSHUT_USERS", "http://localhost:3001");
        let api_categories =
            env::extract_variable("API_SELLERSHUT_CATEGORIES", "http://localhost:3000");

        let metrics_handle = setup_metrics_recorder()?;

        let bucket_name = env::extract_variable("S3_BUCKET_NAME", "sh-listings");
        let bucket_region = env::extract_variable("S3_BUCKET_REGION", "eu-central-1");
        let bucket_endpoint = env::extract_variable("S3_BUCKET_ENDPOINT", "http://localhost:19000");

        Ok(AppState {
            api_users,
            api_categories,
            port,
            database_dsn,
            database_username,
            database_password,
            database_name,
            database_namespace,
            frontend_url,
            metrics_handle,
            redis_dsn,
            meilisearch_host,
            meilisearch_api_key,
            redis_clustered: redis_clustered.parse().unwrap_or_else(|_| {
                warn!("REDIS_CLUSTER is not a boolean value");
                false
            }),
            db_pool_size: pool_size.parse().unwrap_or_else(|_| {
                error!(
                    val = pool_size,
                    default = 10,
                    "connection pool size invalid"
                );
                10
            }),
            cache_ttl: cache_ttl.parse().unwrap_or_else(|_| {
                error!(val = cache_ttl, default = 5000, "cache ttl invalid");
                5000
            }),
            s3_config: S3Config {
                bucket_name,
                region: bucket_region,
                endpoint: bucket_endpoint,
                access_key: std::env::var("S3_ACCESS_KEY").ok(),
                secret_key: std::env::var("S3_SECRET_KEY").ok(),
            },
        })
    }

    pub fn database_credentials(&self) -> DatabaseCredentials {
        DatabaseCredentials {
            db_dsn: &self.database_dsn,
            db_user: &self.database_username,
            db_pass: &self.database_password,
            db_ns: &self.database_namespace,
            db: &self.database_name,
        }
    }

    pub fn apis(&self) -> Apis {
        Apis {
            users: &self.api_users,
            categories: &self.api_categories,
        }
    }

    pub fn meilisearch_credentials(&self) -> (&str, Option<&str>) {
        (&self.meilisearch_host, self.meilisearch_api_key.as_deref())
    }

    pub fn redis_credentials(&self) -> RedisConfig {
        RedisConfig {
            redis_dsn: &self.redis_dsn,
            clustered: self.redis_clustered,
            pool_size: self.db_pool_size,
            ttl: self.cache_ttl,
        }
    }

    pub fn bucket_details(&self) -> &S3Config {
        &self.s3_config
    }
}

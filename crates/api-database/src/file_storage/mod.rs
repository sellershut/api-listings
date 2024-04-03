use s3::creds::Credentials;
use s3::{Bucket, BucketConfiguration, Region};
use surrealdb::sql::Uuid;
use tracing::{info, warn};

use crate::ClientError;

#[derive(Clone, Debug)]
pub struct S3Config {
    pub bucket_name: String,
    pub region: String,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub endpoint: String,
}

pub async fn start(config: &S3Config) -> Result<Bucket, ClientError> {
    // TODO: check minio-rs crate for v0.2.0
    let bucket_name = &config.bucket_name;
    let region = || Region::Custom {
        region: config.region.to_owned(),
        endpoint: config.endpoint.to_owned(),
    };
    let credentials = || Credentials {
        access_key: config.access_key.clone(),
        secret_key: config.secret_key.clone(),
        session_token: None,
        security_token: None,
        expiration: None,
    };

    let mut bucket = match Bucket::new(bucket_name, region(), credentials()) {
        Ok(bucket) => bucket.with_path_style(),
        Err(e) => {
            warn!("{e}");
            Bucket::create_with_path_style(
                bucket_name,
                region(),
                credentials(),
                BucketConfiguration::public(),
            )
            .await?
            .bucket
        }
    };

    let content = "I want to go to S3".as_bytes();
    let id = format!("/sh-listings-test-{}", Uuid::new_v7());

    match bucket.put_object(&id, content).await {
        Ok(_) => {
            if let Err(e) = bucket.delete_object(&id).await {
                warn!("{e}");
                bucket = Bucket::create_with_path_style(
                    bucket_name,
                    region(),
                    credentials(),
                    BucketConfiguration::public(),
                )
                .await?
                .bucket;
            };
        }
        Err(err) => {
            warn!("{err}");
            bucket = Bucket::create_with_path_style(
                bucket_name,
                region(),
                credentials(),
                BucketConfiguration::public(),
            )
            .await?
            .bucket;
        }
    }
    bucket.put_object(&id, content).await?;
    bucket.delete_object(&id).await?;

    info!("bucket is ready");

    Ok(bucket)
}

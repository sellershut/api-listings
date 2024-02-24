use crate::{
    redis::{cache_keys::CacheKey, PoolLike, PooledConnectionLike},
    tests::create_client,
    Client,
};
use anyhow::Result;
use api_core::{api::QueryListings, reexports::uuid::Uuid, Listing};

async fn check_listings_by_id(client: Client, id: &Uuid, expected_result: bool) -> Result<()> {
    match client.get_listing_by_id(id).await {
        Ok(res) => {
            assert_eq!(res.is_some(), expected_result);
        }
        Err(_) => {
            assert!(!expected_result);
        }
    }

    Ok(())
}

async fn check_all(expected_result: bool) -> Result<()> {
    let client = create_client(None, false, false).await?;

    let res = client.get_listings().await;

    assert_eq!(res.is_ok(), expected_result);

    Ok(())
}

#[tokio::test]
async fn query_by_unavailable_id() -> Result<()> {
    let client = create_client(None, false, false).await?;
    check_listings_by_id(client, &Uuid::now_v7(), false).await?;

    let client = create_client(None, true, false).await?;
    check_listings_by_id(client, &Uuid::now_v7(), false).await?;

    Ok(())
}

#[tokio::test]
async fn query_by_available_id() -> Result<()> {
    let client = create_client(None, false, false).await?;

    let mut res = client
        .client
        .query("SELECT * FROM listing LIMIT 5;")
        .await?;

    let resp: Vec<Listing> = res.take(0)?;

    if let Some(item) = resp.first() {
        check_listings_by_id(client, &item.id, true).await?;
    }

    Ok(())
}

#[tokio::test]
async fn query_with_meilisearch() -> Result<()> {
    let client = create_client(None, true, true).await?;

    if let Some((ref redis, _)) = client.redis {
        let mut redis = redis.get().await?;
        redis.del::<_, ()>(CacheKey::AllListings).await?;
    }

    let _results: Vec<_> = client.get_listings().await?.collect();
    let res = client.search("some thing").await;
    assert!(res.is_ok());

    Ok(())
}

#[tokio::test]
async fn query_all() -> Result<()> {
    check_all(true).await?;

    Ok(())
}

mod async_graphql;
mod db;

use crate::{tests::db::SampleDbSend, Listing};

use self::db::SampleDb;
use time::OffsetDateTime;
use uuid::Uuid;

impl Default for Listing {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id: Uuid::now_v7(),
            title: String::from("My listing"),
            description: String::from("More info about listing"),
            price: 250.50,
            category_id: Uuid::now_v7(),
            image_url: String::from("https://dummyimage.com/420x260"),
            other_images: vec![],
            active: true,
            tags: vec![],
            location_id: String::default(),
            liked_by: vec![],
            created_at: OffsetDateTime::now_utc(),
            deleted_at: None,
            updated_at: None,
        }
    }
}

#[test]
fn encode() {
    let listing = Listing::default();

    let json = serde_json::to_string(&listing).unwrap();
    dbg!(&json);
    let bytes = bincode::serialize(&listing).unwrap();

    let value = serde_json::from_str::<Listing>(&json);
    dbg!(&value);

    assert!(value.is_ok());
    let val: Listing = bincode::deserialize(&bytes[..]).unwrap();
    assert_eq!(val, listing);
}

#[test]
fn deserialise_list() {
    let listing = Listing::default();

    let listing_2 = Listing::default();

    let listings = vec![listing, listing_2];

    let str_val = serde_json::to_string(&listings);

    let bytes = bincode::serialize(&listings).unwrap();

    let source = bincode::deserialize::<Vec<Listing>>(&bytes[..]).unwrap();

    dbg!(&str_val);

    assert!(str_val.is_ok());
    assert_eq!(source, listings);
}

#[tokio::test]
async fn trait_blank_queries() {
    use crate::api::LocalQueryListings;

    let db = SampleDb.get_listings().await;
    assert!(db.is_ok());

    let generated_id = Uuid::now_v7();
    let db = SampleDb.get_listing_by_id(&generated_id).await;
    assert!(db.is_ok());

    let db = SampleDb.get_listings().await;
    assert!(db.is_ok());

    let db = SampleDb.get_listings_from_user(&generated_id).await;
    assert!(db.is_ok());

    let db = SampleDb.get_listings_in_category(&generated_id).await;
    assert!(db.is_ok());
}

#[tokio::test]
async fn trait_blank_mutations() {
    use crate::api::LocalMutateListings;

    let listing = Listing::default();

    let db = SampleDb.create_listing(&listing).await;
    assert!(db.is_ok());

    let id = Uuid::now_v7();
    let db = SampleDb.update_listing(&id, &listing).await;
    assert!(db.is_ok());

    let db = SampleDb.delete_listing(&id).await;
    assert!(db.is_ok());
}

#[tokio::test]
async fn mutation_returns_send() {
    use crate::api::MutateListings;

    let listing = Listing::default();

    let id = Uuid::now_v7();
    let db = SampleDbSend.create_listing(&listing).await;
    assert!(db.is_ok());

    let db = SampleDbSend.update_listing(&id, &listing).await;
    assert!(db.is_ok());

    let db = SampleDbSend.delete_listing(&id).await;
    assert!(db.is_ok());
}

#[tokio::test]
async fn query_returns_send() {
    use crate::api::QueryListings;

    let db = SampleDbSend.get_listings().await;
    assert!(db.is_ok());

    let generated_id = Uuid::now_v7();
    let db = SampleDbSend.get_listing_by_id(&generated_id).await;
    assert!(db.is_ok());

    let db = SampleDbSend.get_listings().await;
    assert!(db.is_ok());

    let db = SampleDbSend.get_listings_from_user(&generated_id).await;
    assert!(db.is_ok());

    let db = SampleDbSend.get_listings_in_category(&generated_id).await;
    assert!(db.is_ok());
}

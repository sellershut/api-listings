use crate::tests::external_mutation::{
    create_category::Variables,
    create_sample_category, create_sample_user,
    create_user::{self, UserType},
};

use super::create_client;
use anyhow::Result;
use api_core::{
    api::{MutateListings, QueryListings},
    reexports::uuid::Uuid,
    Listing,
};
use time::OffsetDateTime;

fn create_listing_item() -> Listing {
    Listing {
        id: Uuid::now_v7(),
        user_id: Uuid::now_v7(),
        title: "Test Listing".to_owned(),
        description: "Test description".to_owned(),
        price: 25.4,
        category_id: Uuid::now_v7(),
        image_url: "http://testpicture.com".to_owned(),
        other_images: vec![],
        published: true,
        tags: vec![],
        location_id: String::default(),
        liked_by: vec![],
        created_at: OffsetDateTime::now_utc(),
        updated_at: None,
        deleted_at: None,
    }
}

fn check_similarities(source: &Listing, dest: &Listing) {
    assert_eq!(source.user_id, dest.user_id);
    assert_eq!(source.title, dest.title);
    assert_eq!(source.description, dest.description);
    assert_eq!(source.price, dest.price);
    assert_eq!(source.created_at, dest.created_at);
}

#[tokio::test]
async fn create_listing() -> Result<()> {
    let client = create_client(Some("test-mutation-create"), false, false).await?;
    let mut listing = create_listing_item();
    let res = client.create_listing(&listing).await;

    let all_listings = client.get_listings().await?;

    let base_count = all_listings.count();

    assert!(res.is_err()); // no category

    let uuid = create_sample_category(
        &client.http_client,
        Variables {
            name: "TestCategory".to_owned(),
        },
    )
    .await
    .expect("Category to be created via post request");

    listing.category_id = uuid;

    let res = client.create_listing(&listing).await;

    assert!(res.is_err()); // no user
    let uuid = create_sample_user(
        &client.http_client,
        create_user::Variables {
            username: "TestCategory".to_owned(),
            user_type: Some(UserType::INDIVIDUAL),
        },
    )
    .await
    .expect("User to be created via post request");

    listing.user_id = uuid;

    let input = client.create_listing(&listing).await?;

    let updated_listings = client.get_listings().await?;

    assert_eq!(base_count + 1, updated_listings.count());
    check_similarities(&input, &listing);

    let get_by_id = client.get_listing_by_id(&input.id).await?;
    assert_eq!(get_by_id, Some(input.clone()));

    let mut update = input.clone();
    let new_title = "FooBar".to_string();
    update.title = new_title.clone();

    // This ID does exist
    let update_res = client
        .update_listing(&input.id, &update)
        .await?
        .expect("listing to exist in db");

    assert_eq!(update_res.id, input.id);
    assert_eq!(update_res.title, new_title);

    client.delete_listing(&input.id).await?;
    Ok(())
}

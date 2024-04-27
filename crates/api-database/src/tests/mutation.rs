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
use fake::{
    faker::{internet::en::Username, lorem::en::Words},
    Fake,
};
use time::OffsetDateTime;

fn create_listing_item() -> Listing {
    let title: Vec<String> = Words(3..5).fake();
    let title = title.join(" ");

    let description: Vec<String> = Words(10..50).fake();
    let description = description.join(" ");

    Listing {
        id: Uuid::now_v7(),
        title,
        description,
        price: 250.50,
        category_id: Uuid::now_v7(),
        image_url: String::from("https://dummyimage.com/420x260"),
        other_images: vec![],
        published: true,
        location_id: Uuid::now_v7(),
        created: OffsetDateTime::now_utc(),
        deleted: None,
        updated: OffsetDateTime::now_utc(),
        condition_id: Uuid::now_v7(),
        negotiable: true,
        expires: None,
    }
}

fn check_similarities(source: &Listing, dest: &Listing) {
    assert_eq!(source.title, dest.title);
    assert_eq!(source.description, dest.description);
    assert_eq!(source.price, dest.price);
    assert_eq!(source.created, dest.created);
}

#[tokio::test]
async fn create_listing() -> Result<()> {
    let client = create_client(Some("test-mutation-create"), false, false).await?;
    let mut listing = create_listing_item();
    let user_id = Uuid::now_v7();
    let res = client.create_listing(&listing, &user_id).await;

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

    let res = client.create_listing(&listing, &user_id).await;

    assert!(res.is_err()); // no user
    let user_id = create_sample_user(
        &client.http_client,
        create_user::Variables {
            username: Username().fake(),
            user_type: Some(UserType::INDIVIDUAL),
        },
    )
    .await
    .expect("User to be created via post request");

    let input = client.create_listing(&listing, &user_id).await?;

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
        .update_listing(&input.id, &update, &user_id)
        .await?
        .expect("listing to exist in db");

    assert_eq!(update_res.id, input.id);
    assert_eq!(update_res.title, new_title);

    client.delete_listing(&input.id, &user_id).await?;
    Ok(())
}

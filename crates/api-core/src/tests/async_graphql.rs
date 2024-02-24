use async_graphql::{EmptySubscription, Object, Schema};

use crate::Listing;

struct Root;

#[Object]
impl Root {
    async fn output(&self) -> Listing {
        Listing::default()
    }

    async fn input(&self, listing: Listing) -> Listing {
        listing
    }
}

#[tokio::test]
async fn gql_query() {
    let schema = Schema::new(Root, Root, EmptySubscription);

    let res = schema
        .execute(
            r#"
              query {
                output {
                  title
                }
              }
            "#,
        )
        .await;

    dbg!(&res);

    assert!(res.errors.is_empty());
}

#[tokio::test]
async fn gql_mutation() {
    let schema = Schema::new(Root, Root, EmptySubscription);

    let res = schema
        .execute(
            r#"
              mutation {
                input (listing: {
                    userId: "7c503531-2900-4fb3-b4ac-203f7bb6ac2f",
                    categoryId: "7c503531-2910-4fb3-b4ac-203f7bb6ac2f",
                    title: "Title",
                    description: "Desc",
                    price: 34.3,
                    imageUrl: "url",
                    active: true,
                    location: "",
                }) {
                  id
                }
              }
            "#,
        )
        .await;

    dbg!(&res);

    assert!(res.errors.is_empty());
}

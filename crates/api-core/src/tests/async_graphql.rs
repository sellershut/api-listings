use async_graphql::{EmptySubscription, Object, Schema};

use crate::Listing;

struct Root;

#[Object]
impl Root {
    async fn output(&self) -> Listing {
        Listing::default()
    }

    async fn input(&self, category: Listing) -> Listing {
        category
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
                  name
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
                input (category: {name: "Lorem"}) {
                  name
                }
              }
            "#,
        )
        .await;

    dbg!(&res);

    assert!(res.errors.is_empty());
}

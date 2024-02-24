#[tokio::test]
async fn gql_query() -> Result<(), Box<dyn std::error::Error>> {
    let schema = super::init_schema().await;

    let res = schema
        .execute(
            r#"
           query {
             listings(first: 2) {
               edges{
                 cursor
                 node{
                   id,
                   title
                 }
               },
               pageInfo {
                 hasNextPage,
                 hasPreviousPage
               }
             }
           }
           "#,
        )
        .await;

    assert!(res.errors.is_empty());

    Ok(())
}

#[tokio::test]
async fn gql_query_category_by_id_ok() {
    let schema = super::init_schema().await;

    let res = schema
        .execute(
            r#"
           query {
             listingById(id: "018d930d-073c-73c2-b9d6-24f1461c18d3") {
               id,
               title
             }
           }
           "#,
        )
        .await;

    dbg!(&res.errors);
    assert!(res.errors.is_empty());
}

#[tokio::test]
async fn gql_search_ok() -> Result<(), Box<dyn std::error::Error>> {
    let schema = super::init_schema().await;

    let res = schema
        .execute(
            r#"
           query {
             search(last: 2, query: "Some Text") {
               edges{
                 cursor
                 node{
                   id,
                 }
               },
               pageInfo {
                 hasNextPage,
                 hasPreviousPage
               }
             }
           }
           "#,
        )
        .await;

    // search client not configured
    assert!(!res.errors.is_empty());

    Ok(())
}

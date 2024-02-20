use std::error::Error;

use graphql_client::{GraphQLQuery, Response};
use tracing::trace;

#[allow(clippy::upper_case_acronyms)]
type UUID = uuid::Uuid;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql_externals/categories-schema.json",
    query_path = "src/graphql_externals/categories_query.graphql",
    response_derives = "Debug"
)]
#[allow(non_camel_case_types)]
pub struct categoryById;

#[tracing::instrument(skip(variables))]
async fn find_category_by_id(
    categories_api: &str,
    variables: category_by_id::Variables,
) -> Result<bool, Box<dyn Error>> {
    let request_body = categoryById::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post(categories_api)
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<category_by_id::ResponseData> = res.json().await?;

    let located = if let Some(resp) = response_body.data {
        if let Some(val) = resp.category_by_id {
            trace!("found category {}", val.id);
            true
        } else {
            false
        }
    } else {
        false
    };
    Ok(located)
}

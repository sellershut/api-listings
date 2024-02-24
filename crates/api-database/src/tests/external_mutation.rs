use api_core::api::CoreError;
use graphql_client::{GraphQLQuery, Response};
use uuid::Uuid;

#[allow(clippy::upper_case_acronyms)]
type UUID = uuid::Uuid;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql_externals/categories-schema.json",
    query_path = "src/graphql_externals/categories_mutation.graphql",
    response_derives = "Debug"
)]
#[allow(non_camel_case_types)]
pub struct createCategory;

pub(crate) async fn create_sample_category(
    client: &reqwest::Client,
    variables: create_category::Variables,
) -> Result<Uuid, CoreError> {
    let request_body = createCategory::build_query(variables);

    let map_err = |e: reqwest::Error| CoreError::Other(e.to_string());
    let api = std::env::var("TEST_SELLERSHUT_API_CATEGORIES")
        .expect("TEST_SELLERSHUT_API_CATEGORIES variable");

    let req = client
        .post(api)
        .json(&request_body)
        .build()
        .map_err(map_err)?;

    let resp = client.execute(req).await.map_err(map_err)?;

    let response_body: Response<create_category::ResponseData> =
        resp.json().await.map_err(map_err)?;

    if let Some(resp) = response_body.data {
        return Ok(resp.create_category.id);
    }
    Err(CoreError::Other("Could not create category".to_owned()))
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql_externals/users-schema.json",
    query_path = "src/graphql_externals/users_mutation.graphql",
    response_derives = "Debug"
)]
#[allow(non_camel_case_types)]
pub struct createUser;

pub(crate) async fn create_sample_user(
    client: &reqwest::Client,
    variables: create_user::Variables,
) -> Result<Uuid, CoreError> {
    let request_body = createUser::build_query(variables);

    let map_err = |e: reqwest::Error| CoreError::Other(e.to_string());
    let api =
        std::env::var("TEST_SELLERSHUT_API_USERS").expect("TEST_SELLERSHUT_API_USERS variable");

    let req = client
        .post(api)
        .json(&request_body)
        .build()
        .map_err(map_err)?;

    let resp = client.execute(req).await.map_err(map_err)?;

    let response_body: Response<create_user::ResponseData> = resp.json().await.map_err(map_err)?;

    if let Some(resp) = response_body.data {
        return Ok(resp.create_user.id);
    }
    Err(CoreError::Other("Could not create user".to_owned()))
}

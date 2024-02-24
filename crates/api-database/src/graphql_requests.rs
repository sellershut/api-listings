use api_core::api::CoreError;
use graphql_client::{GraphQLQuery, Response};
use opentelemetry::global;
use opentelemetry_http::HeaderInjector;
use tracing::{trace, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql_externals/users-schema.json",
    query_path = "src/graphql_externals/users_query.graphql",
    response_derives = "Debug"
)]
#[allow(non_camel_case_types)]
pub struct userById;

#[tracing::instrument(skip(variables))]
pub(crate) async fn find_category_by_id(
    client: &reqwest::Client,
    categories_api: &str,
    variables: category_by_id::Variables,
) -> Result<bool, CoreError> {
    let context = Span::current().context();
    let request_body = categoryById::build_query(variables);

    let map_err = |e: reqwest::Error| CoreError::Other(e.to_string());

    let mut req = client
        .post(categories_api)
        .json(&request_body)
        .build()
        .map_err(map_err)?;

    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&context, &mut HeaderInjector(req.headers_mut()))
    });

    let resp = client.execute(req).await.map_err(map_err)?;

    let response_body: Response<category_by_id::ResponseData> =
        resp.json().await.map_err(map_err)?;

    let located = if let Some(resp) = response_body.data {
        if let Some(val) = resp.category_by_id {
            trace!("found category: {}", val.id);
            true
        } else {
            false
        }
    } else {
        false
    };
    Ok(located)
}

#[tracing::instrument(skip(variables))]
pub(crate) async fn find_user_by_id(
    client: &reqwest::Client,
    users_api: &str,
    variables: user_by_id::Variables,
) -> Result<bool, CoreError> {
    let context = Span::current().context();
    let request_body = userById::build_query(variables);

    let map_err = |e: reqwest::Error| CoreError::Other(e.to_string());

    let mut req = client
        .post(users_api)
        .json(&request_body)
        .build()
        .map_err(map_err)?;

    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&context, &mut HeaderInjector(req.headers_mut()))
    });

    let resp = client.execute(req).await.map_err(map_err)?;

    let response_body: Response<user_by_id::ResponseData> = resp.json().await.map_err(map_err)?;

    let located = if let Some(resp) = response_body.data {
        if let Some(val) = resp.user_by_id {
            trace!("found user: {}", val.id);
            true
        } else {
            false
        }
    } else {
        false
    };
    Ok(located)
}

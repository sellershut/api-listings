mod routes;
mod state;
mod telemetry;

#[cfg(test)]
mod tests;

use std::future::ready;

use anyhow::Result;
use async_graphql::extensions::Tracing;
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    http::{header, HeaderValue, Method},
    middleware,
    routing::get,
    Router,
};
use tokio::signal;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::routes::{
    handler,
    middleware::{graphql::Metrics, track_metrics},
};

const SUBSCRIPTION_ENDPOINT: &str = "/ws";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let _sentry_guard = telemetry::initialise()?;

    let state = state::AppState::try_from_env()?;

    let port = state.port;

    let router = create_router(state).await?;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn create_router(state: state::AppState) -> Result<Router> {
    let schema_builder = api_interface::ApiSchemaBuilder::new(
        state.database_credentials(),
        Some(state.redis_credentials()),
        Some(state.meilisearch_credentials()),
        state.apis(),
        state.bucket_details(),
    )
    .await?
    .with_extension(Tracing)
    .with_extension(Metrics);

    let schema = schema_builder.build();

    let router = Router::new()
        .route("/", get(handler).post_service(GraphQL::new(schema.clone())))
        .route(
            "/metrics",
            get(move || ready(state.metrics_handle.render())),
        )
        .route_service(SUBSCRIPTION_ENDPOINT, GraphQLSubscription::new(schema))
        .route_layer(middleware::from_fn(track_metrics))
        // If you want to customize the behavior using closures here is how.
        .layer(TraceLayer::new_for_http().make_span_with(make_span))
        .layer(
            CorsLayer::new()
                .allow_origin(state.frontend_url.parse::<HeaderValue>()?)
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_methods([Method::GET, Method::POST]),
        );

    Ok(router)
}

fn make_span(request: &axum::http::Request<axum::body::Body>) -> tracing::Span {
    let headers = request.headers();

    let header_map: std::collections::HashMap<String, String> = headers
        .iter()
        .map(|(key, value)| {
            (
                key.to_string(),
                String::from_utf8_lossy(value.as_bytes()).to_string(),
            )
        })
        .collect();

    let parent_ctx = opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.extract(&header_map)
    });

    let span = tracing::info_span!("listings.request", ?headers);
    span.set_parent(parent_ctx);
    span
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

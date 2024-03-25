use api_core::{api::QueryListings, reexports::uuid::Uuid, Listing};
use async_graphql::{Context, Object};
use tracing::instrument;

use crate::graphql::{extract_db, query::Params};

use super::{pagination::paginate, ConnectionResult};

#[derive(Default, Debug)]
pub struct ListingQuery;

#[Object]
impl ListingQuery {
    #[instrument(skip(ctx), err(Debug))]
    async fn listings(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<Listing> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let listings = database.get_listings().await?;

        paginate(listings, p, 100).await
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn listing_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<Listing>> {
        let database = extract_db(ctx)?;

        database.get_listing_by_id(&id).await.map_err(|e| e.into())
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn listing_in_category(
        &self,
        ctx: &Context<'_>,
        category_id: Uuid,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<Listing> {
        let p = Params::new(after, before, first, last)?;
        let database = extract_db(ctx)?;

        let listings = database.get_listings_in_category(&category_id).await?;

        paginate(listings, p, 100).await
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn search(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] query: String,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<Listing> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let listings = database.search(&query).await?;

        paginate(listings, p, 100).await
    }
}

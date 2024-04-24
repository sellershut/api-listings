use api_core::{api::QueryListingCondition, reexports::uuid::Uuid, ListingCondition};
use async_graphql::{Context, Object};
use tracing::instrument;

use crate::graphql::{extract_db, query::Params};

use super::{pagination::paginate, ConnectionResult};

#[derive(Default, Debug)]
pub struct ListingConditionQuery;

#[Object]
impl ListingConditionQuery {
    #[instrument(skip(ctx), err(Debug))]
    async fn conditions(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<ListingCondition> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let tags = database.get_conditions().await?;

        paginate(tags, p, 100).await
    }
}

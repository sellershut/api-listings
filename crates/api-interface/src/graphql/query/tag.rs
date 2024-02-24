use api_core::{api::QueryTags, reexports::uuid::Uuid, Tag};
use async_graphql::{Context, Object};
use tracing::instrument;

use crate::graphql::{extract_db, query::Params};

use super::{pagination::paginate, ConnectionResult};

#[derive(Default, Debug)]
pub struct TagQuery;

#[Object]
impl TagQuery {
    #[instrument(skip(ctx), err(Debug))]
    async fn tags(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<Tag> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let tags = database.get_tags().await?;

        paginate(tags, p, 100).await
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn tag_by_id(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Option<Tag>> {
        let database = extract_db(ctx)?;

        database.get_tag_by_id(&id).await.map_err(|e| e.into())
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
    ) -> ConnectionResult<Tag> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let tags = database.search(&query).await?;

        paginate(tags, p, 100).await
    }
}

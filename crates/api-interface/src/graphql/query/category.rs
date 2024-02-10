use api_core::{api::QueryCategories, reexports::uuid::Uuid, Category};
use async_graphql::{Context, Object, SimpleObject};
use tracing::instrument;

use crate::graphql::{extract_db, query::Params};

use super::{pagination::paginate, ConnectionResult};

#[derive(Default, Debug)]
pub struct CategoryQuery;

#[derive(SimpleObject)]
pub struct SearchResult {
    category: Category,
    parent_name: Option<String>,
}

#[Object]
impl CategoryQuery {
    #[instrument(skip(ctx), err(Debug))]
    async fn categories(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<Category> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let categories = database.get_categories().await?;

        paginate(categories, p, 100).await
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn sub_categories(
        &self,
        ctx: &Context<'_>,
        parent_id: Option<Uuid>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<Category> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let categories = database.get_sub_categories(parent_id.as_ref()).await?;

        paginate(categories, p, 100).await
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn category_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<Category>> {
        let database = extract_db(ctx)?;

        database.get_category_by_id(&id).await.map_err(|e| e.into())
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
    ) -> ConnectionResult<Category> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let categories = database.search(&query).await?;

        paginate(categories, p, 100).await
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn search_with_parent_name(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] query: String,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<SearchResult> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;
        let res = database.search_with_parent_name(&query).await?;
        let mapped = res.into_iter().map(|(category, parent_name)| SearchResult {
            category,
            parent_name,
        });

        paginate(mapped, p, 100).await
    }
}

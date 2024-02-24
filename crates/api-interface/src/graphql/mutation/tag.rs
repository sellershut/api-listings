use api_core::{
    api::{MutateTags, Uuid},
    Tag,
};
use api_database::Client;
use async_graphql::{Context, Object};
use tracing::instrument;

#[derive(Default, Debug)]
pub struct TagMutation;

#[Object]
impl TagMutation {
    #[instrument(skip(ctx), err(Debug))]
    async fn create_tag(&self, ctx: &Context<'_>, input: Tag) -> async_graphql::Result<Tag> {
        let database = ctx.data::<Client>()?;

        match database.create_tag(&input).await {
            Ok(tag) => Ok(tag),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn update_tag(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: Tag,
    ) -> async_graphql::Result<Option<Tag>> {
        let database = ctx.data::<Client>()?;

        match database.update_tag(&id, &input).await {
            Ok(tag) => Ok(tag),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn delete_tag(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Option<Tag>> {
        let database = ctx.data::<Client>()?;

        match database.delete_tag(&id).await {
            Ok(listing) => Ok(listing),
            Err(e) => Err(e.into()),
        }
    }
}

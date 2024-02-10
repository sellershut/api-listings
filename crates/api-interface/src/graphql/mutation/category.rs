use api_core::{
    api::{MutateCategories, Uuid},
    Category,
};
use api_database::Client;
use async_graphql::{Context, Object};
use tracing::instrument;

use crate::graphql::subscription::{broker::SimpleBroker, CategoryChanged};

#[derive(Default, Debug)]
pub struct CategoryMutation;

#[Object]
impl CategoryMutation {
    #[instrument(skip(ctx), err(Debug))]
    async fn create_category(
        &self,
        ctx: &Context<'_>,
        input: Category,
    ) -> async_graphql::Result<Category> {
        let database = ctx.data::<Client>()?;

        match database.create_category(&input).await {
            Ok(category) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: super::MutationType::Created,
                    id: category.id,
                });

                Ok(category)
            }
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn update_category(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: Category,
    ) -> async_graphql::Result<Option<Category>> {
        let database = ctx.data::<Client>()?;

        match database.update_category(&id, &input).await {
            Ok(category) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: super::MutationType::Updated,
                    id,
                });
                Ok(category)
            }
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn delete_category(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<Category>> {
        let database = ctx.data::<Client>()?;

        match database.delete_category(&id).await {
            Ok(category) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: super::MutationType::Deleted,
                    id,
                });
                Ok(category)
            }
            Err(e) => Err(e.into()),
        }
    }
}

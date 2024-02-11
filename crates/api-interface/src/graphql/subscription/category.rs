use api_core::{api::QueryCategories, Listing};
use async_graphql::{Context, Object, Subscription};
use futures_util::{Stream, StreamExt};

use crate::graphql::{extract_db, mutation::MutationType, subscription::CategoryChanged};

use super::broker::SimpleBroker;

#[derive(Default)]
pub struct CategorySubscription;

#[Subscription]
impl CategorySubscription {
    async fn categories(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = CategoryChanged> {
        SimpleBroker::<CategoryChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}

#[Object]
impl CategoryChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn category(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Listing>> {
        let database = extract_db(ctx)?;
        let category = database.get_category_by_id(&self.id).await?;

        Ok(category)
    }
}

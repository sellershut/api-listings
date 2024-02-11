use api_core::{api::QueryListings, Listing};
use async_graphql::{Context, Object, Subscription};
use futures_util::{Stream, StreamExt};

use crate::graphql::{extract_db, mutation::MutationType, subscription::ListingChanged};

use super::broker::SimpleBroker;

#[derive(Default)]
pub struct ListingSubscription;

#[Subscription]
impl ListingSubscription {
    async fn categories(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = ListingChanged> {
        SimpleBroker::<ListingChanged>::subscribe().filter(move |event| {
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
impl ListingChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn category(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Listing>> {
        let database = extract_db(ctx)?;
        let category = database.get_listing_by_id(self.id).await?;

        Ok(category)
    }
}

use api_core::{api::QueryListings, Listing};
use async_graphql::{Context, Object, Result, Subscription};
use futures_util::Stream;

use crate::graphql::{extract_db, mutation::MutationType, subscription::ListingChanged};

#[derive(Default)]
pub struct ListingSubscription;

#[Subscription]
impl ListingSubscription {
    async fn listings<'a>(
        &'a self,
        ctx: &'a Context<'a>,
    ) -> Result<impl Stream<Item = Listing> + 'a> {
        let database = extract_db(ctx)?;
        Ok(database.live_listings().await?)
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

    async fn listing(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Listing>> {
        let database = extract_db(ctx)?;
        let listing = database.get_listing_by_id(&self.id).await?;

        Ok(listing)
    }
}

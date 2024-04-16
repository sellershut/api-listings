use api_core::{
    api::{MutateListings, Uuid},
    Listing,
};
use api_database::Client;
use async_graphql::{Context, Object};
use tracing::instrument;

use crate::graphql::subscription::{broker::SimpleBroker, ListingChanged};

#[derive(Default, Debug)]
pub struct ListingMutation;

#[Object]
impl ListingMutation {
    #[instrument(skip(ctx), err(Debug))]
    async fn create_listing(
        &self,
        ctx: &Context<'_>,
        input: Listing,
        user_id: Uuid,
    ) -> async_graphql::Result<Listing> {
        let database = ctx.data::<Client>()?;

        match database.create_listing(&input, &user_id).await {
            Ok(listing) => {
                SimpleBroker::publish(ListingChanged {
                    mutation_type: super::MutationType::Created,
                    id: listing.id,
                });

                Ok(listing)
            }
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn update_listing(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: Listing,
        user_id: Uuid,
    ) -> async_graphql::Result<Option<Listing>> {
        let database = ctx.data::<Client>()?;

        match database.update_listing(&id, &input, &user_id).await {
            Ok(listing) => {
                SimpleBroker::publish(ListingChanged {
                    mutation_type: super::MutationType::Updated,
                    id,
                });
                Ok(listing)
            }
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn delete_listing(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        user_id: Uuid,
    ) -> async_graphql::Result<Option<Listing>> {
        let database = ctx.data::<Client>()?;

        match database.delete_listing(&id, &user_id).await {
            Ok(listing) => {
                SimpleBroker::publish(ListingChanged {
                    mutation_type: super::MutationType::Deleted,
                    id,
                });
                Ok(listing)
            }
            Err(e) => Err(e.into()),
        }
    }
}

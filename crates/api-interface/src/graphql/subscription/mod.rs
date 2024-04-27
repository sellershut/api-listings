pub(crate) mod listing;
use api_core::reexports::uuid::Uuid;

use super::mutation::MutationType;

#[derive(async_graphql::MergedSubscription, Default)]
pub struct Subscription(listing::ListingSubscription);

#[derive(Debug, Clone)]
pub(crate) struct ListingChanged {
    pub mutation_type: MutationType,
    pub id: Uuid,
}

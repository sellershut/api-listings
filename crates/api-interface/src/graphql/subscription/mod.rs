pub(crate) mod broker;
pub(crate) mod category;
use api_core::reexports::uuid::Uuid;

use super::mutation::MutationType;

#[derive(async_graphql::MergedSubscription, Default)]
pub struct Subscription(category::CategorySubscription);

#[derive(Debug, Clone)]
pub(crate) struct CategoryChanged {
    pub mutation_type: MutationType,
    pub id: Uuid,
}

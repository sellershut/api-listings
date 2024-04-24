use api_core::{api::CoreError, reexports::uuid::Uuid, ListingCondition};
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

use super::create_string_from_id;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DatabaseEntityListingCondition {
    pub id: RecordId,
    pub condition: String,
}

impl TryFrom<DatabaseEntityListingCondition> for ListingCondition {
    type Error = CoreError;

    fn try_from(entity: DatabaseEntityListingCondition) -> Result<Self, Self::Error> {
        let pk = create_string_from_id(&entity.id);
        let id = Uuid::parse_str(&pk)?;

        Ok(ListingCondition {
            id,
            condition: entity.condition,
        })
    }
}

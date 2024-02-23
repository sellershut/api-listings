use api_core::{api::CoreError, reexports::uuid::Uuid, Tag};
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

use super::create_string_from_id;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DatabaseEntityTag {
    pub id: RecordId,
    pub name: String,
}

impl TryFrom<DatabaseEntityTag> for Tag {
    type Error = CoreError;

    fn try_from(entity: DatabaseEntityTag) -> Result<Self, Self::Error> {
        let pk = create_string_from_id(&entity.id);
        let id = Uuid::parse_str(&pk)?;

        Ok(Tag {
            id,
            name: entity.name,
        })
    }
}

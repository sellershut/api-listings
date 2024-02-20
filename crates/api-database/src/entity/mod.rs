use surrealdb::opt::RecordId;
use uuid::Uuid;

use crate::collections::Collection;

pub(crate) mod listing;

pub(crate) mod tag;

pub(crate) fn create_thing_from_id(collection: Collection, id: &Uuid) -> RecordId {
    RecordId::from((collection.to_string(), id.to_string()))
}

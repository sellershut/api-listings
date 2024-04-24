use surrealdb::opt::RecordId;
use uuid::Uuid;

use crate::collections::Collection;

pub(crate) mod condition;
pub(crate) mod listing;

pub(crate) mod tag;

pub(crate) fn create_thing_from_id(collection: Collection, id: &Uuid) -> RecordId {
    RecordId::from((collection.to_string(), id.to_string()))
}

pub(crate) fn create_string_from_id(id: &RecordId) -> String {
    let id = id.id.to_raw();
    id.split(':')
        .next()
        .unwrap_or(&id)
        .chars()
        .filter(|&c| c != '⟨' && c != '⟩')
        .collect()
}

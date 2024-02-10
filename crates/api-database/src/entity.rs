use api_core::{api::CoreError, reexports::uuid::Uuid, Category};
use serde::{Deserialize, Serialize};
use surrealdb::{opt::RecordId, sql::Id};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DatabaseEntity {
    pub id: RecordId,
    pub name: String,
    pub sub_categories: Option<Vec<RecordId>>, // empty vec wont work for playground type
    pub image_url: Option<String>,
    pub parent_id: Option<RecordId>,
}

impl TryFrom<DatabaseEntity> for Category {
    type Error = CoreError;

    fn try_from(entity: DatabaseEntity) -> Result<Self, Self::Error> {
        let id_to_string = |id: &Id| -> String {
            let id = id.to_raw();
            id.split(':')
                .next()
                .unwrap_or(&id)
                .chars()
                .filter(|&c| c != '⟨' && c != '⟩')
                .collect()
        };

        let pk = id_to_string(&entity.id.id);
        let id = Uuid::parse_str(&pk)?;

        let sub_categories = entity.sub_categories.map_or(Ok(vec![]), |sub_categories| {
            sub_categories
                .into_iter()
                .map(|record_id| Uuid::parse_str(&id_to_string(&record_id.id)))
                .collect::<Result<Vec<Uuid>, _>>()
        })?;

        let parent_id = entity
            .parent_id
            .map_or(Ok::<Option<Result<Uuid, _>>, Self::Error>(None), |f| {
                Ok(Some(Uuid::parse_str(&id_to_string(&f.id))))
            })?;

        Ok(Category {
            id,
            name: entity.name,
            sub_categories: if sub_categories.is_empty() {
                None
            } else {
                Some(sub_categories)
            },
            image_url: entity.image_url,
            parent_id: match parent_id {
                Some(parent_id) => Some(parent_id?),
                None => None,
            },
        })
    }
}

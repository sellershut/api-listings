use api_core::{api::CoreError, reexports::uuid::Uuid, Listing};
use serde::{Deserialize, Serialize};
use surrealdb::{opt::RecordId, sql::Id};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DatabaseListing {
    pub id: RecordId,
    pub user_id: RecordId,
    pub title: String,
    pub description: String,
    pub price: f32,
    pub category_id: RecordId,
    pub image_url: String,
    pub other_images: Option<Vec<String>>,
    pub active: bool,
    pub tags: Option<Vec<RecordId>>,
    pub location: String,
    pub likes: usize,
    pub created_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl TryFrom<DatabaseListing> for Listing {
    type Error = CoreError;

    fn try_from(entity: DatabaseListing) -> Result<Self, Self::Error> {
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

        let user_id_fk = id_to_string(&entity.user_id.id);
        let user_id = Uuid::parse_str(&user_id_fk)?;

        let category_id_fk = id_to_string(&entity.category_id.id);
        let category_id = Uuid::parse_str(&category_id_fk)?;

        let tags = entity.tags.map_or(Ok(vec![]), |tags| {
            tags.into_iter()
                .map(|record_id| Uuid::parse_str(&id_to_string(&record_id.id)))
                .collect::<Result<Vec<Uuid>, _>>()
        })?;

        Ok(Listing {
            id,
            user_id,
            title: entity.title,
            description: entity.description,
            price: entity.price,
            category_id,
            other_images: entity.other_images,
            active: entity.active,
            location: entity.location,
            likes: entity.likes,
            created_at: entity.created_at,
            deleted_at: entity.deleted_at,
            tags: if tags.is_empty() { None } else { Some(tags) },
            image_url: entity.image_url,
        })
    }
}

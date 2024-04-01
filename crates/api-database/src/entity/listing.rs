use api_core::{api::CoreError, reexports::uuid::Uuid, Listing, ListingCondition};
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use time::OffsetDateTime;

use super::create_string_from_id;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DatabaseEntityListing {
    pub id: RecordId,
    pub user_id: RecordId,
    pub title: String,
    pub description: String,
    pub price: f32,
    pub category_id: RecordId,
    pub image_url: String,
    pub other_images: Vec<String>,
    pub active: bool,
    pub negotiable: bool,
    pub tags: Vec<RecordId>,
    pub location_id: RecordId,
    pub condition: ListingCondition,
    pub expires_at: Option<OffsetDateTime>,
    pub quantity: u32,
    pub likes: Vec<RecordId>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

impl TryFrom<DatabaseEntityListing> for Listing {
    type Error = CoreError;

    fn try_from(entity: DatabaseEntityListing) -> Result<Self, Self::Error> {
        let pk = create_string_from_id(&entity.id);
        let id = Uuid::parse_str(&pk)?;

        let user_id_fk = create_string_from_id(&entity.user_id);
        let user_id = Uuid::parse_str(&user_id_fk)?;

        let category_id_fk = create_string_from_id(&entity.category_id);
        let category_id = Uuid::parse_str(&category_id_fk)?;

        let location_id_fk = create_string_from_id(&entity.location_id);
        let location_id = Uuid::parse_str(&location_id_fk)?;

        let tags = entity
            .tags
            .into_iter()
            .map(|record_id| Uuid::parse_str(&create_string_from_id(&record_id)))
            .collect::<Result<Vec<Uuid>, _>>()?;

        let likes = entity
            .likes
            .iter()
            .map(|like| Uuid::parse_str(&create_string_from_id(like)))
            .collect::<Result<Vec<Uuid>, _>>()?;

        Ok(Listing {
            id,
            user_id,
            title: entity.title,
            description: entity.description,
            price: entity.price,
            category_id,
            other_images: entity.other_images,
            active: entity.active,
            negotiable: entity.negotiable,
            location_id,
            liked_by: likes,
            created_at: entity.created_at,
            deleted_at: entity.deleted_at,
            condition: entity.condition,
            quantity: entity.quantity,
            expires_at: entity.expires_at,
            tags,
            image_url: entity.image_url,
            updated_at: entity.updated_at,
        })
    }
}

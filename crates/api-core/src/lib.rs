pub mod api;

#[cfg(feature = "async-graphql")]
use async_graphql::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(InputObject, SimpleObject))]
#[cfg_attr(feature = "async-graphql", graphql(input_name = "ListingInput"))]
pub struct Listing {
    #[cfg_attr(feature = "async-graphql", graphql(skip_input))]
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub price: f32,
    pub category_id: Uuid,
    pub image_url: String,
    pub condition: ListingCondition,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub other_images: Vec<String>,
    pub published: bool,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub negotiable: bool,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub tags: Vec<Uuid>,
    pub location_id: Uuid,
    #[cfg_attr(
        feature = "async-graphql",
        graphql(default_with = "default_date_time()")
    )]
    pub created_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(Enum))]
pub enum ListingCondition {
    New,
    LikeNew,
    UsedExcellent,
    UsedGood,
    UsedFair,
    UsedRefurbished,
    PartsOrNotWorking,
}

#[cfg(feature = "async-graphql")]
fn default_date_time() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(InputObject, SimpleObject))]
#[cfg_attr(feature = "async-graphql", graphql(input_name = "TagInput"))]
pub struct Tag {
    #[cfg_attr(feature = "async-graphql", graphql(skip_input))]
    pub id: Uuid,
    pub name: String,
}

pub mod reexports {
    pub use uuid;
}

#[cfg(test)]
mod tests;

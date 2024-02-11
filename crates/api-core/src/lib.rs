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
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub price: f32,
    pub category_id: Uuid,
    pub image_url: String,
    pub other_images: Option<Vec<String>>,
    pub active: bool,
    pub tags: Option<Vec<Uuid>>,
    pub location: String,
    #[cfg_attr(feature = "async-graphql", graphql(skip_input))]
    pub likes: usize,
    pub created_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}


#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(InputObject, SimpleObject))]
#[cfg_attr(feature = "async-graphql", graphql(input_name = "TagInput"))]
pub struct Tag {
    #[cfg_attr(feature = "async-graphql", graphql(skip_input))]
    id: Uuid,
    name: String,
}

pub mod reexports {
    pub use uuid;
}

#[cfg(test)]
mod tests;

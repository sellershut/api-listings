pub mod api;

#[cfg(feature = "async-graphql")]
use async_graphql::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(InputObject, SimpleObject))]
#[cfg_attr(feature = "async-graphql", graphql(input_name = "CategoryInput"))]
pub struct Category {
    /// Category ID
    #[cfg_attr(feature = "async-graphql", graphql(skip_input))]
    pub id: Uuid,
    /// Category name
    pub name: String,
    /// A list of IDs that are subcategories for the current item
    pub sub_categories: Option<Vec<Uuid>>, // empty vec wont work for playground type
    /// An image representing the current ID
    pub image_url: Option<String>,
    /// Id of this category's parent
    pub parent_id: Option<Uuid>,
}

pub mod reexports {
    pub use uuid;
}

#[cfg(test)]
mod tests;

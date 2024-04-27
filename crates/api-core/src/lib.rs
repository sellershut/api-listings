pub mod api;

#[cfg(feature = "async-graphql")]
use async_graphql::*;

use rust_decimal::Decimal;
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
    pub price: Decimal,
    pub image_url: String,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub other_images: Vec<String>,
    pub published: bool,
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    pub negotiable: bool,
    #[cfg_attr(
        feature = "async-graphql",
        graphql(default_with = "default_date_time()")
    )]
    pub created: OffsetDateTime,
    pub expires: Option<OffsetDateTime>,
    #[cfg_attr(
        feature = "async-graphql",
        graphql(default_with = "default_date_time()")
    )]
    pub updated: OffsetDateTime,
    pub deleted: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(InputObject, SimpleObject))]
pub struct ListingCondition {
    pub id: Uuid,
    pub condition: String,
}

#[cfg(feature = "async-graphql")]
fn default_date_time() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub mod reexports {
    pub use uuid;
}

#[cfg(test)]
mod tests;

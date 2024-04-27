use std::fmt;

use api_core::{api::CoreError, reexports::uuid::Uuid, Listing};
use rust_decimal::Decimal;
use serde::{de, Deserialize, Deserializer, Serialize};
use surrealdb::opt::RecordId;
use time::{format_description::well_known::Iso8601, OffsetDateTime};

use super::create_string_from_id;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DatabaseEntityListing {
    pub id: RecordId,
    pub title: String,
    pub description: String,
    pub price: Decimal,
    pub image_url: String,
    pub other_images: Vec<String>,
    pub active: bool,
    pub negotiable: bool,
    #[serde(deserialize_with = "date_time_opt")]
    pub expires: Option<OffsetDateTime>,
    #[serde(deserialize_with = "deserialize_date_time")]
    pub created: OffsetDateTime,
    #[serde(deserialize_with = "deserialize_date_time")]
    pub updated: OffsetDateTime,
    #[serde(deserialize_with = "date_time_opt")]
    pub deleted: Option<OffsetDateTime>,
}

fn callback<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    OffsetDateTime::deserialize(deserializer)
}

#[derive(Debug, Deserialize)]
struct WrappedDateTime(#[serde(deserialize_with = "callback")] OffsetDateTime);

pub fn date_time_opt<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<WrappedDateTime>::deserialize(deserializer).map(
        |opt_wrapped: Option<WrappedDateTime>| {
            opt_wrapped.map(|wrapped: WrappedDateTime| wrapped.0)
        },
    )
}

fn deserialize_date_time<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct OffsetDateTimeVisitor;

    impl<'de> de::Visitor<'de> for OffsetDateTimeVisitor {
        type Value = OffsetDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a ISO8601 date")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            OffsetDateTime::parse(v, &Iso8601::DEFAULT).map_err(E::custom)
        }
    }

    deserializer.deserialize_any(OffsetDateTimeVisitor)
}

impl TryFrom<DatabaseEntityListing> for Listing {
    type Error = CoreError;

    fn try_from(entity: DatabaseEntityListing) -> Result<Self, Self::Error> {
        let pk = create_string_from_id(&entity.id);
        let id = Uuid::parse_str(&pk)?;

        Ok(Listing {
            id,
            title: entity.title,
            description: entity.description,
            price: entity.price,
            other_images: entity.other_images,
            published: entity.active,
            negotiable: entity.negotiable,
            created: entity.created,
            deleted: entity.deleted,
            expires: entity.expires,
            image_url: entity.image_url,
            updated: entity.updated,
        })
    }
}

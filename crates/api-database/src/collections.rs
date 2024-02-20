use serde::{Deserialize, Serialize};
use surrealdb::{
    opt::{IntoResource, Resource},
    sql::Table,
};

#[non_exhaustive]
#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Collection {
    Listing,
    User,
    Tag,
}

impl From<&str> for Collection {
    fn from(value: &str) -> Self {
        match value {
            "user" => Self::User,
            _ => unimplemented!("{value}"),
        }
    }
}

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Collection::Listing => "listing",
                Collection::Tag => "tag",
                Collection::User => "user",
            }
        )
    }
}

impl<R> IntoResource<Vec<R>> for Collection {
    fn into_resource(self) -> Result<Resource, surrealdb::Error> {
        Ok(Resource::Table(Table(self.to_string())))
    }
}

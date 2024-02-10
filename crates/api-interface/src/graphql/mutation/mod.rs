use std::fmt::Display;

use async_graphql::Enum;

pub(crate) mod category;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(category::CategoryMutation);

#[derive(Enum, Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) enum MutationType {
    Created,
    Updated,
    Deleted,
}

impl Display for MutationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MutationType::Created => "created",
                MutationType::Updated => "updated",
                MutationType::Deleted => "deleted",
            }
            .to_uppercase()
        )
    }
}

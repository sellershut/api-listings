use std::fmt::Display;

use api_core::reexports::uuid::Uuid;
use redis::ToRedisArgs;

#[derive(Clone, Copy)]
pub enum CacheKey<'a> {
    AllListings,
    AllTags,
    UserListing { user_id: &'a Uuid },
    Listing { id: &'a Uuid },
    Tag { id: &'a Uuid },
}

impl Display for CacheKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "listings:{}",
            match self {
                CacheKey::AllListings => "all".to_string(),
                CacheKey::UserListing { user_id } => format!("from_user={user_id}"),
                CacheKey::Listing { id } => format!("id={id}"),
                CacheKey::AllTags => {
                    "all_tags".to_string()
                }
                CacheKey::Tag { id } => {
                    format!("tag={id}")
                }
            }
        )
    }
}

impl ToRedisArgs for CacheKey<'_> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(self.to_string().as_bytes())
    }
}

use api_core::{
    api::{CoreError, MutateCategories},
    reexports::uuid::Uuid,
    Category,
};
use surrealdb::{opt::RecordId, sql::Thing};
use tracing::instrument;

use crate::{collections::Collections, entity::DatabaseEntity, map_db_error, Client};

impl MutateCategories for Client {
    #[instrument(skip(self), err(Debug))]
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError> {
        if let Some(ref parent) = category.parent_id {
            let id = Thing::from(("category", parent.to_string().as_str()));
            let item: Option<DatabaseEntity> =
                self.client.select(&id).await.map_err(map_db_error)?;
            if item.is_none() {
                return Err(CoreError::Database(format!(
                    "provided parent does not exist: {id}"
                )));
            }
        }
        let input_category = InputCategory::from(category);

        let id = Uuid::now_v7().to_string();
        let item: Option<DatabaseEntity> = self
            .client
            .create(("category", id))
            .content(input_category)
            .await
            .map_err(map_db_error)?;

        match item {
            Some(e) => Category::try_from(e),
            None => Err(CoreError::Unreachable),
        }
    }

    #[instrument(skip(self, id), err(Debug))]
    async fn update_category(
        &self,
        id: &Uuid,
        data: &Category,
    ) -> Result<Option<Category>, CoreError> {
        if let Some(ref parent) = data.parent_id {
            let id = Thing::from(("category", parent.to_string().as_str()));
            let item: Option<DatabaseEntity> =
                self.client.select(&id).await.map_err(map_db_error)?;
            if item.is_none() {
                return Err(CoreError::Database(format!(
                    "provided parent does not exist: {id}"
                )));
            }
        }

        let id = Thing::from((
            Collections::Category.to_string().as_str(),
            id.to_string().as_str(),
        ));

        let input_category = InputCategory::from(data);

        let item: Option<DatabaseEntity> = self
            .client
            .update(id)
            .content(input_category)
            .await
            .map_err(map_db_error)?;
        let res = match item {
            Some(e) => Some(Category::try_from(e)?),
            None => None,
        };

        Ok(res)
    }

    #[instrument(skip(self, id), err(Debug))]
    async fn delete_category(&self, id: &Uuid) -> Result<Option<Category>, CoreError> {
        let id = Thing::from((
            Collections::Category.to_string().as_str(),
            id.to_string().as_ref(),
        ));

        let res: Option<DatabaseEntity> = self.client.delete(id).await.map_err(map_db_error)?;
        let res = match res {
            Some(e) => Some(Category::try_from(e)?),
            None => None,
        };

        Ok(res)
    }
}

#[derive(serde::Serialize)]
struct InputCategory<'a> {
    name: &'a str,
    sub_categories: Option<Vec<RecordId>>,
    image_url: Option<&'a str>,
    parent_id: Option<RecordId>,
}

impl<'a> From<&'a Category> for InputCategory<'a> {
    fn from(value: &'a Category) -> Self {
        Self {
            name: &value.name,
            sub_categories: value.sub_categories.as_ref().map(|f| {
                f.iter()
                    .map(|str| RecordId::from(("category", str.to_string().as_str())))
                    .collect()
            }),
            image_url: value.image_url.as_deref(),
            parent_id: value
                .parent_id
                .as_ref()
                .map(|f| RecordId::from(("category", f.to_string().as_str()))),
        }
    }
}

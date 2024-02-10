use std::{fmt::Debug, str::FromStr};

use uuid::Uuid;

use crate::{
    api::{
        CoreError, LocalMutateCategories, LocalQueryCategories, MutateCategories, QueryCategories,
    },
    Category,
};

pub struct SampleDb;
pub struct SampleDbSend;

impl LocalQueryCategories for SampleDb {
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_sub_categories(
        &self,
        _id: Option<&Uuid>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_category_by_id(&self, _id: &Uuid) -> Result<Option<Category>, CoreError> {
        Ok(None)
    }

    async fn search(
        &self,
        _query: impl AsRef<str> + Debug + Send,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        Ok([].into_iter())
    }
}

impl LocalMutateCategories for SampleDb {
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError> {
        Ok(category.to_owned())
    }

    async fn update_category(
        &self,
        id: &Uuid,
        data: &Category,
    ) -> Result<Option<Category>, CoreError> {
        if id.as_ref().is_empty() {
            Err(CoreError::from_str("Id cannot be empty")?)
        } else {
            Ok(Some(data.to_owned()))
        }
    }

    async fn delete_category(&self, _id: &Uuid) -> Result<Option<Category>, CoreError> {
        Ok(None)
    }
}

impl MutateCategories for SampleDbSend {
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError> {
        Ok(category.to_owned())
    }

    async fn update_category(
        &self,
        _id: &Uuid,
        data: &Category,
    ) -> Result<Option<Category>, CoreError> {
        Ok(Some(data.to_owned()))
    }

    async fn delete_category(&self, _id: &Uuid) -> Result<Option<Category>, CoreError> {
        Ok(None)
    }
}

impl QueryCategories for SampleDbSend {
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_sub_categories(
        &self,
        _id: Option<&Uuid>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_category_by_id(&self, _id: &Uuid) -> Result<Option<Category>, CoreError> {
        Ok(None)
    }

    async fn search(
        &self,
        _query: impl AsRef<str> + Debug + Send,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        Ok([].into_iter())
    }
}

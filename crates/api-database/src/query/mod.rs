use api_core::{
    api::{CoreError, QueryListings},
    reexports::uuid::Uuid,
    Listing,
};

use crate::Client;

impl QueryListings for Client {
    async fn search(
        &self,
        query: impl AsRef<str> + Send + std::fmt::Debug,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok(vec![].into_iter())
    }

    async fn get_listings(&self) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok(vec![].into_iter())
    }

    async fn get_listing_by_id(
        &self,
        listing_id: impl Into<Uuid> + Send,
    ) -> Result<Option<Listing>, CoreError> {
        todo!()
    }

    async fn get_listings_from_user(
        &self,
        user_id: impl Into<Uuid> + Send,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok(vec![].into_iter())
    }

    async fn get_listings_from_category(
        &self,
        category_id: impl Into<Uuid> + Send,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok(vec![].into_iter())
    }

    async fn get_listings_in_price_range(
        &self,
        min: f32,
        max: f32,
    ) -> Result<impl ExactSizeIterator<Item = Listing>, CoreError> {
        Ok(vec![].into_iter())
    }
}

/* impl Client {
    pub async fn search_with_parent_name(
        &self,
        query: &str,
    ) -> Result<Vec<(Listing, Option<String>)>, CoreError> {
        if let Some(ref client) = self.search_client {
            let mut index = None;
            for _retries in 0..3 {
                if let Ok(idx) = client.get_index("categories").await {
                    index = Some(idx);
                    break;
                } else {
                    let _categories = db_get_categories(self).await?;
                }
            }
            match index {
                Some(index) => {
                    let query = SearchQuery::new(&index).with_query(query.as_ref()).build();

                    let results: SearchResults<Listing> = index
                        .execute_query(&query)
                        .await
                        .map_err(|e| CoreError::Other(e.to_string()))?;

                    let parent_ids: Vec<_> = results
                        .hits
                        .iter()
                        .filter_map(|f| f.result.parent_id.map(|parent_id| parent_id.to_string()))
                        .collect();
                    let parent_ids_str: Vec<&str> = parent_ids.iter().map(|f| f.as_str()).collect();

                    let futures = parent_ids_str
                        .iter()
                        .map(|parent_id| index.get_document::<Listing>(parent_id));

                    let res: Vec<Listing> = futures_util::future::try_join_all(futures)
                        .await
                        .map_err(|e| CoreError::Other(e.to_string()))?;

                    let search_results: Vec<_> = results
                        .hits
                        .into_iter()
                        .map(|hit| {
                            let category = Listing {
                                id: hit.result.id,
                                name: hit.result.name,
                                sub_categories: hit.result.sub_categories,
                                parent_id: hit.result.parent_id,
                                image_url: hit.result.image_url,
                            };
                            let parent = if let Some(parent_id) = hit.result.parent_id {
                                res.iter().find_map(|category| {
                                    if parent_id == category.id {
                                        Some(category.name.to_owned())
                                    } else {
                                        None
                                    }
                                })
                            } else {
                                None
                            };
                            (category, parent)
                        })
                        .collect();

                    Ok(search_results)
                }
                None => Err(CoreError::Other(
                    "items could not be indexed for search".into(),
                )),
            }
        } else {
            Err(CoreError::Other(String::from(
                "no client configured for search",
            )))
        }
    }
} */

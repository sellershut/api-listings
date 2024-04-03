use api_core::api::LocalMutateListings;
use api_database::Client;
use async_graphql::{Context, Object, Result, SimpleObject, Upload};
use tracing::instrument;

#[derive(Default, Debug)]
pub struct UploadMutation;

#[derive(Clone, SimpleObject)]
pub struct FileInfo {
    id: String,
    url: String,
}

#[Object]
impl UploadMutation {
    #[instrument(skip(ctx), err(Debug))]
    async fn upload_images(&self, ctx: &Context<'_>, files: Vec<Upload>) -> Result<Vec<FileInfo>> {
        let database = ctx.data::<Client>()?;

        let mut futs = Vec::with_capacity(files.len());

        for file in files.iter() {
            let val = file.value(ctx)?;
            futs.push(val.content.to_vec());
        }

        let slices: Vec<_> = futs.iter().map(|f| f.as_slice()).collect();
        Ok(database
            .upload_images(&slices)
            .await?
            .into_iter()
            .map(|(id, url)| FileInfo { id, url })
            .collect())
    }
}

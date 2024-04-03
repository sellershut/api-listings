use async_graphql::{Context, Object, Result, SimpleObject, Upload};
use tracing::instrument;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct UploadMutation;

#[derive(Clone, SimpleObject)]
pub struct FileInfo {
    id: Uuid,
    url: String,
}

#[Object]
impl UploadMutation {
    #[instrument(skip(ctx), err(Debug))]
    async fn upload_images(&self, ctx: &Context<'_>, files: Vec<Upload>) -> Result<Vec<FileInfo>> {
        for file in files.iter() {
            let val = file.value(ctx).unwrap();
            // val.content
        }
        todo!()
    }
}

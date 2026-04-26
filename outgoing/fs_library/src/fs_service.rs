use async_trait::async_trait;
use bytes::Bytes;
use pantsu_domain::common::result::Result;
use pantsu_domain::image::PantsuImage;
use pantsu_domain::library::LibraryService;
use pantsu_worker::worker_connection::WorkerConnectionTx;
use pantsu_worker::JobResponder;
use tokio::sync::oneshot;

pub enum FsJob {
    StoreImage(PantsuImage, Bytes, JobResponder<()>),
}

pub struct DefaultFsService {
    worker_connection: WorkerConnectionTx<FsJob>,
}

impl DefaultFsService {
    pub fn new(worker_connection: WorkerConnectionTx<FsJob>) -> Self {
        return DefaultFsService { worker_connection };
    }
}

#[async_trait]
impl LibraryService for DefaultFsService {
    async fn store_image(&self, image: PantsuImage, file_content: Bytes) -> Result<()> {
        let (sender, receiver) = oneshot::channel::<Result<()>>();
        let job = FsJob::StoreImage(image, file_content, sender);
        self.worker_connection.send_job(job).await?;
        return receiver.await?;
    }
}

use async_trait::async_trait;
use kani_domain::common::result::Result;
use kani_domain::reverse_image_search::ReverseImageSearchService;
use kani_worker::worker_connection::WorkerConnectionTx;
use kani_worker::JobResponder;
use tokio::sync::oneshot;

pub enum IqdbJob {
    GetSauce(String, JobResponder<String>)
}

pub struct DefaultIqdbService {
    worker_connection: WorkerConnectionTx<IqdbJob>,
}

impl DefaultIqdbService {
    pub fn new(worker_connection: WorkerConnectionTx<IqdbJob>) -> Self {
        return DefaultIqdbService { worker_connection }
    }
}

#[async_trait]
impl ReverseImageSearchService for DefaultIqdbService {
    async fn get_sauce(&self, image: String) -> Result<String> {
        let (sender, receiver) = oneshot::channel::<Result<String>>();
        let job = IqdbJob::GetSauce(image, sender);
        self.worker_connection.send_job(job).await?;
        return receiver.await?;
    }
}

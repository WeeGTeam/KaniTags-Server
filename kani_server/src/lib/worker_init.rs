use kani_domain_api_outgoing::reverse_image_search::ReverseImageSearchService;
use kani_iqdb::iqdb_service::DefaultIqdbService;
use kani_iqdb::iqdb_worker;
use kani_worker::worker_connection::{create_worker_connection, WorkerConnectionRx, WorkerConnectionTx};
use std::future::Future;
use tokio::task;

pub fn create_worker<J, F, P, Fut>(worker_run: F, params: P) -> WorkerConnectionTx<J>
where
    F: FnOnce(WorkerConnectionRx<J>, P) -> Fut + Send + 'static,
    Fut: Future<Output = Result<(), anyhow::Error>> + Send,
    J: Send + 'static,
    P: Send + 'static,
{
    let (connection_tx, connection_rx) = create_worker_connection::<J>(128);
    task::spawn(async move {
        let _ = worker_run(connection_rx, params).await;
        println!("oopsie") // todo: log
    });
    return connection_tx;
}

pub fn init_iqdb() -> impl ReverseImageSearchService + Send + Sync {
    let connection_tx = create_worker(iqdb_worker::worker_run, ());
    let iqdb_service = DefaultIqdbService::new(connection_tx);
    return iqdb_service;
}

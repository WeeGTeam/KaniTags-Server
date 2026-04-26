use pantsu_domain::common::error::Error;
use pantsu_domain::library::LibraryService;
use pantsu_domain::reverse_image_search::ReverseImageSearchService;
use pantsu_fs_library::fs_service::DefaultFsService;
use pantsu_fs_library::fs_worker;
use pantsu_iqdb::iqdb_service::DefaultIqdbService;
use pantsu_iqdb::iqdb_worker;
use pantsu_worker::worker_connection::{create_worker_connection, WorkerConnectionRx, WorkerConnectionTx};
use std::future::Future;
use std::path::PathBuf;
use tokio::task;

pub fn create_worker<J, F, P, Fut>(worker_run: F, params: P) -> WorkerConnectionTx<J>
where
    F: FnOnce(WorkerConnectionRx<J>, P) -> Fut + Send + 'static,
    Fut: Future<Output = Result<(), Error>> + Send,
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

pub fn init_fs(lib_path: PathBuf) -> impl LibraryService + Send + Sync {
    let connection_tx = create_worker(fs_worker::worker_run, lib_path);
    let fs_service = DefaultFsService::new(connection_tx);
    return fs_service;
}

use super::fs_service::FsJob;
use crate::library::PantsuLibrary;
use bytes::Bytes;
use pantsu_domain::common::error::Error;
use pantsu_domain::common::result::Result;
use pantsu_domain::image::PantsuImage;
use pantsu_domain::library::{Library, GALLERY_THUMBNAIL_OPTIONS};
use pantsu_worker::worker_connection::WorkerConnectionRx;
use pantsu_worker::JobResponder;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

pub async fn worker_run<'r>(
    connection_rx: WorkerConnectionRx<FsJob>,
    lib_path: PathBuf,
) -> Result<()> {
    /*loop {
        connection_rx.recv_job(handle_job).await?;
    }*/
    connection_rx
        .recv_stream(|job: FsJob| handle_job(job, lib_path.clone()), 4)
        .await
}

async fn handle_job<'r>(job: FsJob, lib_path: PathBuf) -> Result<()> {
    sleep(Duration::from_secs(1)).await;
    match job {
        FsJob::StoreImage(image, file_content, responder) => {
            let answer = handle_store_image(image, file_content, lib_path).await;
            respond(responder, answer)?;
        }
    }
    Ok(())
}

fn respond<T>(responder: JobResponder<T>, response: Result<T>) -> Result<()> {
    responder.send(response).map_err(|_| {
        Error::WorkerCommunicationError("Worker unable to send response to Service".to_string())
    })
}

async fn handle_store_image<'r>(
    image: PantsuImage,
    file_content: Bytes,
    lib_path: PathBuf,
) -> Result<()> {
    let library = PantsuLibrary::new(lib_path).await?;
    library.store_image(&image, file_content.clone()).await?;
    library.create_thumbnail(&image, file_content, GALLERY_THUMBNAIL_OPTIONS).await?;
    Ok(())
}

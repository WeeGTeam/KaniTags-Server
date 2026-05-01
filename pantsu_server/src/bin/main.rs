use pantsu_domain::{common::error::Error, image_management::image_management_service::ImageManagementServiceImpl};
use pantsu_domain::reverse_image_search::ReverseImageSearchService;
use pantsu_http_api::launch_server;
use pantsu_lib::config::ServerConfig;
use pantsu_lib::log::setup_logger;
use pantsu_lib::worker_init;
use std::sync::Arc;
use tracing::{debug, info, Level};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger(Level::DEBUG);
    let config = ServerConfig::load_config().map_err(|_| Error::TodoError())?;
    println!("{:?}", config);
    debug!("{:?}", config);

    let iqdb_service = worker_init::init_iqdb();
    let sauce = iqdb_service.get_sauce("Megumin".to_string()).await?;
    info!("the sauce of {} is {}", "Megumin", sauce);

    let fs_service = worker_init::init_fs(config.library_path.clone());

    let image_management_service = ImageManagementServiceImpl::new(
        Arc::new(fs_service),
    );

    /*let stream_service = worker_init::init_iqdb();
    let mut sauce_jobs: FuturesUnordered<_> = (1..512)
        .map({
            let ss = &stream_service;
            move |i| async move {
                let sauce = ss.get_sauce(format!("Megumin {}", i)).await.unwrap();
                println!("The sauce of {} is {}", i, sauce);
            }
        }).collect();
    while let Some(_) = sauce_jobs.next().await {}*/

    /* for i in 1..512 {
        task::spawn(async {
            let req = format!("Megumin {}", i);
            let sauce = stream_service.get_sauce(req.to_string()).await.unwrap();
            println!("The sauce of {} is {}", req, sauce);
        });
    }; */

    /*
    stream::iter(1..512)
        .map(|num| format!("Megumin {}", num))
        .map(|req| (req, stream_service.get_sauce(req)))
        .for_each_concurrent(512, |(req, sauce)| async {
            println!("The sauce of {} is {}", req, sauce.await.unwrap());
        }).await;
    */

    launch_server(
        Arc::new(iqdb_service),
        Arc::new(image_management_service),
        config.request_body_limit.as_u64() as usize,
        config.server_port,
    )
    .await?;

    Ok(())
}

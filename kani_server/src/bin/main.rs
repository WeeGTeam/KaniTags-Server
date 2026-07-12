use anyhow::Context;
use kani_domain::collection::CollectionServiceImpl;
use kani_domain::image_management::image_management_service::ImageManagementServiceImpl;
use kani_domain::image_search::ImageSearchServiceImpl;
use kani_domain::similarity::SimilarityServiceImpl;
use kani_domain::tag::tag_service::TagServiceImpl;
use kani_domain::user::login_service::LoginServiceImpl;
use kani_domain_api_outgoing::reverse_image_search::ReverseImageSearchService;
use kani_fs::fs_image_repository::FsImageRepository;
use kani_http_api::launch_server;
use kani_lib::config::ServerConfig;
use kani_lib::log::setup_logger;
use kani_lib::worker_init;
use kani_postgres::Postgres;
use std::sync::Arc;
use tracing::{debug, info, Level};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_logger(Level::DEBUG);
    let config = ServerConfig::load_config()?;
    debug!("{:?}", config);

    let iqdb_service = worker_init::init_iqdb();
    let sauce = iqdb_service.get_sauce("Megumin".to_string()).await?;
    info!("the sauce of {} is {}", "Megumin", sauce);

    let fs_image_repository = FsImageRepository::new(config.library_path.clone());
    let fs_image_repository = Arc::new(fs_image_repository);
    let database = Postgres::new(&config.db_url, &config.db_username, &config.db_password).context("Failed to initialize database")?;
    database.setup().context("Failed to setup database")?;
    let database = Arc::new(database);

    let collection_service = CollectionServiceImpl::new(
        database.clone()
    );

    let image_management_service = ImageManagementServiceImpl::new(
        fs_image_repository.clone(),
        database.clone(),
    );
    let image_search_service = ImageSearchServiceImpl::new(
        database.clone(),
    );

    let login_service = LoginServiceImpl::new(
        database.clone(),
    );

    let similarity_service = SimilarityServiceImpl::new(
        database.clone(),
    );

    let tag_service = TagServiceImpl::new(
        database.clone(),
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
        Arc::new(collection_service),
        Arc::new(image_management_service),
        Arc::new(image_search_service),
        Arc::new(login_service),
        Arc::new(similarity_service),
        Arc::new(tag_service),
        config.request_body_limit.as_u64() as usize,
        config.server_port,
        config.auth_user_header
    )
    .await?;

    Ok(())
}

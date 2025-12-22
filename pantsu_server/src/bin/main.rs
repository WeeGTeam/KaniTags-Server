use pantsu_lib::common::result::Result;
use pantsu_lib::config::ServerConfig;
use pantsu_lib::log::setup_logger;
use pantsu_lib::routes;
use pantsu_lib::routes::AppState;
use pantsu_lib::worker::iqdb::iqdb_service::IqdbService;
use pantsu_lib::worker::worker_init;
use std::sync::Arc;
use tracing::{debug, info, Level};
use utoipa::openapi::server;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger(Level::DEBUG);
    let config = ServerConfig::load_config()?;
    println!("{:?}", config);
    debug!("{:?}", config);

    let iqdb_service = worker_init::init_iqdb();
    let sauce = iqdb_service.get_sauce("Megumin".to_string()).await?;
    info!("the sauce of {} is {}", "Megumin", sauce);

    let fs_service = worker_init::init_fs(config.clone());

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

    let app_state = AppState::new(
        Arc::new(iqdb_service),
        Arc::new(fs_service),
        config
    );

    launch_server(app_state).await?;

    Ok(())
}

pub async fn launch_server(shared_state: AppState) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", shared_state.config.server_port)).await?;

    let (router, api) = routes::get_router(&shared_state.config)
        .with_state(shared_state)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    axum::serve(listener, router).await?;

    Ok(())
}

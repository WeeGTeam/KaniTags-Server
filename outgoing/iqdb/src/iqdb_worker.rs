use super::iqdb_service::IqdbJob;
use anyhow::anyhow;
use kani_worker::worker_connection::WorkerConnectionRx;
use kani_worker::JobResponder;
use std::time::Duration;
use tokio::time::sleep;

pub async fn worker_run(connection_rx: WorkerConnectionRx<IqdbJob>, _params: ()) -> Result<(), anyhow::Error>{
    /*loop {
        connection_rx.recv_job(handle_job).await?;
    }*/
    connection_rx.recv_stream(handle_job, 4).await
}

async fn handle_job(job: IqdbJob) -> Result<(), anyhow::Error> {
    sleep(Duration::from_secs(1)).await;
    match job {
        IqdbJob::GetSauce(image, responder) => {
            let answer = handle_get_sauce(image);
            respond(responder, answer)?;
        }
    }
    Ok(())
}

fn respond<T>(responder: JobResponder<T>, response: Result<T, anyhow::Error>) -> Result<(), anyhow::Error> {
    responder.send(response)
        .map_err(|_| anyhow!("connection not working, oopsie".to_string()))
}

fn handle_get_sauce(image: String) -> Result<String, anyhow::Error> {
    if image.starts_with("Megumin") {
        let number = image.chars().into_iter().skip(7).collect::<String>();
        Ok("Bestgirl.moe".to_string() + number.as_str())
    }
    else {
        Ok("Whatever, move the board".to_string())
    }
}

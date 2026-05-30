use anyhow::{anyhow, Context};
use futures::StreamExt;
use std::future::Future;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;

pub struct WorkerConnectionTx<J> {
    request_tx: Sender<J>,
}

impl <J> WorkerConnectionTx<J> {
    pub async fn send_job(&self, job: J) -> Result<(), anyhow::Error> {
        self.request_tx.send(job).await
            .map_err(|_| anyhow!("Failed to send job to worker: channel closed"))?;
        Ok(())
    }
}

pub struct WorkerConnectionRx<J> {
    request_rx: Receiver<J>,
}

impl <J> WorkerConnectionRx<J> {
    pub async fn recv_job<F, Fut>(&mut self, job_handler: F) -> Result<(), anyhow::Error>
    where
        F: FnOnce(J) -> Fut,
        Fut: Future<Output = Result<(), anyhow::Error>>,
    {
        let job = self.request_rx.recv().await.context("receive failed: channel closed".to_string())?;
        let handler_result = job_handler(job).await;
        if let Err(e) = handler_result {
            println!("Warning: handler failed: {:?}", e) // todo: log
        };
        Ok(())
    }

    pub async fn recv_stream<F, Fut>(self, job_handler: F, num_workers: usize) -> Result<(), anyhow::Error>
    where
        F: Fn(J) -> Fut,
        Fut: Future<Output = Result<(), anyhow::Error>>,
    {
        ReceiverStream::new(self.request_rx)
            .map(|job| async {
                let handler_result = job_handler(job).await;
                if let Err(e) = handler_result {
                    println!("Warning: handler failed: {:?}", e) // todo: log
                };
            })
            .buffered(num_workers)
            .for_each(|_| async {()}).await;
        Err(anyhow!("receive failed: channel closed".to_string()))
    }
}



pub fn create_worker_connection<J>(channel_size: usize) -> (WorkerConnectionTx<J>, WorkerConnectionRx<J>) {
    let (tx, rx) = channel(channel_size);
    let connection_tx = WorkerConnectionTx {
        request_tx: tx,
    };
    let connection_rx = WorkerConnectionRx {
        request_rx: rx,
    };

    (connection_tx, connection_rx)
}

use pantsu_domain::common::result::Result;
use tokio::sync::oneshot;

pub mod worker_connection;

pub type JobResponder<T> = oneshot::Sender<Result<T>>;

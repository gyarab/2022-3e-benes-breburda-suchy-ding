use std::future::Future;

use async_std::{channel::{bounded, Sender, Receiver}, task::JoinHandle};
use std::pin::Pin;

pub type Work = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
pub type WorkQueue = Sender<Work>;

pub async fn create_task_runner() -> (WorkQueue, JoinHandle<()>) {
    let (s, r) = bounded::<Work>(64);

    (s, async_std::task::spawn(task_runner(r)))
}

async fn task_runner(recv: Receiver<Work>) {
    while let Ok(msg) = recv.recv().await {
        msg.await;
    }
}

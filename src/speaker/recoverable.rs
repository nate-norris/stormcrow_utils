use tokio::sync::watch;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

use super::trait_speaker::SpeakerT;

pub(crate) struct RecoverableRunner<T> {
    tx: watch::Sender<bool>,
    _task_handle: tokio::task::JoinHandle<()>,
    _target: Arc<T>, // stored Arc for task
}

impl<T> RecoverableRunner<T>
where
    T: SpeakerT + Send + Sync + 'static,
{
    pub(crate) fn new(target: Arc<T>) -> Self {
        let (tx, mut rx) = watch::channel(false);
        let task_target_clone = Arc::clone(&target);

        let task_handle = tokio::spawn(async move {
            loop {
                // do nothing while not in recoverable mode
                while !*rx.borrow() {
                    if rx.changed().await.is_err() {
                        return;
                    }
                }

                // call recoverable action if in recoverable mode every 2 sec
                while *rx.borrow() {
                    if let Err(e) = task_target_clone.perform_recoverable().await {
                        println!("{}", e);
                    }
                    sleep(Duration::from_secs(2)).await;
                }
            }
        });

        Self {
            tx,
            _task_handle: task_handle,
            _target: target,
        }
    }

    pub fn on(&self) {
        let _ = self.tx.send(true);
    }

    pub fn off(&self) {
        let _ = self.tx.send(false);
    }
}
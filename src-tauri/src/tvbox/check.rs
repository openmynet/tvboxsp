use std::sync::Arc;

use super::{spawn, Connection};
use tauri::{Runtime, Window};
use tokio::sync::Mutex;

#[derive(Clone, Serialize)]
pub struct ProgressPayload {
    pub progress: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionStatus<T> {
    pub connectable: bool,
    pub extra: T,
}

pub async fn check_connections<T, R>(
    window: Window<R>,
    links: Vec<T>,
    quick_mode: bool,
    skip_ipv6: Option<bool>,
) -> Vec<ConnectionStatus<T>>
where
    T: for<'se> Connection + Clone + Send + Sync + 'static,
    R: Runtime,
{
    if links.is_empty() {
        return vec![];
    }
    let skip_ipv6 = skip_ipv6.unwrap_or_default();
    let threads = {
        let tasks = links.len().max(1);
        let threads = (num_cpus::get() / 2).max(1);
        if tasks > threads * threads {
            threads
        } else {
            (tasks / threads).max(1)
        }
    };
    let mut size = links.len() / threads;
    if size == 0 {
        size = links.len();
    }

    let total = links.len() as u64;
    let count = Arc::new(Mutex::new(0));
    let chunck = links.chunks(size).map(|ch| ch.to_vec());
    let mut tasks = vec![];
    window
        .emit(
            "check_connections://progress",
            ProgressPayload { progress: 0, total },
        )
        .ok();
    for c in chunck.into_iter() {
        let w = window.clone();
        let cnt = count.clone();
        let t = spawn(async move {
            let mut items = vec![];
            for mut i in c {
                let ok = i
                    .check(quick_mode, skip_ipv6)
                    .await
                    .ok()
                    .unwrap_or_default();
                items.push(ConnectionStatus {
                    connectable: ok,
                    extra: i,
                });
                let mut c = cnt.lock().await;
                *c += 1;
                w.emit(
                    "check_connections://progress",
                    ProgressPayload {
                        progress: *c as u64,
                        total,
                    },
                )
                .ok();
            }
            items
        });
        tasks.push(t);
    }
    let mut items = vec![];
    for t in tasks {
        if let Ok(mut v) = t.await {
            items.append(&mut v);
        }
    }
    items
}

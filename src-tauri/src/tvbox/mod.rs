pub mod check;
pub mod playlist;
pub mod source;
use crate::utils;

use async_trait::async_trait;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::sync::Arc;
use tauri::{Runtime, Window};
use tokio::sync::Mutex;

#[async_trait]
pub trait Connection {
    /// 检测是否可以连通
    async fn check(&mut self, quick_mode: bool, skip_ipv6: bool) -> anyhow::Result<bool>;
}

/// 地址可访问性，
/// quick_mode开启时检测url服务器是否可连接，
/// quick_mode关闭时检测url地址是否可以访问
/// skip_ipv6：跳过ipv6可以加快检测速度
pub async fn urls_accessibility<R: Runtime>(
    window: Window<R>,
    urls: Vec<String>,
    quick_mode: bool,
    skip_ipv6: Option<bool>,
    check_m3u8: Option<bool>,
) -> Vec<String> {
    if urls.is_empty() {
        return vec![];
    }
    let skip_ipv6 = skip_ipv6.unwrap_or_default();
    let check_m3u8 = check_m3u8.unwrap_or_default();
    // 生成一个合理线程数
    let threads = {
        let tasks = urls.len();
        if tasks == 0 {
            1
        } else {
            let threads = num_cpus::get();
            if tasks > threads * threads {
                threads
            } else {
                let threads = tasks / threads;
                if threads == 0 {
                    1
                } else {
                    threads
                }
            }
        }
    };
    let mut size = urls.len() / threads;
    if size == 0 {
        size = urls.len();
    }
    let total = urls.len() as u64;
    let count = Arc::new(Mutex::new(0));
    // 每50份为一个处理单元
    let chunck = urls.chunks(size);
    // 使用多线程处理数据
    let mut tasks = vec![];
    window
        .emit(
            "urls_accessibility://progress",
            check::ProgressPayload { progress: 0, total },
        )
        .ok();
    for c in chunck {
        let c = c.to_vec();
        let w = window.clone();
        let cnt = count.clone();
        let t = spawn(async move {
            let mut items = vec![];
            for i in c {
                if url::Url::parse(&i).is_ok() {
                    if skip_ipv6 && i.contains("://[") {
                        // http://[ipv6]:port/path?query or https://[ipv6]:port/path?query
                        continue;
                    }
                    let ok = if quick_mode {
                        utils::url_connectivity(&i).await.unwrap_or_default()
                    } else {
                        if check_m3u8  {
                            utils::url_m3u8_accessibility(&i).await.unwrap_or_default()
                        }else{
                            utils::url_accessibility(&i).await.unwrap_or_default()
                        }                        
                    };
                    if ok {
                        items.push(i);
                    }
                    let mut c = cnt.lock().await;
                    *c += 1;
                    w.emit(
                        "urls_accessibility://progress",
                        check::ProgressPayload {
                            progress: *c as u64,
                            total,
                        },
                    )
                    .ok();
                }
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

/// Spawn a new tokio Task and cancel it on drop.
pub fn spawn<T>(future: T) -> Wrapper<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    Wrapper(tokio::task::spawn(future))
}

/// Cancels the wrapped tokio Task on Drop.
pub struct Wrapper<T>(tokio::task::JoinHandle<T>);

impl<T> Future for Wrapper<T> {
    type Output = Result<T, tokio::task::JoinError>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { Pin::new_unchecked(&mut self.0) }.poll(cx)
    }
}

impl<T> Drop for Wrapper<T> {
    fn drop(&mut self) {
        // do `let _ = self.0.cancel()` for `async_std::task::Task`
        self.0.abort();
    }
}

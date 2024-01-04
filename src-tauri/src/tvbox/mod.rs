pub mod check;
pub mod playlist;
pub mod source;
use crate::utils;

use async_trait::async_trait;

#[async_trait]
pub trait Connection {
    /// 检测是否可以连通
    async fn check(&mut self, quick_mode: bool, skip_ipv6: bool) -> anyhow::Result<bool>;
}

/// 地址可访问性，
/// quick_mode开启时检测url服务器是否可连接，
/// quick_mode关闭时检测url地址是否可以访问
/// skip_ipv6：跳过ipv6可以加快检测速度
pub async fn urls_accessibility(
    urls: Vec<String>,
    quick_mode: bool,
    skip_ipv6: Option<bool>,
) -> Vec<String> {
    if urls.is_empty() {
        return vec![];
    }
    let skip_ipv6 = skip_ipv6.unwrap_or_default();
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
    // 每50份为一个处理单元
    let chunck = urls.chunks(size);
    // 使用多线程处理数据
    let mut tasks = vec![];
    for c in chunck {
        let c = c.to_vec();
        let t = tokio::spawn(async move {
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
                        utils::url_accessibility(&i).await.unwrap_or_default()
                    };
                    if ok {
                        items.push(i);
                    }
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

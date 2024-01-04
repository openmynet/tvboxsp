use crate::utils;
use anyhow::Result;
use m3u8_rs::{AlternativeMedia, MediaSegment, Playlist, VariantStream};

#[derive(Debug, Default, Serialize)]
pub struct PlaylistCheckResult {
    pub loss: usize,
    pub count: usize,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistSource {
    /// 指定线程数量：默认16
    pub threads: Option<u16>,
    /// 播放列表内容
    pub content: String,
    /// 跳过IPv6的URL地址检查
    pub skip_ipv6: Option<bool>,
}
/// TODO 待实现进度功能
impl PlaylistSource {
    pub fn threads(&self, tasks: usize) -> usize {
        let tasks = tasks.max(1);
        let threads = (num_cpus::get() / 2).max(1);
        if tasks > threads * threads {
            threads
        } else {
            (tasks / threads).max(1)
        }
    }
    pub fn skip_ipv6(&self) -> bool {
        let b = self.skip_ipv6.as_ref().unwrap_or(&true);
        *b
    }

    pub async fn check(&self) -> Result<PlaylistCheckResult> {
        let content = self.content.as_bytes();
        match m3u8_rs::parse_playlist_res(&content) {
            Ok(Playlist::MasterPlaylist(mut pl)) => {
                println!("{:#?}", pl);
                let count = pl.alternatives.len();
                pl.alternatives = self.check_master_playlist(pl.alternatives).await;
                let loss = count - pl.alternatives.len();

                pl.variants = self.check_variant_stream(pl.variants).await;

                let mut content = vec![];
                pl.write_to(&mut content)?;
                Ok(PlaylistCheckResult {
                    loss,
                    count,
                    content: String::from_utf8(content)?,
                })
            }
            Ok(Playlist::MediaPlaylist(mut pl)) => {
                let count = pl.segments.len();
                pl.segments = self.check_media_playlist(pl.segments).await;
                let loss = count - pl.segments.len();
                let mut content = vec![];
                pl.write_to(&mut content)?;
                Ok(PlaylistCheckResult {
                    loss,
                    count,
                    content: String::from_utf8(content)?,
                })
            }
            Err(e) => {
                // 对错误进行简单的处理
                let err = e.map(|e| format!("Error.code: {:?}", e.code)).to_string();
                Err(anyhow!(err))
            }
        }
    }

    async fn check_master_playlist(
        &self,
        playlist: Vec<AlternativeMedia>,
    ) -> Vec<AlternativeMedia> {
        if playlist.is_empty() {
            return vec![];
        }
        // 指定线程数量
        let skip_ipv6 = self.skip_ipv6();
        let threads = self.threads(playlist.len());
        let mut size = playlist.len() / threads;
        if size == 0 {
            size = playlist.len();
        }
        // 每50份为一个处理单元
        let chunck = playlist.chunks(size);
        // 使用多线程处理数据
        let mut tasks = vec![];
        for c in chunck {
            let c = c.to_vec();
            let t = tokio::spawn(async move {
                let mut items = vec![];
                for i in c {
                    if let Some(uri) = i.uri.as_ref() {
                        if skip_ipv6 && uri.contains("://[") {
                            // http://[ipv6]:port/path?query or https://[ipv6]:port/path?query
                            continue;
                        }
                        if url::Url::parse(uri).is_ok() {
                            let ok = utils::url_accessibility(uri).await.unwrap_or_default();
                            if ok {
                                items.push(i);
                            }
                        } else {
                            // TODO 暂时默认它是Path，不做不处理
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

    async fn check_variant_stream(&self, playlist: Vec<VariantStream>) -> Vec<VariantStream> {
        if playlist.is_empty() {
            return vec![];
        }
        // 指定线程数量
        let skip_ipv6 = self.skip_ipv6();
        let threads = self.threads(playlist.len());
        let mut size = playlist.len() / threads;
        if size == 0 {
            size = playlist.len();
        }
        // 每50份为一个处理单元
        let chunck = playlist.chunks(size);
        // 使用多线程处理数据
        let mut tasks = vec![];
        for c in chunck {
            let c = c.to_vec();
            let t = tokio::spawn(async move {
                let mut items = vec![];
                for i in c {
                    if url::Url::parse(&i.uri).is_ok() {
                        if skip_ipv6 && i.uri.contains("://[") {
                            // http://[ipv6]:port/path?query or https://[ipv6]:port/path?query
                            continue;
                        }
                        let ok = utils::url_accessibility(&i.uri).await.unwrap_or_default();
                        if ok {
                            items.push(i);
                        }
                    } else {
                        // TODO 暂时默认它是Path，不做不处理
                        items.push(i);
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

    async fn check_media_playlist(&self, playlist: Vec<MediaSegment>) -> Vec<MediaSegment> {
        if playlist.is_empty() {
            return vec![];
        }
        let skip_ipv6 = self.skip_ipv6();
        // 指定线程数量
        let threads = self.threads(playlist.len());
        let mut size = playlist.len() / threads;
        if size == 0 {
            size = playlist.len();
        }
        // 每50份为一个处理单元
        let chunck = playlist.chunks(size);
        // 使用多线程处理数据
        let mut tasks = vec![];
        for c in chunck {
            let c = c.to_vec();
            let t = tokio::spawn(async move {
                let mut items = vec![];
                for i in c {
                    if url::Url::parse(&i.uri).is_ok() {
                        if skip_ipv6 && i.uri.contains("://[") {
                            // http://[ipv6]:port/path?query or https://[ipv6]:port/path?query
                            continue;
                        }
                        let ok = utils::url_accessibility(&i.uri).await.unwrap_or_default();
                        if ok {
                            items.push(i);
                        }
                    } else {
                        // TODO 暂时默认它是Path，不做不处理
                        items.push(i);
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
}

#[tokio::test]
async fn test_m3u8() {
    let file = "../data/test.m3u8";
    let content = std::fs::read_to_string(file).unwrap();
    let pl = PlaylistSource {
        threads: None,
        skip_ipv6: None,
        content,
    };
    let res = pl.check().await;
    assert!(res.is_ok());
    let res = res.unwrap();
    println!("loss: {}, count: {}", res.loss, res.count);
    std::fs::write("../data/test_checked.m3u8", &res.content).unwrap();
    assert_ne!(res.content.len(), 0);
}

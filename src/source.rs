use std::sync::Arc;

use crate::utils;
use anyhow::Result;
use indicatif::ProgressBar;
use serde_aux::prelude::*;

/// 视频源结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub sites: Vec<Vod>,
    pub lives: Vec<Live>,
    pub parses: Vec<Parse>,
    pub flags: Vec<String>,
    pub ijk: Vec<Ijk>,
    pub ads: Vec<String>,
    pub wallpaper: Option<String>,
    pub spider: Option<String>,
}

impl Source {
    pub fn parse(i: &str, illegal_comment: char) -> Result<Self> {
        // 过滤[#]

        let r = regex::Regex::new(&format!("^{}.*", illegal_comment))?;
        let i = r.replace_all(&i, "").to_string();
        let r = regex::Regex::new(&format!("\n{}.*", illegal_comment)).unwrap();
        let i = r.replace_all(&i, "").to_string();
        let doc = json5::from_str::<Self>(&i);
        if doc.is_ok() {
            // debug!("json5 解析成功!");
            return Ok(doc.unwrap());
        }
        // 过滤[/]
        let r = regex::Regex::new("^//.*")?;
        let i = r.replace_all(&i, "").to_string();
        let r = regex::Regex::new("\n//.*").unwrap();
        let i = r.replace_all(&i, "").to_string();
        let doc = serde_json::from_str::<Self>(&i)?;
        // debug!("json 解析成功!");
        Ok(doc)
    }
    pub async fn check(&mut self) -> Result<()> {
        let threads = 16;
        self.check_vod(threads).await;
        self.check_parsers(threads).await;
        self.check_lives(threads).await;
        Ok(())
    }
    /// 检查点播
    async fn check_vod(&mut self, threads: usize) {
        info!("开始检查点播源...");
        let pb = progress_bar(self.sites.len() as u64);
        let mut tasks = vec![];
        let chunk_size = self.sites.len() / threads;
        for i in self.sites.chunks(chunk_size) {
            let list = i.to_vec();
            let pb = pb.clone();
            let t = tokio::spawn(async move {
                let mut valids = vec![];
                for i in list {
                    if i.check().await.is_ok() {
                        valids.push(i.clone());
                    }
                    pb.inc(1);
                }
                valids
            });
            tasks.push(t);
        }
        let mut items = vec![];
        for t in tasks {
            if let Ok(mut v) = t.await {
                items.append(&mut v);
            }
        }
        pb.finish();
        info!(
            "原始点播源: {} / 可用点播源: {}",
            self.sites.len(),
            items.len()
        );
        self.sites = items;
    }
    /// 检查解析器
    async fn check_parsers(&mut self, threads: usize) {
        info!("开始检查解析源...");
        let pb = progress_bar(self.parses.len() as u64);
        let mut tasks = vec![];
        let chunk_size = self.parses.len() / threads;
        for i in self.parses.chunks(chunk_size) {
            let list = i.to_vec();
            let pb = pb.clone();
            let t = tokio::spawn(async move {
                let mut valids = vec![];
                for i in list {
                    if i.check().await.is_ok() {
                        valids.push(i.clone());
                    }
                    pb.inc(1);
                }
                valids
            });
            tasks.push(t);
        }
        let mut items = vec![];
        for t in tasks {
            if let Ok(mut v) = t.await {
                items.append(&mut v);
            }
        }
        pb.finish();
        info!(
            "原始解析源: {} / 可用解析源: {}",
            self.parses.len(),
            items.len()
        );
        self.parses = items;
    }
    /// 检查直播
    async fn check_lives(&mut self, threads: usize) {
        info!("开始检查直播源...");
        let mut count = 0;
        let mut urls = vec![];
        self.lives.iter().for_each(|i| {
            i.channels.iter().for_each(|a| {
                count += a.urls.len();
                let mut addrs = a.addresses();
                urls.append(&mut addrs);
            })
        });
        let mut addrs = urls.into_iter().filter(|a| a.is_domain).collect::<Vec<_>>();
        // 去除重复
        addrs.dedup_by(|a, b| a.origin == b.origin);
        let addrs = if !addrs.is_empty() {
            let pb = progress_bar(addrs.len() as u64);
            let mut tasks = vec![];
            let chunk_size = addrs.len() / threads;
            for i in addrs.chunks(chunk_size) {
                let list = i.to_vec();
                let pb = pb.clone();
                let t = tokio::spawn(async move {
                    let mut valids = vec![];
                    for i in list {
                        let res = utils::check_url(&i.origin).await.unwrap_or_default();
                        if res {
                            valids.push(i.origin);
                        }
                        pb.inc(1);
                    }
                    valids
                });
                tasks.push(t);
            }
            let mut addrs = vec![];
            for t in tasks {
                if let Ok(mut v) = t.await {
                    if !v.is_empty() {
                        addrs.append(&mut v);
                    }
                }
            }
            pb.finish();
            addrs
        } else {
            vec![]
        };
        self.lives.iter_mut().for_each(|item| {
            item.check(&addrs);
        });
        let lives = self
            .lives
            .clone()
            .into_iter()
            .filter(|i| !i.channels.is_empty())
            .collect::<Vec<_>>();
        self.lives = lives;
        let mut sum = 0;
        self.lives.iter().for_each(|i| {
            i.channels.iter().for_each(|a| {
                sum += a.urls.len();
            })
        });
        info!("原始直播源: {} / 可用直播源: {}", count, sum);
    }
    /// 转换到字符串
    pub fn to_string(&self) -> Result<String> {
        let content = serde_json::to_string_pretty(&self)?;
        Ok(content)
    }
    /// 合并另一个对象
    pub fn merge(&mut self, other: &mut Self) {
        self.sites.append(&mut other.sites);
        self.lives.append(&mut other.lives);
        self.parses.append(&mut other.parses);
        self.ads.append(&mut other.ads);
    }
    /// 去除重复
    pub fn dedup(&mut self) {
        // 点播
        self.sites.dedup_by(|a, b| a.api == b.api);
        // 解析
        self.parses.dedup_by(|a, b| a.url == b.url);
        // 广告
        self.ads.dedup_by(|a, b| a == b);
        // 直播
        self.lives.iter_mut().for_each(|item| {
            item.channels.iter_mut().for_each(|chn| {
                chn.urls.dedup();
            })
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vod {
    pub key: String,
    pub name: String,
    #[serde(rename = "type", deserialize_with = "deserialize_number_from_string")]
    pub i_type: i32,
    pub api: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub searchable: i32,
    #[serde(
        rename = "quickSearch",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    pub quick_search: i32,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub filterable: Option<i32>,
    pub ext: Option<serde_json::Value>,
}

impl Vod {
    pub async fn check(&self) -> Result<()> {
        if self.api.starts_with("http://") || self.api.starts_with("https://") {
            let ok = utils::check_url(&self.api).await?;
            if !ok {
                return Err(anyhow!("连接失败!"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Live {
    pub group: String,
    pub channels: Vec<Channel>,
}

impl Live {
    pub fn check(&mut self, origins: &Vec<String>) {
        let mut items = vec![];
        for i in &mut self.channels {
            i.check(origins);
            if !i.urls.is_empty() {
                items.push(i.clone());
            };
        }
        self.channels = items;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub urls: Vec<String>,
}
impl Channel {
    pub fn check(&mut self, origins: &Vec<String>) {
        let mut items = vec![];
        for i in &self.urls {
            let uri = UrlLike::parse(i);
            if let Ok(uri) = uri {
                if !uri.is_domain {
                    items.push(i.clone());
                } else if origins.contains(&uri.origin) {
                    items.push(i.clone());
                }
            }
        }
        self.urls = items;
    }
    pub fn addresses(&self) -> Vec<UrlLike> {
        let addrs = self
            .urls
            .clone()
            .into_iter()
            .filter_map(|uri| UrlLike::parse(&uri).ok())
            .collect::<Vec<_>>();
        addrs
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parse {
    pub name: String,
    #[serde(
        rename = "type",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    pub i_type: i32,
    pub url: String,
    pub ext: Option<serde_json::Value>,
}

impl Parse {
    pub async fn check(&self) -> Result<()> {
        if self.url.starts_with("http://") || self.url.starts_with("https://") {
            let ok = utils::check_url(&self.url).await?;
            if !ok {
                return Err(anyhow!("连接失败!"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ijk {
    pub group: String,
    pub options: Vec<Opt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opt {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub category: i32,
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Config {
    message: String,
    n: i32,
}

pub fn progress_bar(count: u64) -> Arc<ProgressBar> {
    let len = format!("{}", count).len();
    let template = format!(
        "[{{elapsed_precise}}] {{wide_bar:.white/white}} {{pos:>{}}}/{{len:{}}}",
        len, len
    );
    let pb = ProgressBar::new(count as u64);
    let style = indicatif::ProgressStyle::with_template(&template)
        .unwrap()
        .progress_chars("█░");
    pb.enable_steady_tick(std::time::Duration::from_secs(1));
    pb.set_style(style);
    Arc::new(pb)
}

#[derive(Debug, Clone)]
pub struct UrlLike {
    pub origin: String,
    pub scheme: String,
    pub is_domain: bool,
}
impl UrlLike {
    pub fn parse(s: &str) -> Result<UrlLike> {
        let uri = url::Url::parse(s)?;
        let scheme = uri.scheme();
        let host = uri.host_str().unwrap_or_default();
        let port = uri
            .port()
            .and_then(|p| Some(format!(":{}", p)))
            .unwrap_or_default();
        let addr = format!("{}://{}{}", scheme, host, port);
        Ok(Self {
            origin: addr,
            scheme: scheme.to_string(),
            is_domain: !host.contains("="),
        })
    }
}

#[test]
fn test_config() {
    let i = "proxy://do=live&type=txt&ext=aHR0cDovL3d3dy5tOTUyNy50b3AvbWwudHh0";
    // let i = "rtmp://45.88.148.178:12/channel/60c2f331961593122ebaf8c7?sign=epgg4";
    let uri = UrlLike::parse(i);
    assert!(uri.is_ok());
    let uri = uri.unwrap();
    println!("{:?}", uri);
    assert_eq!(1, 1)
}

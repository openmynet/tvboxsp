use crate::utils;
use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};

use super::{super::Connection, base_url};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Live {
    pub name: Option<String>,
    pub group: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub epg: Option<String>,
    #[serde(rename = "type")]
    pub src_type: Option<i32>,
    pub url: Option<String>,
}

impl Live {
    pub fn base(&mut self, base: &str) {
        if let Some(url) = self.url.as_mut() {
            *url = base_url(base, &url);
        }
        if let Some(chns) = self.channels.as_mut() {
            chns.iter_mut().for_each(|c| c.base(base))
        }
    }
}

#[async_trait]
impl Connection for Live {
    async fn check(&mut self, quick_mode: bool, skip_ipv6: bool) -> Result<bool> {
        let mut ok = false;
        // url
        if let Some(url) = self.url.as_ref() {
            if skip_ipv6 && url.contains("://[") {
                self.url = None;
            } else {
                let connectable = if quick_mode {
                    utils::url_connectivity(url).await.unwrap_or_default()
                } else {
                    utils::url_txt_playlist_accessibility(url).await.unwrap_or_default()
                };
                // if !connectable {
                //     self.url = None;
                // }
                ok = connectable;
            }
        }
        // channel
        if let Some(channels) = self.channels.as_mut() {
            let mut items = vec![];
            for i in channels {
                i.check(quick_mode, skip_ipv6).await.ok();
                if !i.urls.is_empty() {
                    items.push(i.clone());
                };
            }
            if !ok {
                ok = !items.is_empty();
            }
            self.channels = Some(items);
        }
        Ok(ok)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub urls: Vec<String>,
}
impl Channel {
    pub fn base(&mut self, base: &str) {
        for i in &mut self.urls {
            *i = base_url(base, i);
        }
    }
}

#[async_trait]
impl Connection for Channel {
    async fn check(&mut self, quick_mode: bool, skip_ipv6: bool) -> Result<bool> {
        let mut connectable = vec![];
        for i in &self.urls {
            if utils::is_http_url(&i) {
                if skip_ipv6 && i.contains("://[") {
                    continue;
                }
                let ok = if quick_mode {
                    utils::url_connectivity(&i).await.unwrap_or_default()
                } else {
                    utils::url_accessibility(&i).await.unwrap_or_default()
                };
                if ok {
                    connectable.push(i.to_string())
                }
            } else if i.starts_with("proxy://") {
                let proxy = i.trim_start_matches("proxy://");
                let proxy = Proxy::parse(proxy).ok();
                if let Some(proxy) = proxy.and_then(|p| p.url()) {
                    if skip_ipv6 && proxy.contains("://[") {
                        continue;
                    }
                    let ok = if quick_mode {
                        utils::url_connectivity(&proxy).await.unwrap_or_default()
                    } else {
                        utils::url_accessibility(&proxy).await.unwrap_or_default()
                    };
                    if ok {
                        connectable.push(i.to_string())
                    }
                }
            }
        }
        self.urls = connectable;
        Ok(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(rename = "do")]
    pub action: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub ext: String,
}

impl Proxy {
    pub fn parse(input: &str) -> Result<Self> {
        let doc = serde_qs::from_str(input)?;
        Ok(doc)
    }
    pub fn url(&self) -> Option<String> {
        if utils::is_http_url(&self.ext) {
            return Some(self.ext.to_string());
        }
        let decode = general_purpose::URL_SAFE
            .decode(&self.ext)
            .ok()
            .and_then(|buff| String::from_utf8(buff).ok());
        decode
    }
}

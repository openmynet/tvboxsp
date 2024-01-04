use super::{super::Connection, base_url};
use crate::utils;
use anyhow::Result;
use async_trait::async_trait;
use serde_aux::prelude::*;

// 参考： https://github.com/takagen99/Box/blob/main/app/src/main/java/com/github/tvbox/osc/bean/SourceBean.java

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vod {
    pub key: String,
    pub name: String,
    /// 0 xml 1 json 3 Spider
    #[serde(rename = "type", deserialize_with = "deserialize_number_from_string")]
    pub src_type: i32,
    // 接口地址
    pub api: String,
    /// 是否可搜索
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub searchable: i32,
    /// 是否可以快速搜索
    #[serde(
        rename = "quickSearch",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    pub quick_search: i32,
    /// 可筛选?
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub filterable: Option<i32>,
    // 站点解析Url
    #[serde(rename = "playerUrl")]
    pub player_url: Option<String>,
    /// 扩展数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
    // 自定义JAR刮削库 - 引用第三方
    pub jar: Option<String>,
    /// 播放器类型<br>
    ///  0 system 1 ikj 2 exo 10 mxplayer -1 以参数设置页面的为准
    #[serde(
        rename = "playerType",
        default = "Vod::player_type_default",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    pub player_type: Option<i32>,
    /// 分类&排序
    pub categories: Option<Vec<String>>,
    /// 需要点击播放的嗅探站点selector   ddrk.me;#id
    pub click: Option<String>,
    #[serde(
        default = "Vod::player_type_default",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    pub hide: Option<i32>,
}
impl Vod {
    pub fn player_type_default() -> Option<i32> {
        Some(-1)
    }
    pub fn base(&mut self, base: &str) {
        self.api = base_url(base, &self.api);
        if let Some(ext) = self.ext.as_mut() {
            match ext {
                serde_json::Value::String(x) => {
                    *x = base_url(base, x);
                }
                _ => {}
            }
        }
    }
}

#[async_trait]
impl Connection for Vod {
    async fn check(&mut self, quick_mode: bool, skip_ipv6: bool) -> Result<bool> {
        // 当前测试仅针对，api和ext两个字段
        if utils::is_http_url(&self.api) {
            let ok = if quick_mode {
                utils::url_connectivity(&self.api).await?
            } else {
                utils::url_accessibility(&self.api).await?
            };
            if !ok {
                return Err(anyhow!("连接失败!"));
            }
            if let Some(ext) = self.ext.as_ref() {
                match ext {
                    serde_json::Value::String(x) => {
                        // 跳过本地ip
                        if x.starts_with("http://127.0.0.1") || x.starts_with("http://localhost") {
                            return Ok(true);
                        }
                        // 测试js-spider
                        if utils::is_http_url(&x) {
                            if skip_ipv6 && x.contains("://[") {
                                return Ok(false);
                            }
                            // 检查配置文件在的点播站点是否还有效
                            // js通常会配合drpy.js一起使用
                            if x.ends_with(".js") || x.ends_with(".py") {
                                let content = reqwest::get(x).await?.text().await?;
                                let r = regex::Regex::new("https?://[0-9A-Za-z.%-]+")?;
                                let host = r
                                    .find(&content)
                                    .and_then(|m| Some(m.as_str().to_string()))
                                    .ok_or(anyhow!("找不到点播站点"))?;
                                let ok = utils::url_connectivity(&host).await.unwrap_or_default();
                                if !ok {
                                    return Err(anyhow!("点播站点无法连接!"));
                                }
                            } else {
                                // 其他地址只要能保证可以访问就行
                                let ok = if quick_mode {
                                    utils::url_connectivity(&x).await?
                                } else {
                                    utils::url_accessibility(&x).await?
                                };
                                if !ok {
                                    return Err(anyhow!("连接失败!"));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(true)
    }
}

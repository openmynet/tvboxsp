use std::sync::Arc;

use anyhow::Result;
use indicatif::ProgressBar;
pub mod ijk;
pub mod live;
pub mod parse;
pub mod rule;
pub mod vod;
use ijk::Ijk;
use live::Live;
use parse::Parse;
use rule::Rule;
use vod::Vod;
/// 视频源结构
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Source {
    pub sites: Vec<Vod>,
    pub lives: Vec<Live>,
    /// 解析地址
    pub parses: Option<Vec<Parse>>,
    /// 需要使用vip解析的flag
    pub flags: Option<Vec<String>>,
    pub ijk: Option<Vec<Ijk>>,
    pub rules: Option<Vec<Rule>>,
    pub ads: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallpaper: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spider: Option<String>,
    #[serde(rename = "warningText")]
    pub warning_text: Option<String>,
}

impl Source {
    pub fn base(&mut self, base: &str) -> Result<()> {
        self.sites.iter_mut().for_each(|item| item.base(base));
        self.lives.iter_mut().for_each(|item| item.base(base));
        if let Some(spider) = self.spider.as_mut() {
            let mut s = spider.split(";").collect::<Vec<_>>();
            let p = s.first_mut().unwrap();
            let n = base_url(base, p);
            *p = n.as_str();
            *spider = s.join(";");
        }
        Ok(())
    }
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
        let r = regex::Regex::new("\\s+//.*").unwrap();
        let i = r.replace_all(&i, "").to_string();
        let doc = serde_json::from_str::<Self>(&i).map_err(|e| {
            println!("json5.parse.error: {:?}", e);
            anyhow!("解析失败, 不是有效的 json/json5 文件.")
        })?;
        // debug!("json 解析成功!");
        Ok(doc)
    }
}

///
/// let pb = progress_bar(self.sites.len() as u64);+
/// pb.inc(1);
/// pb.finish();
///
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

fn base_url(base: &str, path: &str) -> String {
    if path.starts_with(".") || path.starts_with("/") {
        if let Ok(base) = url::Url::parse(base) {
            return base
                .join(path)
                .and_then(|new| Ok(new.to_string()))
                .unwrap_or(path.to_string());
        }
    }
    path.to_string()
}

#[tokio::test]
async fn test_d() {
    let _i = "https://jihulab.com/z-blog/xh2/-/raw/main/t.json";
    let i = "https://jihulab.com/z-blog/vip/-/raw/main/ysc/t.json";
    let content = crate::utils::read_content(i).await;
    assert!(content.is_ok());
    let content = content.unwrap();
    let src = Source::parse(&content, '#');
    println!("{:?}", src);
    assert!(src.is_ok());
    let src = src.unwrap();
    println!("live: {}", src.lives.len());
    println!("sites: {}", src.sites.len());
    println!("spider: {:?}", src.spider);
    println!("parses: {}", src.parses.is_some());
    println!("ads: {:?}", src.ads.and_then(|s| Some(s.len())));
    println!("flags: {:?}", src.flags.and_then(|s| Some(s.len())));
    println!("rules: {:?}", src.rules.and_then(|r| Some(r.len())));
    println!("wallpaper: {:?}", src.wallpaper);
    println!("warning_text: {:?}", src.warning_text);
    src.sites.iter().for_each(|i| {
        if i.key == "csp_xBPQ_奇优" {
            println!("{:?}", i)
        }
    });
    assert_eq!(1, 1)
}

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

use std::str::FromStr;

use anyhow::Result;

use clap::Parser;

mod source;
mod utils;

/// tvbox 视频源检查
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// tvbox 视频源 json 文件或者地址
    #[arg(short, long)]
    input: String,

    /// tvbox 视频源 json 文件输出名称 [可选]
    #[arg(short, long)]
    output: Option<String>,

    /// tvbox 视频源 json 文件非法注释符
    #[arg(short, long, default_value = "#")]
    comment: char,

    /// tvbox 视频源 操作模式
    #[arg(short, long, value_enum, default_value = "check")]
    mode: Mode,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
enum Mode {
    /// 检查
    Check,
    /// 合并
    Merge,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = if let Some(v) = std::env::var_os("RUST_LOG") {
        v.to_string_lossy().to_string()
    } else {
        "info".to_owned()
    };
    let log_level = format!("{},hyper=error,reqwest=error", env);
    std::env::set_var("RUST_LOG", log_level);

    env_logger::builder().format_target(false).init();
    let args = Args::parse();
    let input = args.input;
    let output = args.output;
    let (content, file_name) = get_content(&input, true).await?;

    let content = match args.mode {
        Mode::Check => check_mode(&content, args.comment).await?,
        Mode::Merge => merge_mode(&content, args.comment).await?,
    };

    let name = if let Some(name) = output {
        name
    } else {
        let name = file_name.unwrap_or("default".to_string());
        format!("{}_valid.json", name)
    };
    std::fs::write(name, &content)?;
    Ok(())
}

/// 检查模式
async fn check_mode(content: &str, comment: char) -> Result<String> {
    info!("开始解析文件...");
    let mut src = source::Source::parse(&content, comment)?;
    info!("文件解析完成!");
    src.check().await?;
    let content = src.to_string()?;
    Ok(content)
}

async fn merge_mode(content: &str, comment: char) -> Result<String> {
    info!("开始进行合并任务...");
    let lines = content
        .lines()
        .filter_map(|line| {
            let p = std::path::Path::new(line);
            if p.exists() {
                Some(line.to_string())
            } else {
                url::Url::parse(line).ok().and_then(|i| Some(i.to_string()))
            }
        })
        .collect::<Vec<_>>();
    let length = lines.len();
    let pb = source::progress_bar(length as u64);
    let threads = 16;
    let size = lines.len() / threads;
    // 每50份为一个处理单元
    let chunck = lines.chunks(size);
    // 使用多线程处理数据
    let mut tasks = vec![];
    for c in chunck {
        let c = c.to_vec();
        let pb = pb.clone();
        let t = tokio::spawn(async move {
            let mut items = vec![];
            for i in c {
                let content = get_content(&i, false).await;
                if let Ok((content, _)) = content {
                    let s = source::Source::parse(&content, comment);
                    if let Ok(s) = s {
                        items.push(s);
                    }
                }
                pb.inc(1);
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
    pb.finish();
    info!("预期合并:{}, 实际合并: {}", length, items.len());
    if items.is_empty() {
        return Err(anyhow!("获取内容为空！"));
    }
    let mut first = items[0].clone();
    items.iter_mut().enumerate().for_each(|(i, src)| {
        if i > 0 {
            first.merge(src);
        }
    });
    // 先除去重复再检查
    first.dedup();
    info!("完成合并!");
    first.check().await?;
    first.to_string()
}

/// 获取输入的文件或地址的内容
async fn get_content(i: &str, log: bool) -> Result<(String, Option<String>)> {
    let p = std::path::Path::new(&i);
    if p.exists() {
        let content = std::fs::read_to_string(p)?;
        let file_name = p
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| Some(s.to_string()));
        return Ok((content, file_name));
    }
    if i.starts_with("http://") || i.starts_with("https://") {
        if log {
            info!("开始下载: {}", i);
        }

        let file_name = url::Url::from_str(&i).ok().and_then(|i| {
            let p = std::path::Path::new(i.path());
            p.file_stem()
                .and_then(|s| s.to_str())
                .and_then(|s| Some(s.to_string()))
        });
        let client = reqwest::ClientBuilder::new()
            .pool_idle_timeout(std::time::Duration::from_secs(10))
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        let res = client.get(i).send().await?;
        let content = res.text().await?;
        if log {
            info!("文件下载完成!");
        }
        return Ok((content, file_name));
    }
    Err(anyhow!("无效文件或者地址: {}!", i))
}

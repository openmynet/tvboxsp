// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use crate::{
    tvbox::{
        self,
        check::ConnectionStatus,
        source::{live::Live, parse::Parse, vod::Vod},
    },
    utils,
};
use tauri::Result;

#[tauri::command]
pub async fn parse_playlist(
    uri: String,
    threads: Option<u16>,
    skip_ipv6: Option<bool>,
) -> Result<tvbox::playlist::PlaylistCheckResult> {
    let content = utils::read_content(&uri).await.map_err(|e| {
        println!("utils::read_content:{:?}", e);
        tauri::Error::AssetNotFound(e.to_string())
    })?;
    let source = tvbox::playlist::PlaylistSource {
        threads,
        skip_ipv6,
        content,
    };
    let res = source
        .check()
        .await
        .map_err(|e| tauri::Error::ApiNotAllowlisted(e.to_string()))?;
    Ok(res)
}

#[tauri::command]
pub async fn parse_tvbox(uri: String, base: Option<String>) -> Result<tvbox::source::Source> {
    let content = utils::read_content(&uri).await.map_err(|e| {
        println!("err:{:?}", e);
        tauri::Error::AssetNotFound(e.to_string())
    })?;
    let mut source = tvbox::source::Source::parse(&content, '#').map_err(|e| {
        println!("err:{:?}", e);
        tauri::Error::ApiNotAllowlisted(e.to_string())
    })?;
    if uri.starts_with("http://") || uri.starts_with("https://") {
        source.base(&uri).ok();
    } else if let Some(base) = base {
        source.base(&base).ok();
    }
    Ok(source)
}

#[tauri::command]
pub async fn get_content(uri: String) -> String {
    utils::read_content(&uri).await.unwrap_or_default()
}

#[tauri::command]
pub async fn urls_accessibility(
    urls: Vec<String>,
    quick_mode: Option<bool>,
    skip_ipv6: Option<bool>,
) -> Vec<String> {
    tvbox::urls_accessibility(urls, quick_mode.unwrap_or_default(), skip_ipv6).await
}

/// 执行
#[tauri::command]
pub async fn exec(args: String) -> String {
    inline_exec(args).await
}

#[tauri::command]
pub async fn vods_connectivity(
    items: Vec<Vod>,
    quick_mode: Option<bool>,
    skip_ipv6: Option<bool>,
) -> Vec<ConnectionStatus<Vod>>
where
{
    let items =
        tvbox::check::check_connections(items, quick_mode.unwrap_or_default(), skip_ipv6).await;
    items
}
#[tauri::command]
pub async fn live_connectivity(
    items: Vec<Live>,
    quick_mode: Option<bool>,
    skip_ipv6: Option<bool>,
) -> Vec<ConnectionStatus<Live>>
where
{
    let items =
        tvbox::check::check_connections(items, quick_mode.unwrap_or_default(), skip_ipv6).await;
    items
}

#[tauri::command]
pub async fn parses_connectivity(
    items: Vec<Parse>,
    quick_mode: Option<bool>,
    skip_ipv6: Option<bool>,
) -> Vec<ConnectionStatus<Parse>>
where
{
    let items =
        tvbox::check::check_connections(items, quick_mode.unwrap_or_default(), skip_ipv6).await;
    items
}

#[tauri::command]
pub async fn save(path: String, content: String) -> bool {
    std::fs::write(path, content).is_ok()
}

#[tauri::command]
pub async fn cache(key: String, value: String) {
    crate::server::updata_cache(&key, value).await;
}

#[tauri::command]
pub async fn lan_ip() -> Option<Vec<String>> {
    crate::utils::lan_ip()
}

#[tauri::command]
pub async fn is_install(application: String) -> bool {
    crate::utils::is_installed(&application)
}

pub async fn inline_exec(args: String) -> String {
    if args.is_empty() {
        return args;
    }
    let (shell, first) = if cfg!(windows) {
        ("cmd", "/c")
    } else {
        ("sh", "-c")
    };
    println!("args: {}", args);
    let child = tokio::process::Command::new(shell)
        .args([first])
        .args(&[args])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn();
    if let Ok(child) = child {
        let one_minute = std::time::Duration::from_secs(60);
        tokio::time::timeout(one_minute, child.wait_with_output())
            .await
            .ok()
            .and_then(|out| out.map_err(|e| println!("shell.error: {:?}", e)).ok())
            .and_then(|out| {
                println!("out: {:?}", out);
                String::from_utf8(out.stdout).ok()
            })
            .unwrap_or_default()
    } else {
        println!("shell.err2: {:?}", child);
        String::default()
    }
}

#[tokio::test]
async fn test_exec() {
    let args = r#"start  /d D:\"Program Files"\mpv mpv http://39.134.24.162/dbiptv.sn.chinamobile.com/PLTV/88888888/224/3221226395/1.m3u8"#;
    inline_exec(args.to_string()).await;
    assert_eq!(1, 1)
}

use anyhow::Result;
use cached::proc_macro::cached;
use reqwest::header::CONTENT_TYPE;
use std::{net::ToSocketAddrs, time::Duration};

/// 检测url的服务器网络可连接性，并不检测实际url的内容
#[cached(key = "String", result = true, convert = r#"{ format!("{}", uri) }"#)]
pub async fn url_connectivity(uri: &str) -> Result<bool> {
    let uri = url::Url::parse(&uri)?;
    let host = uri.host().ok_or(anyhow!("无效主机"))?.to_string();
    let port = uri.port().unwrap_or(80);
    let origin = format!("{}:{}", host, port);
    let addr = origin
        .to_socket_addrs()?
        .next()
        .ok_or(anyhow!("无效主机"))?;
    tokio::time::timeout(
        tokio::time::Duration::from_secs_f32(0.5),
        tokio::net::TcpStream::connect(addr),
    )
    .await??;
    Ok(true)
}

/// 检测url的可访问性
/// 超时时间被设置为1.5秒
pub async fn url_accessibility(uri: &str) -> Result<bool> {
    let ok = url_connectivity(uri).await?;
    if !ok {
        return Ok(false);
    }
    let client = reqwest::ClientBuilder::new()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0",
        )
        .connect_timeout(Duration::from_secs_f32(6.0))
        .timeout(Duration::from_secs_f32(10.0))
        .build()?;
    let resp = client.get(uri).send().await?;
    Ok(resp.status().is_success())
}

/// 检测tvbox中直播源的url的可访问性
pub async fn url_txt_playlist_accessibility(uri: &str) -> Result<bool> {
    let ok = url_connectivity(uri).await?;
    if !ok {
        return Ok(false);
    }
    let client = reqwest::ClientBuilder::new()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0",
        )
        .connect_timeout(Duration::from_secs_f32(6.0))
        .timeout(Duration::from_secs_f32(10.0))
        .build()?;
    let resp = client.get(uri).send().await?;
    if resp.status().is_success() {
        let text_plain = resp
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|contnet_type| contnet_type.to_str().ok())
            .and_then(|contnet_type| Some(contnet_type.contains("text")))
            .unwrap_or_default();
        if text_plain {
            let content = resp.text().await?;
            let checked = content
                .lines()
                .any(|line| line.contains("http://") || line.contains("https://"));
            return Ok(checked);
        }
    }
    Ok(false)
}

/// 检测m3u8直播地址url的可访问性
pub async fn url_m3u8_accessibility(uri: &str) -> Result<bool> {
    let ok = url_connectivity(uri).await?;
    if !ok {
        return Ok(false);
    }
    let client = reqwest::ClientBuilder::new()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0",
        )
        .connect_timeout(Duration::from_secs_f32(6.0))
        .timeout(Duration::from_secs_f32(10.0))
        .build()?;
    let resp = client.get(uri).send().await?;
    if resp.status().is_success() {
        let content = resp.text().await?;
        let checked = m3u8_rs::parse_playlist(content.as_bytes()).is_ok();
        Ok(checked)
    } else {
        Ok(false)
    }
}

/// ipv6下待测试
pub async fn ipv6_connectable() -> bool {
    // Ali DNS
    let url = "tcp://[2400:3200::1]:53";
    url_connectivity(url).await.unwrap_or_default()
}

pub fn is_http_url(i: &str) -> bool {
    return i.to_lowercase().starts_with("http://") || i.to_lowercase().starts_with("https://");
}

/// 读取uri中的内容
pub async fn read_content(uri: &str) -> anyhow::Result<String> {
    let href = url::Url::parse(uri)
        .ok()
        .and_then(|uri| if uri.has_host() { Some(uri) } else { None });
    if let Some(uri) = href {
        let content = reqwest::get(uri).await?.text().await?;
        Ok(content)
    } else if std::path::Path::new(&uri).exists() {
        let content = std::fs::read_to_string(&uri)?;

        Ok(content)
    } else {
        Err(anyhow!("无效资源!"))
    }
}

pub fn lan_ip() -> Option<Vec<String>> {
    default_net::get_default_interface().ok().and_then(|i| {
        Some(
            i.ipv4
                .into_iter()
                .map(|ip| ip.addr.to_string())
                .collect::<Vec<_>>(),
        )
    })
}

pub fn is_installed(app: &str) -> bool {
    if let Some(os_path) = std::env::var_os("PATH") {
        let s = if cfg!(windows) { ";" } else { ":" };
        os_path
            .into_string()
            .unwrap_or_default()
            .split(s)
            .map(|p| {
                if cfg!(windows) {
                    std::path::Path::new(p)
                        .join(format!("{}.exe", app))
                        .exists()
                } else {
                    std::path::Path::new(p).join(app).exists()
                }
            })
            .any(|exist| exist)
    } else {
        false
    }
}

#[tokio::test]
async fn test_get() {
    let x = "https://www.baidu.com/asd";
    let n = url_connectivity(x).await.unwrap_or_default();
    assert!(n)
}
#[tokio::test]
async fn test_rtsp() {
    let x = "rtsp://admin:mm4123456@192.168.0.220:554/Streaming/Channels/101";
    let n = url_connectivity(x).await.unwrap_or_default();
    assert!(n)
}

#[test]
fn test_mpv_installed() {
    let app = "mpv";
    let exist = is_installed(app);
    assert!(exist)
}

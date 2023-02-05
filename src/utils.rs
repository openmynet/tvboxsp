use anyhow::Result;
use qscan::{QSPrintMode, QScanResult, QScanTcpConnectState, QScanType, QScanner};
use std::net::ToSocketAddrs;


/// 检测url的服务器网络可连接性，并不检测实际url的内容
pub async fn check_url(uri: &str) -> Result<bool> {
    let uri = url::Url::parse(&uri)?;
    let host = uri.host().ok_or(anyhow!("无效主机"))?.to_string();
    let port = uri.port().unwrap_or(80).to_string();
    let origin = format!("{}:{}", host, port);
    let mut n = origin.to_socket_addrs()?;
    let addr = n.next().ok_or(anyhow!("无效主机"))?;
    let host = addr.ip();
    let mut scanner = QScanner::default();
    scanner.set_targets_ips(vec![host]);
    scanner.set_targets_port(&port);
    scanner.set_batch(2);
    scanner.set_timeout_ms(2000);
    scanner.set_ntries(1);
    scanner.set_scan_type(QScanType::TcpConnect);
    scanner.set_print_mode(QSPrintMode::NonRealTime);
    let qs = scanner.scan_tcp_connect().await;
    let responses = qs
        .iter()
        .filter_map(|r| {
            if let QScanResult::TcpConnect(sa) = r {
                if sa.state == QScanTcpConnectState::Open {
                    return Some(true);
                }
            }
            None
        })
        .collect::<Vec<_>>();
    Ok(!responses.is_empty())
}


#[tokio::test]
async fn test_get() {
    let x = "https://www.baidu.comx/asd";
    let n = check_url(x).await.unwrap_or_default();
    assert!(n)
}
#[tokio::test]
async fn test_rtsp() {
    let x = "rtsp://admin:mm4123456@192.168.0.220:554/Streaming/Channels/101";
    let n = check_url(x).await.unwrap_or_default();
    assert!(n)
}
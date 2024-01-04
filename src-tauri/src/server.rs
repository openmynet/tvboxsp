use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

pub struct Cache {
    pub tvbox: String,
    pub playlist: String,
}

impl Cache {
    pub fn update(&mut self, key: &str, value: String) {
        let key = key.to_lowercase();
        if key == "tvbox" {
            self.tvbox = value
        } else if key == "playlist" {
            self.playlist = value;
        }
    }
}

pub async fn updata_cache(key: &str, value: String) {
    let mut m = CACHE.lock().await;
    m.update(key, value);
}

static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
    Mutex::new(Cache {
        tvbox: String::default(),
        playlist: String::default(),
    })
});

pub async fn run() {
    let app = Router::new()
        .route("/", get(root))
        .route("/playlist.txt", get(playlist_txt))
        .route("/playlist.m3u", get(playlist_m3u))
        .route("/playlist.m3u8", get(playlist_m3u))
        .route("/tvbox.json", get(tvbox_json));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> impl IntoResponse {
    let content = r#"
    <ul>
    <a target=_blank href=/playlist.txt>playlist.txt</a><br>
    <a target=_blank href=/playlist.m3u>playlist.m3u</a><br>
    <a target=_blank href=/playlist.m3u8>playlist.m3u8</a><br>
    <a target=_blank href=/tvbox.json>tvbox.json</a>
    </ul>
    "#
    .to_string();
    let mut resp = axum::response::Response::new(content);
    resp.headers_mut()
        .insert("content-type", "text/html".parse().unwrap());
    resp
}

/// 文本格式的直播源
async fn playlist_txt() -> impl IntoResponse {
    CACHE.lock().await.playlist.clone()
}
/// m3u格式的直播源
async fn playlist_m3u() -> impl IntoResponse {
    let content = CACHE.lock().await.playlist.clone();
    let mut resp = axum::response::Response::new(content);
    resp.headers_mut()
        .insert("content-type", "application/x-mpegURL".parse().unwrap());
    resp
}
/// tvbox 配置信息
async fn tvbox_json() -> impl IntoResponse {
    let content = CACHE.lock().await.tvbox.clone();
    let mut resp = axum::response::Response::new(content);
    resp.headers_mut()
        .insert("content-type", "application/json".parse().unwrap());
    resp
}

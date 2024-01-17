// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

mod desktop;
mod server;
mod tvbox;
mod utils;
fn main() {
    std::env::set_var("RUST_LOG", "info");
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            server::run().await;
        });
        rt.shutdown_background();
    });
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            desktop::parse_playlist,
            desktop::parse_tvbox,
            desktop::get_content,
            desktop::urls_accessibility,
            desktop::exec,
            desktop::vods_connectivity,
            desktop::live_connectivity,
            desktop::parses_connectivity,
            desktop::save,
            desktop::cache,
            desktop::lan_ip,
            desktop::is_install,
            desktop::download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

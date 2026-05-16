pub mod steam_overlay;

use std::env;
use steamworks::Client;

pub fn run() {
    Client::init_app(480).unwrap();

    tauri::Builder::default()
        .plugin(steam_overlay::init())
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, _event| {});
}

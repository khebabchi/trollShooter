// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, PhysicalSize, Size};
#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    login_token: Option<String>,
}

//if let Some(proj_dirs) = ProjectDirs::from("git","SiDorios","Survive the troll") {
//    proj_dirs.config_dir();
// Lin: /home/alice/.config/barapp
// Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
// Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
//}
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            close_window,
            minimize_window,
            login,
            signin,
            init
        ])
        .manage(UserInfo { login_token: None })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn init(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&window, true).unwrap();
}

#[tauri::command]
fn close_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window.close().unwrap();
}
#[tauri::command]
fn minimize_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window.minimize().unwrap();
}

#[tauri::command]
fn login(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window
        .set_size(Size::Physical(PhysicalSize {
            width: 400,
            height: 650,
        }))
        .unwrap();
}

#[tauri::command]
fn signin(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window
        .set_size(Size::Physical(PhysicalSize {
            width: 400,
            height: 800,
        }))
        .unwrap();
}

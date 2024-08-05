// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rodio::{source::Source, Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::time::Duration;
use std::{fs::File, thread};
use tauri::{AppHandle, Manager, PhysicalSize, Size};
use window_shadows::set_shadow;

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
fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            close_window,
            minimize_window,
            login,
            signin,
            home,
            not_connected,
            play,
            show_window,
            hide_window
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn not_connected(app: AppHandle) {
    thread::spawn(move || {
        let resource_path = app
            .path_resolver()
            .resolve_resource("assets/laugh.mp3")
            .expect("failed to resolve resource");
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // -------------------------------------
        let file = BufReader::new(File::open(&resource_path).unwrap());
        let source_laugh = Decoder::new(file).unwrap().amplify(0.3);
        // --------------------------------------
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source_laugh);

        sink.sleep_until_end();
    });
}
#[tauri::command]
fn close_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window.close().unwrap();
}
#[tauri::command]
fn hide_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window.hide().unwrap();
}
#[tauri::command]
fn show_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
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

#[tauri::command]
fn play(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window
        .set_size(Size::Physical(PhysicalSize {
            width: 793,
            height: 793,
        }))
        .unwrap();
    window.set_decorations(true).unwrap();
}

#[tauri::command]
fn home(app: AppHandle) {
    let resource_path = app
        .path_resolver()
        .resolve_resource("assets/bg.mp3")
        .expect("failed to resolve resource");
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // --------------------------------------

        let file = BufReader::new(File::open(&resource_path).unwrap());
        let source_bg = Decoder::new(file).unwrap().amplify(0.2);
        // --------------------------------------
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source_bg.repeat_infinite());

        sink.sleep_until_end();
    });
    let window = app.get_window("main").unwrap();
    window
        .set_size(Size::Physical(PhysicalSize {
            width: 700,
            height: 850,
        }))
        .unwrap();
}

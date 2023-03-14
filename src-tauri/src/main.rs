#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{Device, Folder, RemarkableClient, ScanEntry};
use tauri::async_runtime::Mutex;

// #[tauri::command]
// fn open_explorer() {
//     println!("asdf");
//     let child = std::process::Command::new("dbus-send")
//         .args([
//             "--session",
//             "--dest=org.freedesktop.FileManager1",
//             "--type=method_call",
//             "/org/freedesktop/FileManager1 org.freedesktop.FileManager1.ShowItems",
//             "array:string:\"file:///path/to/file\"",
//             " string:\"\"",
//         ])
//         .spawn()
//         .expect("Failed to execute command");
// }
//
#[tauri::command]
async fn scan(state: tauri::State<'_, Mutex<RemarkableClient>>) -> Result<Vec<ScanEntry>, String> {
    let client = state.lock().await;
    return client.scan().await.map_err(|e| e.to_string());
}

#[tauri::command]
async fn add_device(
    mac: String,
    device: Device,
    state: tauri::State<'_, Mutex<RemarkableClient>>,
) -> Result<(), String> {
    let mut client = state.lock().await;
    return client.add_device(mac, device).map_err(|e| e.to_string());
}

#[tauri::command]
async fn fs(
    mac: String,
    state: tauri::State<'_, Mutex<RemarkableClient>>,
) -> Result<Folder, String> {
    let mut client = state.lock().await;

    client
        .connect(mac.to_owned())
        .await
        .map_err(|e| e.to_string())?;
    let fs = client.fs(&mac).await.map_err(|e| e.to_string())?;

    todo!()
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(RemarkableClient::new()))
        .invoke_handler(tauri::generate_handler![scan, add_device, fs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

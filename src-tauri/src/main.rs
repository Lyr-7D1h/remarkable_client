#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{Device, RemarkableClient, ScanEntry};
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
) -> Result<Vec<String>, String> {
    let client = state.lock().await;

    return Err("asdf".into());
    // scan_network();
    // let mut visited = HashSet::new();
    // return Ok(vec!["asdf".to_owned(), "asdf".to_owned()]);
    // read_dir(path)
    //     .unwrap()
    //     .into_iter()
    //     .filter_map(|entry| {
    //         let entry = entry.unwrap();
    //         if let Some(file_stem) = entry.path().file_stem() {
    //             if visited.contains(&file_stem.to_owned()) {
    //                 return None;
    //             }
    //         }
    //         if let Some(ext) = entry.path().extension() {
    //             if ext == "metadata" {
    //                 if let Ok(_metadata) =
    //                     serde_json::from_str::<Metadata>(&read_to_string(entry.path()).unwrap())
    //                 {
    //                     let path = entry.path();
    //                     let file_stem = path
    //                         .file_stem()
    //                         .expect(&format!("Could not find filename for {:?}", entry.path()));
    //                     visited.insert(file_stem.to_owned());
    //                 }
    //             }
    //         }
    //
    //         Some(entry)
    //     })
    //     .filter_map(|entry| {
    //         let file_type = entry.file_type().unwrap();
    //         if file_type.is_dir() {
    //             Some(
    //                 button(text(format!(
    //                     "{}",
    //                     entry.file_name().to_string_lossy().to_string()
    //                 )))
    //                 .width(300)
    //                 .on_press(Message::Open(entry.path())),
    //             )
    //         } else {
    //             None
    //         }
    //     })
    //     .into()
}

// struct RemarkableClientState(Mutex<RemarkableClient>);

fn main() {
    tauri::Builder::default()
        // .manage(RemarkableClientState(Mutex::new(RemarkableClient::new())))
        .manage(Mutex::new(RemarkableClient::new()))
        .invoke_handler(tauri::generate_handler![scan, add_device, fs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

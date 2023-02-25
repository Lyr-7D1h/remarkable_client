#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashSet,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

mod remarkable;

fn find_ip() {}

#[tauri::command]
fn open_explorer() {
    println!("asdf");
    let child = std::process::Command::new("dbus-send")
        .args([
            "--session",
            "--dest=org.freedesktop.FileManager1",
            "--type=method_call",
            "/org/freedesktop/FileManager1 org.freedesktop.FileManager1.ShowItems",
            "array:string:\"file:///path/to/file\"",
            " string:\"\"",
        ])
        .spawn()
        .expect("Failed to execute command");
}

#[tauri::command]
fn files(path: PathBuf) -> Vec<String> {
    // let mut visited = HashSet::new();
    return vec!["asdf".to_owned(), "asdf".to_owned()];
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_explorer, files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
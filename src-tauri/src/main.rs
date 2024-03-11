// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arboard::Clipboard;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn filenames() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let lines = clipboard.get_text().unwrap();

    let filenames = lines
        .split("\n")
        .map(extract_filename_from_path)
        .collect::<Vec<&str>>()
        .join("\n");

    clipboard.set_text(&filenames).unwrap();

    filenames
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![filenames])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn extract_filename_from_path(line: &str) -> &str {
    match line.rfind("/") {
        Some(last_slash) => {
            let (_, filename) = line.split_at(last_slash + 1);
            filename
        }
        None => line,
    }
}

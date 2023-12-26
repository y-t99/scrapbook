// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bson::Document;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![document_change])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn document_change(events: Vec<Document>) {
    events.iter().for_each(|event| {
        println!("{}", event);
    });
}
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Seek;
use bson::Document;

struct SState(Vec<u64>);

fn main() {
    let hash: Vec<u64> = Vec::new();
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(SState(hash))
        .invoke_handler(tauri::generate_handler![document_change, history])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn document_change(_state: tauri::State<SState>, events: Vec<Document>) {
    let buffer = File::options().append(true).create(true).open("./../log.bson").expect("Log Open Error.");
    for event in events.iter() {
        event.to_writer(&buffer).expect("Log Append Error.");
        println!("{}", event);
    }
}

#[tauri::command]
fn history() -> Vec<Document> {
    let buffer = File::options().read(true).open("./../log.bson").expect("Log Open Error.");
    let meta= buffer.metadata().expect("Get Metadata Error.");
    let len = meta.len();
    let mut history: Vec<Document> = Vec::new();
    while (&buffer).stream_position().expect("Get Seek Error.") < len {
        let document = Document::from_reader(&buffer).expect("Document Read Error.");
        history.push(document);
    }
    history
}

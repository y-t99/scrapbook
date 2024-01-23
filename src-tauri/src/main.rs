// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::convert::TryInto;
use std::fmt;
use std::fs::File;
use std::io::Seek;
use std::mem::replace;
use std::sync::Mutex;
use bson::Document;
use tauri::State;

// https://stackoverflow.com/questions/33759072/why-doesnt-vect-implement-the-display-trait
struct SliceDisplay<'a, T: 'a>(&'a [T]);

impl<'a, T: fmt::Display + 'a> fmt::Display for SliceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for item in self.0 {
            if !first {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}

struct Indexes {
    table: Mutex<Vec<u64>>
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(Indexes {
            table: Mutex::new(Vec::new())
        })
        .invoke_handler(tauri::generate_handler![document_change, history])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn document_change(indexes: State<Indexes>, events: Vec<Document>) {
    let mut table = indexes.table.lock().unwrap();
    if table.capacity() < 1 {
        table.reserve(256);
    }
    let mut buffer = File::options().append(true).create(true).open("./../log.bson").expect("Log Open Error.");
    for event in events.iter() {
        println!("{}", event);
        let start_point = buffer.metadata().unwrap().len();
        let action_type = event.get("action_type").unwrap().as_str().expect("Type Transformed Error.");
        match action_type {
            "block-added" => {
                let index: usize = event.get("index").unwrap().as_i32().unwrap().try_into().unwrap();
                table.insert(index, start_point);
            },
            "block-removed" => {
                let index: usize = event.get("index").unwrap().as_i64().unwrap().try_into().unwrap();
                table.remove(index);
            },
            "block-moved" => {
                let from_index: usize = event.get("fromIndex").unwrap().as_i64().unwrap().try_into().unwrap();
                let to_index: usize = event.get("toIndex").unwrap().as_i64().unwrap().try_into().unwrap();
                let position = table.remove(from_index);
                table.insert(to_index, position);
            },
            "block-changed" => {
                let index: usize = event.get("index").unwrap().as_i32().unwrap().try_into().unwrap();
                let _ = replace(&mut table[index], start_point);
            },
            _ => panic!("Action Error.") }
        event.to_writer(&buffer).expect("Log Append Error.");
        println!("{}", SliceDisplay(& table));
    }
}

#[tauri::command]
fn history() -> Vec<Document> {
    let buffer = File::options().read(true).open("./../log.bson").expect("Log Open Error.");
    let meta = buffer.metadata().expect("Get Metadata Error.");
    let len = meta.len();
    let mut history: Vec<Document> = Vec::new();
    while (&buffer).stream_position().expect("Get Seek Error.") < len {
        let document = Document::from_reader(&buffer).expect("Document Read Error.");
        history.push(document);
    }
    history
}

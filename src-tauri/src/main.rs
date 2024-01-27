// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bson::Document;
use std::convert::TryInto;
use std::fmt;
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::mem::replace;
use std::sync::Mutex;
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
    table: Mutex<Vec<u64>>,
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(Indexes {
            table: Mutex::new(Vec::new()),
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
    // list action: 0 - remove 1 - insert 2 - replace
    const LIST_REMOVE_ACTION: [u8; 1] = [0];
    const LIST_INSERT_ACTION: [u8; 1] = [1];
    const LIST_REPLACE_ACTION: [u8; 1] = [2];
    let mut index_log = File::options()
        .append(true)
        .create(true)
        .open("./../index-log.data")
        .expect("Log Open Error.");
    let mut data_log = File::options()
        .append(true)
        .create(true)
        .open("./../log.bson")
        .expect("Log Open Error.");
    for event in events.iter() {
        let start_point = data_log.metadata().unwrap().len();
        let action_type = event
            .get("action_type")
            .unwrap()
            .as_str()
            .expect("Type Transformed Error.");
        match action_type {
            "block-added" => {
                let index: usize = event
                    .get("index")
                    .unwrap()
                    .as_i32()
                    .unwrap()
                    .try_into()
                    .unwrap();
                table.insert(index, start_point);
                index_log
                    .write(&LIST_INSERT_ACTION)
                    .expect("Log Write Error.");
                let index_position = (index as u64).to_be_bytes();
                index_log.write(&index_position).expect("Log Write Error.");
                index_log
                    .write(&start_point.to_be_bytes())
                    .expect("Log Write Error.");
            }
            "block-removed" => {
                let index: usize = event
                    .get("index")
                    .unwrap()
                    .as_i32()
                    .unwrap()
                    .try_into()
                    .unwrap();
                table.remove(index);
                index_log
                    .write(&LIST_REMOVE_ACTION)
                    .expect("Log Write Error.");
                let index_position = (index as u64).to_be_bytes();
                index_log.write(&index_position).expect("Log Write Error.");
            }
            "block-moved" => {
                let from_index: usize = event
                    .get("from_index")
                    .unwrap()
                    .as_i32()
                    .unwrap()
                    .try_into()
                    .unwrap();
                let to_index: usize = event
                    .get("to_index")
                    .unwrap()
                    .as_i32()
                    .unwrap()
                    .try_into()
                    .unwrap();
                let position = table.remove(from_index);
                index_log
                    .write(&LIST_REMOVE_ACTION)
                    .expect("Log Write Error.");
                let index_position = (from_index as u64).to_be_bytes();
                index_log.write(&index_position).expect("Log Write Error.");
                table.insert(to_index, position);
                index_log
                    .write(&LIST_INSERT_ACTION)
                    .expect("Log Write Error.");
                let index_position = (to_index as u64).to_be_bytes();
                index_log.write(&index_position).expect("Log Write Error.");
                index_log
                    .write(&position.to_be_bytes())
                    .expect("Log Write Error.");
            }
            "block-changed" => {
                let index: usize = event
                    .get("index")
                    .unwrap()
                    .as_i32()
                    .unwrap()
                    .try_into()
                    .unwrap();
                let _ = replace(&mut table[index], start_point);
                index_log
                    .write(&LIST_REPLACE_ACTION)
                    .expect("Log Write Error.");
                let index_position = (index as u64).to_be_bytes();
                index_log.write(&index_position).expect("Log Write Error.");
                index_log
                    .write(&start_point.to_be_bytes())
                    .expect("Log Write Error.");
            }
            _ => panic!("Action Error."),
        }
        event.to_writer(&data_log).expect("Log Append Error.");
    }
}

#[tauri::command]
fn history(indexes: State<Indexes>) -> Vec<Document> {
    // Read index log to generate index;
    let mut table = indexes.table.lock().unwrap();
    let mut index_log = match File::options().read(true).open("./../index-log.data") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                File::create("./../index-log.data").expect("Log Create Error.");
                File::create("./../log.bson").expect("Log Create Error.");
                File::options().read(true).open("./../index-log.data").expect("Log Open Error.")
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    let index_log_meta = index_log.metadata().expect("Get Metadata Error.");
    let index_log_len = index_log_meta.len();
    let mut list_action_buffer: [u8; 1] = [0; 1];
    let mut u64_buffer: [u8; 8] = [0; 8];
    while (&index_log).stream_position().expect("Get Seek Error.") < index_log_len {
        index_log.read(&mut list_action_buffer).expect("Index Read Error.");
        let list_action = u8::from_be_bytes(list_action_buffer.clone());
        match list_action {
            0 => {
                index_log.read(&mut u64_buffer).expect("Index Read Error.");
                let index = u64::from_be_bytes(u64_buffer.clone());
                table.remove(index.try_into().unwrap());
            },
            1 => {
                index_log.read(&mut u64_buffer).expect("Index Read Error.");
                let index = u64::from_be_bytes(u64_buffer.clone());
                index_log.read(&mut u64_buffer).expect("Index Read Error.");
                let value = u64::from_be_bytes(u64_buffer.clone());
                table.insert(index.try_into().unwrap(), value);
            },
            2 => {
                index_log.read(&mut u64_buffer).expect("Index Read Error.");
                let index: usize = u64::from_be_bytes(u64_buffer.clone()).try_into().unwrap();
                index_log.read(&mut u64_buffer).expect("Index Read Error.");
                let value = u64::from_be_bytes(u64_buffer.clone());
                let _ = replace(&mut table[index], value);
            },
            _ => {
                panic!("Index Log Error.");
            }
        }
    }
    let mut buffer = File::options()
        .read(true)
        .open("./../log.bson")
        .expect("Log Open Error.");
    let mut history: Vec<Document> = Vec::new();
    for index in table.iter() {
        buffer.seek(SeekFrom::Start(index.clone())).expect("Document Seek Error.");
        let document = Document::from_reader(&buffer).expect("Document Read Error.");
        history.push(document);
    }
    history
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::any::Any;
use std::os::windows::fs::MetadataExt;

use chrono::{DateTime, Utc};
use serde::Serialize;
use tauri::Manager;
use window_shadows::set_shadow;

use dto::FileItem;

mod dto;

#[tauri::command]
fn get_file_list(dir: &str) -> Vec<FileItem> {
    let paths = std::fs::read_dir(dir).unwrap();
    let mut list: Vec<FileItem>  = Vec::new();
    for item in paths {
        let item = item.unwrap();
        let metadata = item.metadata().unwrap();
        let dt: DateTime<Utc> = metadata.created().unwrap().into();
        let mut file_info:FileItem = FileItem{
            name: item.file_name().into_string().unwrap(),
            size: metadata.len(),
            file_type: if metadata.is_dir() {String::from("dir")} else {String::from("file")},
            edit_date: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            hidden: metadata.file_attributes() & 0x00000002 != 0
        };
        println!("{:?} {:?} {:?}",file_info.name, file_info.size, file_info.file_type);
        list.insert(0,file_info);
    }
    return list;
}

fn main() { 
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&app.get_window("main").unwrap(),true).unwrap();
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_file_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::any::Any;
use std::os::windows::fs::MetadataExt;
use base64::Engine;

use serde::{Deserialize, Serialize};
use tauri::{Icon, Manager};
use window_shadows::set_shadow;

use api::getfilelist;
use fileaccess::icon::get_icon;
use crate::api::getfilelist::get_file_list;
use crate::api::openfile::open_file;

mod dto;
mod fileaccess;
mod api;

// #[derive(Deserialize)]
// struct IconReq {
//     name: String,
//     request_type: String,
// }

// #[tauri::command]
// fn get_icons(iconReqList:Vec<IconReq>) -> Result<Vec<FileItem>,String>{
//     
// }

fn main() { 
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&app.get_window("main").unwrap(),true).unwrap();
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_file_list,
            open_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

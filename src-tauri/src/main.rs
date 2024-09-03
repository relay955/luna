// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::Engine;
use std::any::Any;
use std::os::windows::fs::MetadataExt;
use std::sync::{Mutex, RwLock};

use crate::api::addfavoritefolder::add_favorite_folder;
use crate::api::getdrivelist::get_drive_list;
use crate::api::getfavoritefolder::get_favorite_folders;
use crate::api::getfilelist::get_file_list;
use crate::api::geticons::get_icons;
use crate::api::openfile::open_file;
use crate::api::protection_api::{encrypt_file, enter_protection_mode, exit_protection_mode, is_in_protection_mode};
use crate::api::searchfiles::search_files;
use crate::db::create_env;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use window_shadows::set_shadow;
use crate::api::luna_settings_api::{get_luna_settings, update_luna_settings};

mod fileaccess;
mod api;
mod db;
mod indexer;
mod jobscheduler;
mod module;

#[derive(Serialize, Deserialize)]
pub struct Protection {
    pub key: Option<String>,
}

fn main() {
    let path = "./data";
    tauri::Builder::default()
        .manage(create_env())
        .manage(RwLock::new(Protection {key:None}))
        .setup(|app| {
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&app.get_window("main").unwrap(),true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_drive_list,
            get_favorite_folders,
            add_favorite_folder,
            get_file_list,
            get_icons,
            open_file,
            search_files,
            enter_protection_mode,
            exit_protection_mode,
            is_in_protection_mode,
            encrypt_file,
            get_luna_settings,
            update_luna_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

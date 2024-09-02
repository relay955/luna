use std::collections::HashMap;
use std::os::windows::fs::MetadataExt;
use std::ptr;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use serde_json::from_str;
use tauri::State;
use windows::Win32::Foundation::CRYPT_E_OBJECT_LOCATOR_OBJECT_NOT_FOUND;
use windows::Win32::UI::Shell::ReturnOnlyIfCached;
use crate::api::ApiError;
use crate::fileaccess::file;
use crate::fileaccess::file::FileItem;
use crate::GlobalData;
use crate::module::crypto::decrypt_binary_with_iv;
use crate::module::enc_metadata::{key_to_enc_metadata_signature, EncMetadata};

#[tauri::command]
pub fn get_file_list(
    global_data: State<Mutex<GlobalData>>,
    dir: &str, sort_by:&str, sort_direction:&str,
     grouping_mode:&str, search:&str, filter:&str) -> Result<Vec<FileItem>,String> {
    let mut list = match file::get_file_list(dir) {
        Ok(p) => p,
        Err(_) => return Err("Failed to read directory".to_string())
    };

    //parse encrypted metadata
    let global_data = global_data.lock().unwrap();
    let encryption_key = global_data.encryption_key.clone();
    drop(global_data);
    
    if encryption_key.is_some() {
        decrypt_metadata(encryption_key.unwrap().as_str(), dir, &mut list);
    }

    //sort and filtering
    if sort_by == "name" {
        list.sort_by(|a, b| a.name.cmp(&b.name));
    } else if sort_by == "size" {
        list.sort_by(|a, b| a.size.cmp(&b.size));
    } else if sort_by == "edit_date" {
        list.sort_by(|a, b| a.edit_date.cmp(&b.edit_date));
    }

    if sort_direction == "desc" { list.reverse(); }

    if grouping_mode == "folder"{
        let mut dirs:Vec<FileItem> = list.clone().into_iter().filter(|x| x.file_type == "dir").collect();
        let mut files = list.into_iter().filter(|x| x.file_type == "file").collect();
        dirs.append(&mut files);
        list = dirs;
    }

    if !filter.is_empty() {
        list.retain(|x| x.name.contains(filter));
    }
    
    Ok(list)
}

fn decrypt_metadata(encryption_key:&str, dir:&str, file_list:&mut Vec<FileItem>) -> Result<(),ApiError>{
    let enc_metadata_list = EncMetadata::open(dir, encryption_key)?;
    
    for item in file_list.iter_mut() {
        let key = item.file_type.clone() + "||" + item.name.as_str();
        if !enc_metadata_list.contains_key(&key) { continue; }
        let enc_metadata = match enc_metadata_list.get(&key) {
            Some(m) => m,
            None => continue
        };
        
        item.decrypted_name = Some(enc_metadata.real_name.clone());
    }
    
    Ok(())
}
use std::collections::HashMap;
use std::os::windows::fs::MetadataExt;
use std::ptr;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use serde_json::from_str;
use tauri::State;
use windows::Win32::Foundation::CRYPT_E_OBJECT_LOCATOR_OBJECT_NOT_FOUND;
use windows::Win32::UI::Shell::ReturnOnlyIfCached;
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
        decrypt_metadata(encryption_key.unwrap().as_str(), &mut list);
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

fn decrypt_metadata(encryption_key:&str, file_list:&mut Vec<FileItem>){
    let mut key = encryption_key.to_string();
    let signature = key_to_enc_metadata_signature(&key);
    let metadata_file = file_list.iter().find(|x| x.name.starts_with(&signature) && x.name.len() > 35);
    if metadata_file.is_none() { return }
    let metadata_file = metadata_file.unwrap();

    
    //비밀번호 복사한 부분 zerofill
    unsafe{
        let key_bytes = key.as_mut_vec();
        ptr::write_bytes(key_bytes.as_mut_ptr(),0,key_bytes.len());
    }
}
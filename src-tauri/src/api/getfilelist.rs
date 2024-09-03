use crate::api::{ApiError, ValidationError};
use crate::fileaccess::file;
use crate::fileaccess::file::FileItem;
use crate::module::enc_metadata::{EncMetadata};
use crate::Protection;
use std::sync::{Mutex, RwLock};
use tauri::State;

#[tauri::command]
pub fn get_file_list(
    protection: State<RwLock<Protection>>,
    dir: &str, sort_by:&str, sort_direction:&str,
    grouping_mode:&str, search:&str, filter:&str) -> Result<Vec<FileItem>,ApiError> {
    let mut list = match file::get_file_list(dir) {
        Ok(p) => p,
        Err(_) => return Err(ValidationError::ParseFailed.into())
    };

    //parse encrypted metadata
    {
        let protection = protection.read().unwrap();
        let key = protection.key.as_ref();

        if key.is_some() {
            decrypt_metadata(key.unwrap(), dir, &mut list)?;
        }
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
    let enc_metadata = EncMetadata::open(dir, encryption_key)?;
    
    for item in file_list.iter_mut() {
        if enc_metadata.is_target_metadata(&item.full_path) { 
            item.decrypted_name = Some("ENC_METADATA".to_string());
            continue;
        }
        let enc_file_item = match enc_metadata.get_enc_file_item(&item.full_path){
            Some(m) => m,
            None => continue
        };
        item.decrypted_name = Some(enc_file_item.real_name.clone());
    }
    
    Ok(())
}
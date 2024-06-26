use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Utc};
use crate::fileaccess::file;
use crate::fileaccess::file::FileItem;

#[tauri::command]
pub fn get_file_list(dir: &str, sort_by:&str, sort_direction:&str,
                 grouping_mode:&str, search:&str, filter:&str) -> Result<Vec<FileItem>,String> {
    let mut list = match file::get_file_list(dir) {
        Ok(p) => p,
        Err(_) => return Err("Failed to read directory".to_string())
    };
    
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

    // if list.len() < 500 { Ok(list) }
    // else { Ok(list.splice(0..500, Vec::new()).collect()) }
}

use crate::db::favoritefolder;
use crate::fileaccess::{drive, file};
use crate::fileaccess::file::FileItem;

#[tauri::command]
pub fn get_favorite_folders() -> Vec<FileItem> {
    let favorite_folders = favoritefolder::get_favorite_folders().unwrap();
    
    let mut list: Vec<FileItem> = Vec::new();
    for item in favorite_folders {
        let file_info = match  file::get_file_info(item.as_str()){
            Ok(f) => f,
            Err(_) => continue
        };
        list.push(file_info);
    }
    list
}
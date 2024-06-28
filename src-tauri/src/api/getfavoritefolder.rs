use crate::db::favoritefolder;
use crate::fileaccess::{drive, file};
use crate::fileaccess::file::FileItem;

#[tauri::command]
pub fn get_favorite_folders() -> Vec<FileItem> {
    let favorite_folders = favoritefolder::get_favorite_folders();
    let favorite_folders = match favorite_folders.as_array(){
        Some(f) => f,
        None => return Vec::new()
    };
    let mut list: Vec<FileItem> = Vec::new();
    for item in favorite_folders {
        let item = match item.as_str(){
            Some(i) => i,
            None => continue
        };
        let file_info = match  file::get_file_info(item){
            Ok(f) => f,
            Err(_) => continue
        };
        list.push(file_info);
    }
    list
}
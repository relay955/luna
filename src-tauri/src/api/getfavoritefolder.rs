use heed::Env;
use tauri::{ State};
use crate::db::favorite_folder_accessor::FavoriteFolderAccessor;
use crate::fileaccess::{drive, file};
use crate::fileaccess::file::FileItem;

#[tauri::command]
pub fn get_favorite_folders(db: State<Env>) -> Vec<FileItem> {
    let favorite_folders = FavoriteFolderAccessor::new(&db).get().unwrap();
    
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
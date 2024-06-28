use crate::db;
use crate::db::favoritefolder;

#[tauri::command]
pub fn add_favorite_folder(full_path:&str){
    favoritefolder::add_favorite_folder(full_path);
    db::persist_db();
}
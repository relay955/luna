use crate::db::favorite_folder_accessor::FavoriteFolderAccessor;
use heed::Env;
use tauri::State;

#[tauri::command]
pub fn add_favorite_folder(db:State<Env>, full_path:&str){
    FavoriteFolderAccessor::new(&db).add(full_path);
}
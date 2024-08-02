use heed::Env;
use tauri::State;
use crate::db::favorite_folder_accessor::FavoriteFolderAccessor;

#[tauri::command]
pub fn add_favorite_folder(db:State<Env>, full_path:&str){
    FavoriteFolderAccessor::new(&db).add(full_path);
}
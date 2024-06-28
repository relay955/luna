use serde_json::Value;
use crate::db::DB;

pub fn get_favorite_folders() -> Value{
    let db = match DB.lock() {
        Ok(d) => d,
        Err(_) => return Value::Object(serde_json::Map::new())
    };
    let db = match db.as_object(){
        Some(d) => d,
        None => return Value::Object(serde_json::Map::new())
    };
    match db.get("favorite_folders"){
        Some(f) => f.clone(),
        None => Value::Object(serde_json::Map::new())
    }
}

pub fn add_favorite_folder(folder_full_path:&str) -> bool {
    let mut db = match DB.lock() {
        Ok(d) => d,
        Err(_) => return false
    };
    let mut db = match db.as_object_mut(){
        Some(d) => d,
        None => return false
    };
    let favorite_folders = match db.get_mut("favorite_folders"){
        Some(f) => f,
        None => {
            db.insert("favorite_folders".to_string(), Value::Array(Vec::new()));
            db.get_mut("favorite_folders").unwrap()
        }
    };
    let favorite_folders = match favorite_folders.as_array_mut(){
        Some(f) => f,
        None => return false
    };
    favorite_folders.push(Value::String(folder_full_path.to_string()));
    true
}
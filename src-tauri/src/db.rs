pub mod favoritefolder;

use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::Value;

lazy_static!{
    pub static ref DB:Mutex<Value> = Mutex::new(
        serde_json::from_str("{}")
        .unwrap());
}

pub fn persist_db(){
    let db = DB.lock().unwrap();
    let db = db.to_string();
    std::fs::write("luna.json", db).unwrap();
}

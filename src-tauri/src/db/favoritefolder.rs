use std::path::Path;
use std::sync::Mutex;
use heed::{Database, RwTxn};
use heed::types::Str;
use serde::de::Error;
use crate::db::ENV;

pub fn get_favorite_folders() -> Result<Vec<String>,Box<dyn std::error::Error>>{
    let mut rtxn = ENV.read_txn()?;
    let db:Option<Database<Str,Str>> = ENV.open_database(&mut rtxn, Option::from("favorite_folder"))?;
    let db = match db{
        Some(d) => d,
        None => return Ok(Vec::new())
    };
    
    let mut favorite_folders = Vec::new();
    let iter = db.iter(&rtxn)?;
    
    for result in iter{
        let (key, value) = result.unwrap();
        favorite_folders.push(key.to_string())
    }
    Ok(favorite_folders)
}

pub fn add_favorite_folder(folder_full_path:&str) -> Result<(),Box<dyn std::error::Error>> {
    let mut wtxn = ENV.write_txn()?;
    let db:Database<Str,Str> = ENV.create_database(&mut wtxn,Some("favorite_folder"))?;
    db.put(&mut wtxn, folder_full_path, "")?;
    wtxn.commit()?;
    Ok(())
}
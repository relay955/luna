use std::collections::HashMap;
use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Clone,Serialize)]
pub struct FileItem {
    pub name: String,
    pub size: u64,
    pub file_type: String,
    pub edit_date: String,
    pub hidden: bool,
    pub full_path: String,
    
    pub decrypted_name :Option<String>,
}

impl FileItem {
    pub fn default() -> FileItem {
        FileItem {
            name: String::new(),
            size: 0,
            file_type: String::new(),
            edit_date: String::new(),
            hidden: false,
            full_path: String::new(),
            decrypted_name: None
        }
    }
}

/// 특정 디렉터리 내에 존재하는 파일 목록을 가져옵니다.
pub fn get_file_list(dir:&str) -> Result<Vec<FileItem>,()> {
    let paths = match std::fs::read_dir(dir) {
        Ok(p) => p,
        Err(e) => return Err(())
    };
    let mut list: Vec<FileItem>  = Vec::new();
    for item in paths {
        let item = match item {Ok(i) => i, Err(_) => continue};
        let metadata = match item.metadata() { Ok(m) => m, Err(_) => continue };

        list.insert(0,metadata_into_fileitem(&metadata, item.path().to_str().unwrap()));
    }
    
    Ok(list)
}

pub fn get_file_info(full_path:&str) -> Result<FileItem,()> {
    let metadata = match std::fs::metadata(full_path) {
        Ok(m) => m,
        Err(_) => return Err(())
    };

    return Ok(metadata_into_fileitem(&metadata, full_path));
}

pub fn metadata_into_fileitem(metadata: &std::fs::Metadata, full_path:&str) -> FileItem {
    let dt: DateTime<Utc> = match metadata.created() {
        Ok(t) => t.into(),
        Err(_) => Utc::now()
    };

    let file_info:FileItem = FileItem{
        name: match std::path::Path::new(full_path).file_name().unwrap().to_str() {Some(s) => s.to_string(), None => String::new()},
        size: metadata.len(),
        file_type: if metadata.is_dir() {String::from("dir")} else {String::from("file")},
        edit_date: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        hidden: metadata.file_attributes() & 0x00000002 != 0,
        full_path: full_path.to_string(),
        decrypted_name: None
    };
    return file_info;
}
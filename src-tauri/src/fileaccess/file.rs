use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Clone)]
pub struct FileItem {
    pub name: String,
    pub size: u64,
    pub file_type: String,
    pub edit_date: String,
    pub hidden: bool,
    pub full_path: String
}

impl Serialize for FileItem{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("FileItem", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("file_type", &self.file_type)?;
        state.serialize_field("edit_date", &self.edit_date)?;
        state.serialize_field("hidden", &self.hidden)?;
        state.serialize_field("full_path", &self.full_path)?;
        state.end()
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
        let dt: DateTime<Utc> = match metadata.created() {
            Ok(t) => t.into(),
            Err(_) => continue
        };

        let file_info:FileItem = FileItem{
            name: match item.file_name().into_string() {Ok(s) => s, Err(_) => continue},
            size: metadata.len(),
            file_type: if metadata.is_dir() {String::from("dir")} else {String::from("file")},
            edit_date: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            hidden: metadata.file_attributes() & 0x00000002 != 0,
            full_path: item.path().to_string_lossy().to_string()
        };
        list.insert(0,file_info);
    }
    
    return Ok(list);
}
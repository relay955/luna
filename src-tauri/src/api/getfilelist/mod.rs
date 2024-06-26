use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Utc};
use crate::fileaccess::FileItem;

#[tauri::command]
pub fn get_file_list(dir: &str, sort_by:&str, sort_direction:&str,
                 grouping_mode:&str, search:&str, filter:&str) -> Result<Vec<FileItem>,String> {
    let paths = match std::fs::read_dir(dir) {
        Ok(p) => p,
        Err(e) => return Err(e.to_string())
    };
    let mut list: Vec<FileItem>  = Vec::new();
    for item in paths {
        let item = match item {Ok(i) => i, Err(_) => continue};
        let metadata = match item.metadata() { Ok(m) => m, Err(_) => continue };
        let dt: DateTime<Utc> = match metadata.created() {
            Ok(t) => t.into(),
            Err(_) => continue
        };

        let mut file_info:FileItem = FileItem{
            name: match item.file_name().into_string() {Ok(s) => s, Err(_) => continue},
            size: metadata.len(),
            file_type: if metadata.is_dir() {String::from("dir")} else {String::from("file")},
            edit_date: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            hidden: metadata.file_attributes() & 0x00000002 != 0,
            full_path: item.path().to_string_lossy().to_string()
            // icon : String::new()
        };
        // let icon = match get_icon(&file_info.full_path) {
        //     Ok(icon) => icon,
        //     Err(_) => continue
        // };
        // icon vectors to base64
        // file_info.icon = general_purpose::STANDARD.encode(&icon);

        list.insert(0,file_info);
    }

    if sort_by == "name" {
        list.sort_by(|a, b| a.name.cmp(&b.name));
    } else if sort_by == "size" {
        list.sort_by(|a, b| a.size.cmp(&b.size));
    } else if sort_by == "edit_date" {
        list.sort_by(|a, b| a.edit_date.cmp(&b.edit_date));
    }

    if sort_direction == "desc" { list.reverse(); }

    if grouping_mode == "folder"{
        let mut dirs:Vec<FileItem> = list.clone().into_iter().filter(|x| x.file_type == "dir").collect();
        let mut files = list.into_iter().filter(|x| x.file_type == "file").collect();
        dirs.append(&mut files);
        list = dirs;
    }

    if !filter.is_empty() {
        list.retain(|x| x.name.contains(filter));
    }

    if list.len() < 500 { Ok(list) }
    else {
        Ok(list.splice(0..500, Vec::new()).collect())
    }
}

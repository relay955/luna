use crate::fileaccess::file::{get_file_info, FileItem};
use crate::indexer;

#[tauri::command]
pub fn search_files(command: &str) -> Result<Vec<FileItem>,String> {
    let file_paths = match indexer::search_files(command){
        Ok(paths) => paths,
        Err(e) => return Err(e.to_string())
    };

    let file_items = file_paths
        .iter()
        .map(|x| get_file_info(x).unwrap_or(FileItem::default()))
        .collect();

    Ok(file_items)
}

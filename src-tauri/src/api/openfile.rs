#[tauri::command]
pub fn open_file(file_path: &str) {
    let _ = std::process::Command::new("notepad")
        .arg(file_path)
        .output();
}
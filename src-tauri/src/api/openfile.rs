use crate::fileaccess::associatedprogram::get_associated_program;

#[tauri::command]
pub fn open_file(file_path: &str) {
    if file_path.ends_with(".exe") {
        let _ = async_process::Command::new(file_path)
            .output();
        return;
    }
    
    let associated_program = get_associated_program(file_path);
    println!("{:?}", associated_program);
    if associated_program.is_none() {
        return;
    }
    let _ = async_process::Command::new(associated_program.unwrap())
        .arg(file_path)
        .output();
}
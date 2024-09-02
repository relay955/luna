use crate::fileaccess::drive::DriveInfo;
use crate::fileaccess::drive;

#[tauri::command]
pub fn get_drive_list() -> Vec<DriveInfo> {
    return drive::get_drive_list();
}
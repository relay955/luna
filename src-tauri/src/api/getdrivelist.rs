use crate::fileaccess::{drive, file};
use crate::fileaccess::drive::DriveInfo;

#[tauri::command]
pub fn get_drive_list() -> Vec<DriveInfo> {
    return drive::get_drive_list();
}
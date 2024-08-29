use std::sync::Mutex;
use tauri::State;
use crate::api::{ApiError, ValidationError};
use crate::GlobalData;

#[tauri::command]
pub fn enter_protection_mode(mut global_data:State<Mutex<GlobalData>>, password:&str) -> Result<(),ApiError>{
    let mut global_data = global_data.lock().unwrap();
    if password.len() < 10 {
        return Err(ValidationError::PasswordTooShort.into());
    }
    global_data.encryption_key = Some(password.to_string());
    Ok(())
}

#[tauri::command]
pub fn exit_protection_mode(mut global_data:State<Mutex<GlobalData>>){
    let mut global_data = global_data.lock().unwrap();
    global_data.encryption_key = None;
}

#[tauri::command]
pub fn is_in_protection_mode(global_data:State<Mutex<GlobalData>>) -> bool {
    let mut global_data = global_data.lock().unwrap();
    global_data.encryption_key.is_some()
}

use std::ops::Add;
use std::path::Path;
use std::sync::Mutex;
use tauri::State;
use crate::api::{ApiError, ValidationError};
use crate::GlobalData;
use crate::module::crypto::encrypt_binary_with_iv;
use crate::module::enc_metadata::key_to_enc_metadata_signature;
use crate::module::random_util::generate_hex_string;

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

#[tauri::command]
pub fn encrypt_file(global_data:State<Mutex<GlobalData>>, full_path:&str) -> Result<(),ApiError>{
    let global_data = global_data.lock().unwrap();
    let encryption_key = global_data.encryption_key.clone();
    drop(global_data);
    if encryption_key.is_none() {
        return Err(ApiError::Validation{
            code: "NOT_IN_PROTECTION_MODE".to_string(),
            msg: "보호 모드가 아닙니다.".to_string()
        });
    }
    let encryption_key = encryption_key.unwrap();
    
    let enc_metadata_signature = key_to_enc_metadata_signature(encryption_key.as_str());
    
    let mut file = std::fs::read(full_path)?;
    let mut rand_file_name = generate_hex_string(20);
    while rand_file_name.starts_with(&enc_metadata_signature){
        rand_file_name = generate_hex_string(20);
    }
    rand_file_name = rand_file_name.add(".encrypted");

    encrypt_binary_with_iv(encryption_key.as_str(), &mut file);
    let full_path = Path::new(full_path);
    let full_path = full_path.with_file_name(rand_file_name);
    
    
    std::fs::write(full_path, file)?;
    Ok(())
}
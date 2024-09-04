use crate::api::{ApiError, ValidationError};
use crate::module::crypto::{decrypt_binary_with_iv, encrypt_binary_with_iv};
use crate::module::enc_metadata::{EncFileItem, EncMetadata};
use crate::Protection;
use std::path::Path;
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub fn enter_protection_mode(mut protection:State<RwLock<Protection>>, password:&str) -> Result<(),ApiError>{
    if password.len() < 10 {
        return Err(ValidationError::PasswordTooShort.into());
    }
    {
        let mut protection = protection.write().unwrap();
        protection.key = Some(password.to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn exit_protection_mode(mut protection:State<RwLock<Protection>>){
    let mut protection = protection.write().unwrap();
    protection.key = None;
}

#[tauri::command]
pub fn is_in_protection_mode(protection:State<RwLock<Protection>>) -> bool {
    let mut protection = protection.read().unwrap();
    protection.key.is_some()
}

#[tauri::command]
pub fn encrypt_file(protection:State<RwLock<Protection>>, full_path:&str) -> Result<(),ApiError>{
    let protection = protection.read().unwrap();
    let key = protection.key.as_ref()
        .ok_or(ValidationError::NotInProtectionMode)?;

    let full_path = Path::new(full_path);
    let metadata_path = full_path.parent().ok_or(ValidationError::ParseFailed)?;
    let metadata_path = metadata_path.to_str().ok_or(ValidationError::ParseFailed)?;
    let mut enc_metadata = EncMetadata::open(metadata_path, key)?;
    
    let mut file = std::fs::read(full_path)?;
    let rand_file_name = enc_metadata.generate_random_file_name(true);
    let encrypted_full_path = full_path.with_file_name(&rand_file_name);

    encrypt_binary_with_iv(key, &mut file);
    
    std::fs::write(&encrypted_full_path, file)?;
    
    //메타데이터 업데이트
    let real_name = full_path.file_name().ok_or(ValidationError::ParseFailed)?;
    let real_name = real_name.to_str().ok_or(ValidationError::ParseFailed)?;
    let real_name = real_name.to_string();
    
    enc_metadata.insert(&rand_file_name,EncFileItem{
        real_name
    });
    
    enc_metadata.save(key)?;
    Ok(())
}
#[tauri::command]
pub fn force_decrypt_file(protection:State<RwLock<Protection>>, full_path:&str) -> Result<(),ApiError>{
    let protection = protection.read().unwrap();
    let key = protection.key.as_ref()
        .ok_or(ValidationError::NotInProtectionMode)?;

    let full_path_obj = Path::new(full_path);
    let metadata_path = full_path_obj.parent().ok_or(ValidationError::ParseFailed)?;
    let metadata_path = metadata_path.to_str().ok_or(ValidationError::ParseFailed)?;
    let mut enc_metadata = EncMetadata::open(metadata_path, key)?;
    
    let real_name = enc_metadata.get_enc_file_item(full_path)
        .ok_or(ValidationError::ParseFailed)?
        .real_name.clone();

    let mut file = std::fs::read(full_path)?;
    let decrypt_target_path = full_path_obj.with_file_name(&real_name);
    if decrypt_target_path.exists() {
        return Err(ValidationError::FileAlreadyExists.into());
    }
    
    decrypt_binary_with_iv(key, &mut file);

    std::fs::write(&decrypt_target_path, file)?;
    
    Ok(())
}

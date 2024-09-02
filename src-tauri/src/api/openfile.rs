use std::path::Path;
use std::sync::Mutex;
use heed::Env;
use tauri::State;
use winapi::um::winnt::FILE_ACTION_ADDED;
use crate::api::{ApiError, ValidationError};
use crate::db::luna_settings_accessor;
use crate::db::luna_settings_accessor::LunaSettingsAccessor;
use crate::fileaccess::associatedprogram::get_associated_program;
use crate::fileaccess::file::FileItem;
use crate::GlobalData;
use crate::module::crypto::decrypt_binary_with_iv;
use crate::module::enc_metadata::{key_to_enc_metadata_signature, EncMetadata};
use crate::module::hash::str_to_sha256_hex;

#[tauri::command]
pub fn open_file(global_data:State<Mutex<GlobalData>>,db:State<Env>, file_path: &str) -> Result<(),ApiError> {
    let global_data = global_data.lock().unwrap();
    let encryption_key = global_data.encryption_key.clone();
    drop(global_data);
    
    let mut file_path = file_path.to_string();

    if encryption_key.is_some() && file_path.ends_with(".encrypted") {
        let encryption_key = encryption_key.unwrap();
        let luna_settings = LunaSettingsAccessor::new(&db).get()?;
        
        let original_dir = Path::new(&file_path).parent().ok_or(ValidationError::ParseFailed)?
            .to_str().ok_or(ValidationError::ParseFailed)?;
        let original_dir_hash = str_to_sha256_hex(original_dir);
        let original_file_name = Path::new(&file_path).file_name().ok_or(ValidationError::ParseFailed)?
            .to_str().ok_or(ValidationError::ParseFailed)?;
        
        let mut decrypt_target_path = Path::new(&luna_settings.decrypt_temp_folder_path)
            .join(&original_dir_hash);
        
        //해당 임시폴더까지 경로에 폴더가 없다면 생성
        std::fs::create_dir_all(&decrypt_target_path)?;
        
        decrypt_target_path = decrypt_target_path.join(original_file_name);

        //파일이름 복호화 옵션이 켜져있다면 메타데이터에서 파일이름을 찾아서 복호화, 그렇지 않다면 원래이름 사용
        let real_name = decrypt_folder_name(&encryption_key, &file_path)
            .ok_or(ValidationError::DecryptFailed)?;
        
        if luna_settings.decrypt_file_name {
            decrypt_target_path.set_file_name(&real_name);
        }else{
            let real_ext = Path::new(&real_name).extension().ok_or(ValidationError::ParseFailed)?
                .to_str().ok_or(ValidationError::ParseFailed)?;
            decrypt_target_path.set_extension(real_ext);
        }
        
        //파일을 임시폴더에 복호화
        let mut file_binary = std::fs::read(&file_path)?;
        decrypt_binary_with_iv(&encryption_key,&mut file_binary);
        std::fs::write(&decrypt_target_path, file_binary)?;
        
        file_path = decrypt_target_path.to_str().ok_or(ValidationError::ParseFailed)?.to_string();
    }
    
    if file_path.ends_with(".exe") {
        let _ = async_process::Command::new(&file_path).output();
        return Ok(());
    }
    
    let associated_program = get_associated_program(&file_path);
    if associated_program.is_none() { return Ok(()); }
    
    let _ = async_process::Command::new(associated_program.unwrap())
        .arg(&file_path)
        .output();
    Ok(())
}

fn decrypt_folder_name(encryption_key:&str, file_path:&str) -> Option<String>{
    let file_name = Path::new(file_path).file_name()?.to_str()?;
    let dir = Path::new(file_path).parent()?.to_str()?;
    let enc_metadata_list = match EncMetadata::open(dir, encryption_key){
        Ok(m) => m,
        Err(_) => return None
    };

    let key = format!("file||{file_name}");
    let enc_metadata = enc_metadata_list.get(&key);
    if enc_metadata.is_none() { return None; }

    Some(enc_metadata?.real_name.clone())
}
use std::path::Path;
use std::sync::{Mutex, RwLock};
use heed::Env;
use tauri::State;
use winapi::um::winnt::FILE_ACTION_ADDED;
use crate::api::{ApiError, ValidationError};
use crate::db::luna_settings_accessor;
use crate::db::luna_settings_accessor::LunaSettingsAccessor;
use crate::fileaccess::associatedprogram::get_associated_program;
use crate::fileaccess::file::FileItem;
use crate::Protection;
use crate::module::crypto::decrypt_binary_with_iv;
use crate::module::enc_metadata::{EncMetadata};
use crate::module::hash::str_to_sha256_hex;

#[tauri::command]
pub fn open_file(protection:State<RwLock<Protection>>, db:State<Env>, file_path: &str) -> Result<(),ApiError> {
    let mut file_path = file_path.to_string();

    //암호화된 파일 복호화
    {
        let protection = protection.read().unwrap();
        let key = protection.key.as_ref();
        if key.is_some() && file_path.ends_with(".encrypted") {
            let key = key.unwrap();
            let luna_settings = LunaSettingsAccessor::new(&db).get()?;

            //원래 파일이 있던 폴더의 경로
            let original_dir = Path::new(&file_path)
                .parent().ok_or(ValidationError::ParseFailed)?
                .to_str().ok_or(ValidationError::ParseFailed)?;

            //복호화된 파일을 저장할 임시폴더 경로 (사용자 지정 임시폴더 경로 + 원래 폴더 경로의 SHA256 HEX의 조합)
            let mut decrypt_target_path = Path::new(&luna_settings.decrypt_temp_folder_path)
                .join(&str_to_sha256_hex(original_dir));

            //해당 임시폴더까지 경로에 폴더가 없다면 생성
            std::fs::create_dir_all(&decrypt_target_path)?;
            
            //복호화할 파일 경로 이름 지정
            decrypt_target_path = decrypt_target_path.join(Path::new(&file_path)
                .file_name().ok_or(ValidationError::ParseFailed)?
                .to_str().ok_or(ValidationError::ParseFailed)?);

            //파일이름 복호화 옵션이 켜져있다면 메타데이터에서 파일이름을 찾아서 복호화, 그렇지 않다면 원래이름 사용
            let enc_metadata = EncMetadata::open(original_dir, key)?;
            let real_name = enc_metadata.get_enc_file_item(&file_path)
                .ok_or(ValidationError::DecryptFailed)?.real_name.clone();

            if luna_settings.decrypt_file_name {
                decrypt_target_path.set_file_name(&real_name);
            } else {
                decrypt_target_path.set_extension(Path::new(&real_name)
                    .extension().ok_or(ValidationError::CantOpenNoExtFile)?
                    .to_str().ok_or(ValidationError::ParseFailed)?);
            }

            //파일을 임시폴더에 복호화
            let mut file_binary = std::fs::read(&file_path)?;
            decrypt_binary_with_iv(&key, &mut file_binary);
            std::fs::write(&decrypt_target_path, file_binary)?;

            file_path = decrypt_target_path.to_str().ok_or(ValidationError::ParseFailed)?.to_string();
        }
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
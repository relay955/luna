use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use crate::api::ApiError;
use crate::module::crypto::{decrypt_binary_with_iv, encrypt_binary_with_iv};
use crate::module::hash::str_to_sha256_hex;

#[derive(Serialize,Deserialize)]
pub struct EncMetadata {
    pub real_name: String
}

/// 키를 메타데이터를 식별하기 위한 서명 Hex4자리로 변환합니다.
pub fn key_to_enc_metadata_signature(key: &str) -> String {
    let key = key.to_string() + "enc_metadata_signature";
    let hex = str_to_sha256_hex(&key);
    let hex = str_to_sha256_hex(hex.as_str());
    hex[0..40].to_string()
}

impl EncMetadata {
    /// 암호화된 메타데이터 파일을 엽니다.
    /// 해당 폴더에 메타데이터가 없을 경우, 새 메타데이터 파일을 생성하고 빈 HashMap을 반환합니다.
    pub fn open(folder_path:&str,key:&str) -> Result<HashMap<String,EncMetadata>,ApiError> {
        let metadata_file_name = key_to_enc_metadata_signature(key);
        let metadata_path = folder_path.to_string() + "/" + metadata_file_name.as_str()+".encrypted";
        //메타데이터 파일 바이너리 불러오기
        let mut metadata_binary_buf = std::fs::read(metadata_path);
        
        //메타데이터 파일이 없을 경우, 빈 HashMap 반환
        if metadata_binary_buf.is_err() {
            return Ok(HashMap::new());
        }
        
        let mut metadata_binary_buf = metadata_binary_buf?;

        //키로 복호화
        decrypt_binary_with_iv(&key, &mut metadata_binary_buf);

        //json 파싱
        let json_string = String::from_utf8(metadata_binary_buf)?;
        let metadata_list:HashMap<String, EncMetadata> = from_str(json_string.as_str()).unwrap();
        Ok(metadata_list)
    }
    
    /// 메타데이터를 저장합니다.
    pub fn save(folder_path:&str,key:&str,metadata_list:&HashMap<String,EncMetadata>) -> Result<(),ApiError> {
        let metadata_file_name = key_to_enc_metadata_signature(key);
        let metadata_path = folder_path.to_string() + "/" + metadata_file_name.as_str()+".encrypted";
        //json으로 변환
        let json_string = serde_json::to_string(metadata_list)?;

        //메타데이터 파일 바이너리로 저장
        let mut metadata_binary_buf = json_string.into_bytes();
        encrypt_binary_with_iv(&key, &mut metadata_binary_buf);
        std::fs::write(metadata_path, metadata_binary_buf)?;
        Ok(())
    }
    
    pub fn insert(folder_path:&str, key:&str, value:EncMetadata) {
        let mut metadata_list = EncMetadata::open(folder_path, key).unwrap();
        metadata_list.insert(key.to_string(), value);
        EncMetadata::save(folder_path, key, &metadata_list).unwrap();
    }
}

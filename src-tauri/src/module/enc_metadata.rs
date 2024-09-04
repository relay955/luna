use crate::api::{ApiError, ValidationError};
use crate::module::crypto::{decrypt_binary_with_iv, encrypt_binary_with_iv};
use crate::module::hash::str_to_sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::path::Path;
use windows::Win32::Foundation::FVE_E_POLICY_PROHIBITS_SELFSIGNED;

#[derive(Serialize,Deserialize,Clone)]
pub struct EncFileItem {
    pub real_name: String
}

pub struct EncMetadata{
    full_path:String,//메타데이터 파일의 전체 경로. 불러온 파일을 그대로 저장할 위치를 특정하기 위해 사용되는 내부 변수
    signature:String,//메타데이터 파일의 시그니쳐. .encrypted 확장자는 제외됩니다.
    pub enc_file_items: HashMap<String,EncFileItem>//메타데이터 파일이 가지는 파일의 복호화된 메타정보
}


impl EncMetadata {
    /// 키를 메타데이터를 식별하기 위한 서명 Hex4자리로 변환합니다.
    fn key_to_enc_metadata_signature(key: &str) -> String {
        //TODO 시그니쳐 생성 방식때문에 해시충돌공격에 취약할수있음
        let key = key.to_string() + "enc_metadata_signature";
        let hex = str_to_sha256_hex(&key);
        let hex = str_to_sha256_hex(hex.as_str());
        hex[0..40].to_string()
    }
    
    /// 파일을 저장하기 위한 새로운 랜덤 이름을 생성합니다. 이 이름은 hex 40자리로 구성되며, .encrypted 확장자를 포함합니다.
    /// 해당 메타데이터의 파일 이름과 겹치지 않는다는 것을 보장합니다.
    pub fn generate_random_file_name(&self, with_ext:bool) -> String {
        let mut rand_file_name = crate::module::random_util::generate_hex_string(20);
        while rand_file_name.starts_with(&self.signature){
            rand_file_name = crate::module::random_util::generate_hex_string(20);
        }
        if with_ext {
            rand_file_name = rand_file_name + ".encrypted";
        }
        rand_file_name
    }

    ///대상 파일 경로의 파일이 자기자신인지 확인합니다.
    pub fn is_target_metadata(&self, file_path:&str) -> bool {
        self.full_path == file_path
    }
    
    /// 암호화된 메타데이터 파일을 엽니다.
    /// 해당 폴더에 메타데이터가 없을 경우, 비어있는 메타데이터 객체를 반환합니다.
    pub fn open(folder_path:&str,key:&str) -> Result<EncMetadata,ApiError> {
        let metadata_signature = EncMetadata::key_to_enc_metadata_signature(key);
        let metadata_filename = metadata_signature.clone()+".encrypted";
        let metadata_path = Path::new(folder_path).join(&metadata_filename);
        let metadata_path = metadata_path.to_str().ok_or(ValidationError::ParseFailed)?;
        
        //메타데이터 파일 바이너리 불러오기
        let mut metadata_binary_buf = std::fs::read(metadata_path);
        
        //메타데이터 파일이 없을 경우, 빈 HashMap 반환
        if metadata_binary_buf.is_err() { return Ok(EncMetadata{
                signature: metadata_signature,
                full_path: metadata_path.to_string(),
                enc_file_items: HashMap::new()
        });}
        
        let mut metadata_binary_buf = metadata_binary_buf?;

        //키로 복호화
        decrypt_binary_with_iv(&key, &mut metadata_binary_buf);

        //json 파싱
        let json_string = String::from_utf8(metadata_binary_buf)?;

        Ok(EncMetadata{
            signature: metadata_signature,
            full_path: metadata_path.to_string(),
            enc_file_items: from_str(json_string.as_str())?
        })
    }
    
    /// 메타데이터를 저장합니다.
    pub fn save(&self,key:&str) -> Result<(),ApiError> {
        //json으로 변환
        let json_string = serde_json::to_string(&self.enc_file_items)?;

        //메타데이터 파일 바이너리로 저장
        let mut metadata_binary_buf = json_string.into_bytes();
        encrypt_binary_with_iv(&key, &mut metadata_binary_buf);
        std::fs::write(&self.full_path, metadata_binary_buf)?;
        Ok(())
    }
    
    pub fn insert(&mut self, key:&str, value:EncFileItem) {
        self.enc_file_items.insert(key.to_string(), value);
    }
    
    ///메타데이터를 이용해, 파일 경로에 해당하는 대상 파일의 메타데이터를 얻습니다.
    pub fn get_enc_file_item(&self, file_path:&str) -> Option<&EncFileItem>{
        let file_name = Path::new(file_path).file_name()?.to_str()?;
        Some(self.enc_file_items.get(file_name)?)
    }
}

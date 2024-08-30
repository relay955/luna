use serde::{Deserialize, Serialize};
use crate::module::hash::str_to_sha256_hex;

#[derive(Serialize,Deserialize)]
pub struct EncMetadata {
    pub file_type: String,
    pub real_name: String
}

/// 키를 메타데이터를 식별하기 위한 서명 Hex4자리로 변환합니다.
pub fn key_to_enc_metadata_signature(key: &str) -> String {
    let hex = str_to_sha256_hex(key);
    let hex = str_to_sha256_hex(hex.as_str());
    hex[0..4].to_string()
}

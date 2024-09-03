use heed::MdbError;
use serde::{Deserialize, Serialize};
use std::string::FromUtf8Error;
use tauri::InvokeError;

pub mod getfilelist;
pub mod openfile;
pub mod geticons;
pub mod getdrivelist;
pub mod getfavoritefolder;
pub mod addfavoritefolder;
pub mod searchfiles;
pub mod protection_api;
pub mod luna_settings_api;

#[derive(Debug,thiserror::Error)]
pub enum ApiError{
    #[error("Io::에러가 발생했습니다.")]
    Io(#[from] std::io::Error),
    #[error("Json::에러가 발생했습니다.")]
    Json(#[from] serde_json::Error),
    #[error("FromUtf8::에러가 발생했습니다.")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("heed::에러가 발생했습니다.")]
    Heed(#[from] heed::Error),
    #[error("{code}::{msg}")]
    Validation {
        code: String,
        msg: String
    }
}


impl From<ApiError> for InvokeError{
    fn from(value: ApiError) -> Self {
        InvokeError::from(value.to_string())
    }
}

pub  enum ValidationError{
    PasswordTooShort,ParseFailed,DBOpenFailed,
    DecryptFailed,NotInProtectionMode
}

impl From<ValidationError> for ApiError{
    fn from(value: ValidationError) -> Self {
        match value {
            ValidationError::PasswordTooShort => ApiError::Validation{
                code: "PASSWORD_TOO_SHORT".to_string(),
                msg: "비밀번호는 10자 이상이어야 합니다.".to_string()
            },
            ValidationError::ParseFailed => ApiError::Validation{
                code: "PARSE_FAILED".to_string(),
                msg: "파싱에 실패했습니다.".to_string()
            },
            ValidationError::DBOpenFailed => ApiError::Validation{
                code: "DB_OPEN_FAILED".to_string(),
                msg: "DB 열기에 실패했습니다.".to_string()
            },
            ValidationError::DecryptFailed => ApiError::Validation{
                code: "DECRYPT_FAILED".to_string(),
                msg: "파일 복호화에 실패했습니다.".to_string()
            },
            ValidationError::NotInProtectionMode => ApiError::Validation{
                code: "NOT_IN_PROTECTION_MODE".to_string(),
                msg: "보호 모드가 아닙니다.".to_string()
            }
        }
    }
}

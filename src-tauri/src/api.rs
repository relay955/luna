use serde::{Deserialize, Serialize};
use tauri::InvokeError;

pub mod getfilelist;
pub mod openfile;
pub mod geticons;
pub mod getdrivelist;
pub mod getfavoritefolder;
pub mod addfavoritefolder;
pub mod searchfiles;
pub mod enterprotectionmode;

#[derive(Debug,thiserror::Error)]
pub enum ApiError{
    #[error("Io::에러가 발생했습니다.")]
    Io(#[from] std::io::Error),
    #[error("Json::에러가 발생했습니다.")]
    Json(#[from] serde_json::Error),
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
    PasswordTooShort
}

impl From<ValidationError> for ApiError{
    fn from(value: ValidationError) -> Self {
        match value {
            ValidationError::PasswordTooShort => ApiError::Validation{
                code: "PASSWORD_TOO_SHORT".to_string(),
                msg: "비밀번호는 10자 이상이어야 합니다.".to_string()
            }
        }
    }
}

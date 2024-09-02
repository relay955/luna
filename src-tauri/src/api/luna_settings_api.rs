use crate::api::{ApiError, ValidationError};
use heed::Env;
use tauri::State;
use crate::db::luna_settings_accessor::{LunaSettings, LunaSettingsAccessor};

#[tauri::command]
pub fn get_luna_settings(db:State<Env>) -> Result<LunaSettings,ApiError>{
    let luna_settings = LunaSettingsAccessor::new(&db).get()?;
    Ok(luna_settings)
}

#[tauri::command]
pub fn update_luna_settings(db:State<Env>, luna_settings: LunaSettings) -> Result<(),ApiError>{
    LunaSettingsAccessor::new(&db).update(luna_settings)?;
    Ok(())
}



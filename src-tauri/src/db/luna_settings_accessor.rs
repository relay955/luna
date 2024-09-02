use crate::api::{ApiError, ValidationError};
use heed::types::Str;
use heed::{Database, Env};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct LunaSettings{
    pub decrypt_folder_name:bool,
    pub decrypt_temp_folder_path:String
}
pub struct LunaSettingsAccessor<'a> {
    env:&'a Env
}

impl LunaSettings {
    pub fn new() -> LunaSettings {
        LunaSettings {
            decrypt_folder_name: false,
            decrypt_temp_folder_path: "C:\\decrypt_temp".to_string()
        }
    }
}

impl<'a> LunaSettingsAccessor<'a> {
    pub fn new(env:&'a Env) -> LunaSettingsAccessor<'a> {
        LunaSettingsAccessor {
            env
        }
    }

    pub fn get(&self) -> Result<LunaSettings,ApiError>{
        let mut rtxn = self.env.read_txn().or(Err(ValidationError::DBOpenFailed))?;
        
        let db:Option<Database<Str,Str>> = self.env.open_database(&mut rtxn, Some("settings"))?;
        let db  = match db {
            Some(db) => db,
            None => return Ok(LunaSettings::new())
        };
        
        let jsonstr = match db.get(&rtxn, "lunaSettings")? {
            Some(jsonstr) => jsonstr,
            None => return Ok(LunaSettings::new())
        };
        
        let luna_settings:LunaSettings = serde_json::from_str(jsonstr)?;
        
        Ok(luna_settings)
    }

    pub fn update(&self, luna_settings: LunaSettings) -> Result<(),ApiError> {
        let mut wtxn = self.env.write_txn()?;
        let db:Database<Str,Str> = self.env.create_database(&mut wtxn,Some("settings"))?;
        let jsonstr = serde_json::to_string(&luna_settings)?;
        db.put(&mut wtxn, "lunaSettings",&jsonstr)?;
        wtxn.commit()?;
        Ok(())
    }
}
use crate::fileaccess::icon;
use base64::engine::general_purpose;
use base64::Engine;
use std::collections::HashMap;

#[tauri::command]
pub fn get_icons(req:Vec<String>) -> Result<HashMap<String,String>,String> {
    let mut res: HashMap<String, String> = HashMap::new();
    for req_item in req {
        let icon = match icon::get_icon(&req_item) {
            Ok(icon) => icon,
            Err(_) => continue
        };
        // icon vectors to base64
        res.insert(req_item.clone(),general_purpose::STANDARD.encode(&icon));
    }
    Ok(res)
}

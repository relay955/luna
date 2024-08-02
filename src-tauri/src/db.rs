use std::fs;
use heed::{Env, EnvOpenOptions};

pub mod favorite_folder_accessor;

pub fn create_env() -> Env {
    let path = "./data";
    fs::create_dir_all(path).unwrap();
    unsafe {
        return EnvOpenOptions::new()
            .max_dbs(30)
            .open(path).unwrap();
    }
}
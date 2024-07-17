use std::fs;
use heed::{Env, EnvOpenOptions};

use lazy_static::lazy_static;

pub mod favoritefolder;

lazy_static!{
    static ref ENV:Env = unsafe {
      let path = "./data";
        fs::create_dir_all(path).unwrap();
        EnvOpenOptions::new()
        .max_dbs(30)
        .open(path).unwrap()
    };
}

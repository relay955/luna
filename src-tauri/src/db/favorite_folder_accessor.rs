use heed::{Database, Env, RwTxn};
use heed::types::Str;

pub struct FavoriteFolderAccessor<'a> {
    env:&'a Env
}

impl<'a> FavoriteFolderAccessor<'a> {
    pub fn new(env:&'a Env) -> FavoriteFolderAccessor<'a> {
        FavoriteFolderAccessor {
            env
        }
    }

    pub fn get(&self) -> Result<Vec<String>,Box<dyn std::error::Error>>{
        let mut rtxn = self.env.read_txn()?;
        let db:Option<Database<Str,Str>> = self.env.open_database(&mut rtxn, Option::from("favorite_folder"))?;
        let db = match db{
            Some(d) => d,
            None => return Ok(Vec::new())
        };
        
        let mut favorite_folders = Vec::new();
        let iter = db.iter(&rtxn)?;
        
        for result in iter{
            let (key, value) = result.unwrap();
            favorite_folders.push(key.to_string())
        }
        Ok(favorite_folders)
    }

    pub fn add(&self, folder_full_path:&str) -> Result<(),Box<dyn std::error::Error>> {
        let mut wtxn = self.env.write_txn()?;
        let db:Database<Str,Str> = self.env.create_database(&mut wtxn,Some("favorite_folder"))?;
        db.put(&mut wtxn, folder_full_path, "")?;
        wtxn.commit()?;
        Ok(())
    }
}
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub struct FileItem {
   pub name: String,
   pub size: u64,
   pub file_type: String,
   pub edit_date: String,
   pub hidden: bool
}

impl Serialize for FileItem{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("FileItem", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("file_type", &self.file_type)?;
        state.serialize_field("edit_date", &self.edit_date)?;
        state.serialize_field("hidden", &self.hidden)?;
        state.end()
    }
}

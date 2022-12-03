use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use crate::RawStore;

pub struct FileStore {
    path: String
}

impl FileStore {
    pub fn new<Type>(path: &str,init: Vec<Type>) -> Box<dyn RawStore<Type>>
    where Type: Send + Sync + Clone + Serialize + DeserializeOwned
    {
        let mut new_store = Box::new(
        FileStore {
            path: String::from(path)
        });
        new_store.push(init);
        new_store
    }
}

impl<Type> RawStore<Type> for FileStore
    where Type: Send + Sync + Clone + Serialize + DeserializeOwned
{
    fn push(&mut self,value:Vec<Type>) {
        if let Ok(mut new_file) = File::create(self.path.clone()) {
            new_file.write_all(serde_json::to_string(&value).unwrap().as_bytes()).unwrap();
        } else {
            let mut file = File::open(self.path.clone()).unwrap();
            file.write_all(serde_json::to_string(&value).unwrap().as_bytes()).unwrap();
        }
    }

    fn pull(&self) -> Vec<Type>  {
        if let Ok(mut file) = File::open(self.path.clone()) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or_default();
            serde_json::from_str(contents.as_str()).unwrap_or(vec![])
        } else {
            vec![]
        }
    }
}
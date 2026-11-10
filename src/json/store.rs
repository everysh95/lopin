use std::{fs::File, io::{Read, Write}};

use serde::{
    Deserialize, Serialize, de::DeserializeOwned,
};

use crate::Store;

impl<T: Clone + DeserializeOwned> Store<T> {
    pub fn read_json_file(path: &str) -> Result<Store<T>, ()> {
        if let Ok(mut file) = File::open(path) {
            let mut text = String::new();
            let _ = file.read_to_string(&mut text);
            return match serde_json::from_str(&text) {
                Ok(s) => Ok(s),
                Err(_) => Err(())
            };
        }
        Err(())
    }
}

impl<T: Clone + Serialize> Store<T> {

    pub fn write_json_file(self: &Self,path: &str) -> Result<(), ()> {
        if let Ok(mut file) = File::create(path) {
            if let Ok(text) = serde_json::to_string(self) {
                let _ = file.write(text.as_bytes());
                return Ok(());
            }
        }
        Err(())
    }
}



impl<'de,T: Clone + DeserializeOwned> Deserialize<'de> for Store<T> 
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let kv = Vec::<(usize,T)>::deserialize(deserializer)?;
        let mut result: Store<T> = Store::new();
        for (id, value) in kv {
            result = result.clone() | (id, value);
        }
        Ok(result)
    }
}

impl<T: Clone + Serialize> Serialize for Store<T> 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let kv: Vec<_> = self
            .get_ids()
            .iter()
            .zip(self.get_values().iter())
            .map(|(i, v)| (i.clone(), v.clone()))
            .collect();
        kv.serialize(serializer)
    }
}


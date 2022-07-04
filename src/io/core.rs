use crate::{RawStore, Store};
use async_trait::async_trait;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

struct TextFileStore {
    path: String,
}

#[async_trait]
impl RawStore<String> for TextFileStore {
    async fn get(&mut self) -> Option<String> {
        if let Ok(mut file) = File::open(Path::new(&self.path)) {
            let mut s = String::new();
            if let Ok(_) = file.read_to_string(&mut s) {
                return Some(s.clone());
            }
        }
        None
    }
    async fn put(&mut self, value: String) {
        let path = Path::new(&self.path);
        if let Ok(mut file) = File::create(path) {
            if let Err(why) = file.write_all(value.as_bytes()) {
                panic!("couldn't write to {}: {}", path.display(), why);
            }
        }
    }
}

pub fn text_file(path: &str) -> Store<String> {
    Store::new(Arc::new(Mutex::new(TextFileStore {
        path: path.to_string()
    })))
}
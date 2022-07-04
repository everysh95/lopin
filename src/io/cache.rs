use crate::{RawStore, Store};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

struct InMemoryCacheStore<T: Clone + Send + Sync> {
    cache: Option<T>,
    store: Store<T>
}


#[async_trait]
impl<T: Clone + Send + Sync> RawStore<T> for InMemoryCacheStore<T> {
    async fn get(&mut self) -> Option<T> {
        if let Some(data) = self.cache.clone() {
            return Some(data);
        } else if let Some(data) = self.store.get().await {
            self.cache = Some(data.clone());
            return Some(data);
        }
        None
    }
    async fn put(&mut self, value: T) {
        self.cache = Some(value.clone());
        self.store.put(value.clone()).await;
    }
}



pub fn in_memory_cache<T: Clone + Send + Sync + 'static>(store: Store<T>) -> Store<T> {
    Store::new(Arc::new(Mutex::new(
        InMemoryCacheStore {
            store: store,
            cache: None
        }
    )))
}
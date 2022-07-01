use super::core::{RawStore, Store};
use async_trait::async_trait;
use std::marker::Send;
use std::ops::BitOr;
use std::sync::Arc;
use tokio::sync::Mutex;

struct Merge<T: 'static + Clone + Send + Sync> {
    base: Store<Vec<T>>,
    add: Store<Vec<T>>,
}

#[async_trait]
impl<T: 'static + Clone + Send + Sync> RawStore<Vec<T>> for Merge<T> {
    async fn get(&self) -> Option<Vec<T>> {
        let base_value = self.base.get().await;
        let add_value = self.add.get().await;
        let mut result: Vec<T> = match base_value {
            Some(v) => v,
            None => vec![],
        };
        if let Some(v) = add_value {
            result = vec![result, v].concat();
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
    async fn put(&mut self, value: Vec<T>) {
        self.base.put(value.clone()).await;
        self.add.put(value.clone()).await;
    }
}

impl<T: 'static + Clone + Send + Sync> BitOr<Store<Vec<T>>> for Store<Vec<T>> {
    type Output = Store<Vec<T>>;
    fn bitor(self, rhs: Store<Vec<T>>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(Merge {
            base: self,
            add: rhs,
        })));
    }
}

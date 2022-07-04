use super::core::{RawStore, Store};
use async_trait::async_trait;
use std::ops::BitAnd;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait Condition<T> {
    async fn validation(&self, value: T) -> bool;
}

struct Select<T: Clone + Send + Sync> {
    store: Store<T>,
    condition: Arc<dyn Condition<T> + Send + Sync>,
}

#[async_trait]
impl<T: Clone + Send + Sync> RawStore<T> for Select<T> {
    async fn get(&mut self) -> Option<T> {
        let value = self.store.get().await;
        match value {
            Some(v) => {
                if self.condition.validation(v.clone()).await {
                    Some(v)
                } else {
                    None
                }
            }
            None => None,
        }
    }
    async fn put(&mut self, value: T) {
        if self.condition.validation(value.clone()).await {
            self.store.put(value).await;
        }
    }
}

impl<T: Clone + Send + Sync + 'static> BitAnd<Arc<dyn Condition<T> + Send + Sync>> for Store<T> {
    type Output = Store<T>;
    fn bitand(self, rhs: Arc<dyn Condition<T> + Send + Sync>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(Select {
            store: self,
            condition: rhs,
        })));
    }
}

pub struct SimpleSelect<T> {
    reference: T,
}

#[async_trait]
impl<T: Clone + Send + Sync + PartialEq> Condition<T> for SimpleSelect<T> {
    async fn validation(&self, value: T) -> bool {
        self.reference == value.clone()
    }
}

pub fn select<T: Clone + Send + Sync + PartialEq + 'static>(
    reference: &T,
) -> Arc<dyn Condition<T> + Send + Sync> {
    Arc::new(SimpleSelect {
        reference: reference.clone(),
    })
}

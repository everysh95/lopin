use crate::core::{RawStore, Store};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AssertEqStore<T: Clone + std::fmt::Debug + std::cmp::PartialEq> {
    value: T,
    init: T,
}

#[async_trait]
impl<T: Clone + Send + Sync + std::cmp::PartialEq + std::fmt::Debug + 'static> RawStore<T>
    for AssertEqStore<T>
{
    async fn get(&self) -> Option<T> {
        Some(self.init.clone())
    }
    async fn put(&mut self, value: T) {
        assert_eq!(self.value.clone(), value)
    }
}

pub fn assert_eq_store<T: Clone + Send + Sync + std::fmt::Debug + std::cmp::PartialEq + 'static>(
    value: T,
    init: T,
) -> Store<T> {
    Store::new(Arc::new(Mutex::new(AssertEqStore {
        value: value,
        init: init,
    })))
}

pub struct PrintStore {}

#[async_trait]
impl<T: Clone + Send + Sync + std::cmp::PartialEq + std::fmt::Display + 'static> RawStore<T>
    for PrintStore
{
    async fn get(&self) -> Option<T> {
        None
    }
    async fn put(&mut self, value: T) {
        println!("store value : {}", value)
    }
}

pub fn print_store<T: Clone + Send + Sync + std::cmp::PartialEq + std::fmt::Display + 'static>(
) -> Store<T> {
    Store::new(Arc::new(Mutex::new(PrintStore {})))
}

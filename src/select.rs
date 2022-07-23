use crate::Propaty;

use super::core::{RawStore, Store};
use async_trait::async_trait;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait RawCondition<T> {
    async fn validation_from(&self, value: T) -> bool;
    async fn validation_to(&self, value: T) -> bool;
}

pub struct Condition<T> {
    raw: Arc<dyn RawCondition<T> + Send + Sync>,
}

impl<T> Condition<T> {
    pub async fn validation_from(&self, value: T) -> bool {
        self.raw.validation_from(value).await
    }
    pub async fn validation_to(&self, value: T) -> bool {
        self.raw.validation_to(value).await
    }
    pub fn new(raw: Arc<dyn RawCondition<T> + Send + Sync>) -> Condition<T> {
        Condition { raw }
    }
}

struct Select<T: Clone + Send + Sync> {
    store: Store<T>,
    condition: Condition<T>,
}

#[async_trait]
impl<T: Clone + Send + Sync> RawStore<T> for Select<T> {
    async fn get(&mut self) -> Option<T> {
        let value = self.store.get().await;
        match value {
            Some(v) => {
                if self.condition.validation_to(v.clone()).await {
                    Some(v)
                } else {
                    None
                }
            }
            None => None,
        }
    }
    async fn put(&mut self, value: T) {
        if self.condition.validation_from(value.clone()).await {
            self.store.put(value).await;
        }
    }
}

impl<T: Clone + Send + Sync + 'static> BitAnd<Condition<T>> for Store<T> {
    type Output = Store<T>;
    fn bitand(self, rhs: Condition<T>) -> Self::Output {
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
impl<T: Clone + Send + Sync + PartialEq> RawCondition<T> for SimpleSelect<T> {
    async fn validation_from(&self, _value: T) -> bool {
        true
    }
    async fn validation_to(&self, value: T) -> bool {
        self.reference == value.clone()
    }
}

pub fn select<T: Clone + Send + Sync + PartialEq + 'static>(reference: &T) -> Condition<T> {
    Condition::new(Arc::new(SimpleSelect {
        reference: reference.clone(),
    }))
}

struct SelectPropatyGet<T> {
    key: T,
}

#[async_trait]
impl<T: Clone + Send + Sync + PartialEq> RawCondition<Propaty<T>> for SelectPropatyGet<T> {
    async fn validation_to(&self, value: Propaty<T>) -> bool {
        self.key.clone() == value.key.clone()
    }
    async fn validation_from(&self, _value: Propaty<T>) -> bool {
        true
    }
}

pub fn select_propaty_get<T: Clone + Send + Sync + PartialEq + 'static>(key: &T) -> Condition<Propaty<T>> {
    Condition::new(Arc::new(SelectPropatyGet {
        key: key.clone(),
    }))
}

struct SelectPropatyPut<T> {
    key: T,
}

#[async_trait]
impl<T: Clone + Send + Sync + PartialEq> RawCondition<Propaty<T>> for SelectPropatyPut<T> {
    async fn validation_to(&self, _value: Propaty<T>) -> bool {
        true
    }
    async fn validation_from(&self, value: Propaty<T>) -> bool {
        self.key.clone() == value.key.clone()
    }
}

pub fn select_propaty_put<T: Clone + Send + Sync + PartialEq + 'static>(key: &T) -> Condition<Propaty<T>> {
    Condition::new(Arc::new(SelectPropatyPut {
        key: key.clone(),
    }))
}

struct GetOnly;

#[async_trait]
impl<T: 'static + Clone + Send + Sync + PartialEq> RawCondition<T> for GetOnly {
    async fn validation_from(&self, _value: T) -> bool {
        false
    }
    async fn validation_to(&self, _value: T) -> bool {
        true
    }
}

pub fn get_only<T: Clone + Send + Sync + PartialEq + 'static>() -> Condition<T> {
    Condition::new(Arc::new(GetOnly))
}

struct PutOnly;

#[async_trait]
impl<T: 'static + Clone + Send + Sync + PartialEq> RawCondition<T> for PutOnly {
    async fn validation_from(&self, _value: T) -> bool {
        false
    }
    async fn validation_to(&self, _value: T) -> bool {
        true
    }
}

pub fn put_only<T: Clone + Send + Sync + PartialEq + 'static>() -> Condition<T> {
    Condition::new(Arc::new(PutOnly))
}


struct VecSelect<T: Clone + Send + Sync> {
    store: Store<Vec<T>>,
    condition: Condition<T>,
}

#[async_trait]
impl<T: Clone + Send + Sync> RawStore<Vec<T>> for VecSelect<T> {
    async fn get(&mut self) -> Option<Vec<T>> {
        let value = self.store.get().await;
        match value {
            Some(raw_value) => {
                let mut result: Vec<T> = vec![];
                for v in raw_value.iter() {
                    if self.condition.validation_to(v.clone()).await {
                        result.push(v.clone());
                    }
                }
                Some(result)
            }
            None => None,
        }
    }
    async fn put(&mut self, value: Vec<T>) {
        let mut result: Vec<T> = vec![];
        for v in value.iter() {
            if self.condition.validation_from(v.clone()).await {
                result.push(v.clone());
            }
        }
        self.store.put(result).await;
    }
}

impl<T: Clone + Send + Sync + 'static> BitAnd<Condition<T>> for Store<Vec<T>> {
    type Output = Store<Vec<T>>;
    fn bitand(self, rhs: Condition<T>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(VecSelect {
            store: self,
            condition: rhs,
        })));
    }
}

struct MultiCondition<T: Clone + Send + Sync> {
    lhs: Condition<T>,
    rhs: Condition<T>,
}

#[async_trait]
impl<T: Clone + Send + Sync> RawCondition<T> for MultiCondition<T> {
    async fn validation_from(&self, value: T) -> bool {
        self.lhs.validation_from(value.clone()).await || self.rhs.validation_from(value.clone()).await
    }
    async fn validation_to(&self, value: T) -> bool {
        self.lhs.validation_to(value.clone()).await || self.rhs.validation_to(value.clone()).await
    }
}

impl<T: Clone + Send + Sync + 'static> BitOr<Condition<T>> for Condition<T> {
    type Output = Condition<T>;
    fn bitor(self, rhs: Condition<T>) -> Self::Output {
        return Condition::new(Arc::new(MultiCondition { lhs: self, rhs }));
    }
}

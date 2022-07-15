use crate::{Propaty, PropatyMap, RawStore, Store};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

struct Create<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
> {
    store: Store<Vec<Propaty<IDType>>>,
    key: KeyType,
    tmp_key: Option<IDType>,
}

struct Read<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
> {
    store: Store<Vec<Propaty<IDType>>>,
    key: KeyType,
    tmp_key: Option<IDType>,
}

struct Update<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
> {
    store: Store<Vec<Propaty<IDType>>>,
    key: KeyType,
    tmp_key: Option<IDType>,
}

struct Delete<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
> {
    store: Store<Vec<Propaty<IDType>>>,
    key: KeyType,
    tmp_key: Option<IDType>,
}

#[async_trait]
impl<
        KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
        IDType: 'static + PartialEq + Clone + Send + Sync,
    > RawStore<Vec<Propaty<KeyType>>> for Create<KeyType, IDType>
{
    async fn get(&mut self) -> Option<Vec<Propaty<KeyType>>> {
        if let Some(key) = self.tmp_key.clone() {
            if let Some(sv) = self.store.get().await {
                sv.get_value::<Vec<Propaty<KeyType>>>(&key)
            } else {
                None
            }
        } else {
            None
        }
    }
    async fn put(&mut self, value: Vec<Propaty<KeyType>>) {
        if let Some(key) = value.get_value::<IDType>(&self.key) {
            if let Some(mut sv) = self.store.get().await {
                if sv.iter().find(|p| p.key == key.clone()).is_some() {
                    self.tmp_key = Some(key.clone());
                    sv.push(Propaty::new(key, value));
                    self.store.put(sv).await;
                } else {
                    self.tmp_key = None;
                }
            } else {
                self.store.put(vec![Propaty::new(key, value)]).await;
            }
        }
    }
}

#[async_trait]
impl<
        KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
        IDType: 'static + PartialEq + Clone + Send + Sync,
    > RawStore<Vec<Propaty<KeyType>>> for Read<KeyType, IDType>
{
    async fn get(&mut self) -> Option<Vec<Propaty<KeyType>>> {
        if let Some(key) = self.tmp_key.clone() {
            if let Some(sv) = self.store.get().await {
                sv.get_value::<Vec<Propaty<KeyType>>>(&key)
            } else {
                None
            }
        } else {
            None
        }
    }
    async fn put(&mut self, value: Vec<Propaty<KeyType>>) {
        if let Some(key) = value.get_value::<IDType>(&self.key) {
            self.tmp_key = Some(key.clone());
        }
    }
}

#[async_trait]
impl<
        KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
        IDType: 'static + PartialEq + Clone + Send + Sync,
    > RawStore<Vec<Propaty<KeyType>>> for Update<KeyType, IDType>
{
    async fn get(&mut self) -> Option<Vec<Propaty<KeyType>>> {
        if let Some(key) = self.tmp_key.clone() {
            if let Some(sv) = self.store.get().await {
                sv.get_value::<Vec<Propaty<KeyType>>>(&key)
            } else {
                None
            }
        } else {
            None
        }
    }
    async fn put(&mut self, value: Vec<Propaty<KeyType>>) {
        if let Some(key) = value.get_value::<IDType>(&self.key) {
            if let Some(mut sv) = self.store.get().await {
                if sv.iter().find(|p| p.key == key.clone()).is_some() {
                    self.tmp_key = Some(key.clone());
                    sv = sv
                        .iter()
                        .map(|p| {
                            if p.key == key.clone() {
                                Propaty::new(key.clone(), value.clone())
                            } else {
                                p.clone()
                            }
                        })
                        .collect();
                    self.store.put(sv).await;
                } else {
                    self.tmp_key = None;
                }
            } else {
                self.tmp_key = None
            }
        } else {
            self.tmp_key = None;
        }
    }
}

#[async_trait]
impl<
        KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
        IDType: 'static + PartialEq + Clone + Send + Sync,
    > RawStore<Vec<Propaty<KeyType>>> for Delete<KeyType, IDType>
{
    async fn get(&mut self) -> Option<Vec<Propaty<KeyType>>> {
        if let Some(key) = self.tmp_key.clone() {
            if let Some(sv) = self.store.get().await {
                sv.get_value::<Vec<Propaty<KeyType>>>(&key)
            } else {
                None
            }
        } else {
            None
        }
    }
    async fn put(&mut self, value: Vec<Propaty<KeyType>>) {
        if let Some(key) = value.get_value::<IDType>(&self.key) {
            if let Some(mut sv) = self.store.get().await {
                if sv.iter().find(|p| p.key == key.clone()).is_some() {
                    self.tmp_key = Some(key.clone());
                    sv = sv
                        .iter()
                        .cloned()
                        .filter(|p| p.key != key.clone())
                        .collect();
                    self.store.put(sv).await;
                } else {
                    self.tmp_key = None;
                }
            } else {
                self.tmp_key = None
            }
        } else {
            self.tmp_key = None;
        }
    }
}

pub fn create<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
>(
    key: KeyType,
    store: Store<Vec<Propaty<IDType>>>,
) -> Store<Vec<Propaty<KeyType>>> {
    Store::new(Arc::new(Mutex::new(Create {
        key: key.clone(),
        store,
        tmp_key: None,
    })))
}

pub fn read<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
>(
    key: KeyType,
    store: Store<Vec<Propaty<IDType>>>,
) -> Store<Vec<Propaty<KeyType>>> {
    Store::new(Arc::new(Mutex::new(Read {
        key: key.clone(),
        store,
        tmp_key: None,
    })))
}

pub fn update<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
>(
    key: KeyType,
    store: Store<Vec<Propaty<IDType>>>,
) -> Store<Vec<Propaty<KeyType>>> {
    Store::new(Arc::new(Mutex::new(Update {
        key: key.clone(),
        store,
        tmp_key: None,
    })))
}

pub fn delete<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
>(
    key: KeyType,
    store: Store<Vec<Propaty<IDType>>>,
) -> Store<Vec<Propaty<KeyType>>> {
    Store::new(Arc::new(Mutex::new(Delete {
        key: key.clone(),
        store,
        tmp_key: None,
    })))
}
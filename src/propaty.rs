use crate::core::{Converter, Store};
use async_trait::async_trait;
use std::any::Any;
use std::fmt;
use std::marker::Send;
use std::sync::Arc;

pub trait PropatyValue: fmt::Debug {
    fn clone_arc(&self) -> Arc<dyn PropatyValue + Send + Sync>;
    fn get(&self) -> Arc<dyn Any + Send + Sync>;
    fn eq_value(&self, rhs: Arc<dyn Any + Send + Sync>) -> bool;
}

pub struct Propaty<KeyType> {
    pub key: KeyType,
    pub value: Arc<dyn PropatyValue + Send + Sync>,
}

impl<KeyType: 'static + Clone> Clone for Propaty<KeyType> {
    fn clone(&self) -> Self {
        Propaty {
            key: self.key.clone(),
            value: self.value.clone_arc(),
        }
    }
}

impl<KeyType: 'static + Clone + fmt::Debug> fmt::Debug for Propaty<KeyType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Propaty")
            .field("key", &self.key)
            .field("value", &self.value)
            .finish()
    }
}
impl<KeyType: 'static + Clone + Send + Sync + PartialEq> PartialEq for Propaty<KeyType> {
    fn eq(&self, rhs: &Propaty<KeyType>) -> bool {
        self.key.clone() == rhs.key.clone() && self.value.eq_value(rhs.get())
    }
}

impl<KeyType: 'static + Clone + Send + Sync + PartialEq> Propaty<KeyType> {
    pub fn new<T: 'static + Clone + Send + Sync + fmt::Debug + PartialEq + Any>(
        key: KeyType,
        value: T,
    ) -> Propaty<KeyType> {
        Propaty {
            key: key.clone(),
            value: Arc::new(value),
        }
    }
}

impl<T: 'static + Clone + Send + Sync + fmt::Debug + PartialEq + Any> PropatyValue for T {
    fn clone_arc(&self) -> Arc<dyn PropatyValue + Send + Sync> {
        Arc::new(self.clone())
    }
    fn get(&self) -> Arc<dyn Any + Send + Sync> {
        Arc::new(self.clone())
    }
    fn eq_value(&self, rhs: Arc<dyn Any + Send + Sync>) -> bool {
        match rhs.downcast_ref::<T>() {
            Some(rv) => rv == self,
            None => false,
        }
    }
}

impl<KeyType: 'static + Clone + Send + Sync> Propaty<KeyType> {
    fn get(&self) -> Arc<dyn Any + Send + Sync> {
        self.value.get()
    }
    pub fn rename(&self, new_key: &KeyType) -> Propaty<KeyType> {
        Propaty {
            key: new_key.clone(),
            value: self.value.clone_arc(),
        }
    }
}

pub async fn create_propaty<KeyType: 'static + PartialEq + Clone + Send + Sync>(
    store: Store<Vec<Propaty<KeyType>>>,
) -> Vec<Propaty<KeyType>> {
    match store.get().await {
        Some(v) => v,
        None => vec![],
    }
}

pub struct Named {
    name: String,
}

#[async_trait]
impl<T: 'static + Clone + Send + Sync + fmt::Debug + PartialEq + Any>
    Converter<T, Vec<Propaty<String>>> for Named
{
    async fn to(&self, src: T) -> Option<Vec<Propaty<String>>> {
        Some(vec![Propaty {
            key: self.name.clone(),
            value: Arc::new(src),
        }])
    }
    async fn from(&self, dist: Vec<Propaty<String>>) -> Option<T> {
        dist.get_value(&self.name)
    }
}

pub fn named(name: &str) -> Arc<Named> {
    Arc::new(Named {
        name: name.to_string(),
    })
}

pub struct GetValue {
    name: String,
}

pub fn get_value(name: &str) -> Arc<GetValue> {
    Arc::new(GetValue {
        name: name.to_string(),
    })
}

#[async_trait]
impl<T: 'static + Clone + Send + Sync + fmt::Debug + PartialEq + Any>
    Converter<Vec<Propaty<String>>, T> for GetValue
{
    async fn to(&self, src: Vec<Propaty<String>>) -> Option<T> {
        src.get_value(&self.name)
    }
    async fn from(&self, dist: T) -> Option<Vec<Propaty<String>>> {
        Some(vec![Propaty {
            key: self.name.clone(),
            value: Arc::new(dist),
        }])
    }
}

pub trait PropatyMap<KeyType> {
    fn get_value<T: 'static + Clone>(&self, key: &KeyType) -> Option<T>;
}

impl<KeyType: 'static + PartialEq + Clone + Send + Sync> PropatyMap<KeyType>
    for Vec<Propaty<KeyType>>
{
    fn get_value<T: 'static + Clone>(&self, key: &KeyType) -> Option<T> {
        match self.iter().find(|p| p.key == key.clone()) {
            Some(v) => match v.get().downcast_ref::<T>() {
                Some(v) => Some(v.clone()),
                None => None,
            },
            None => None,
        }
    }
}

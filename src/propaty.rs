use crate::convert::Converter;
use crate::{temporary, unwarp, unwarp_err, BroadcastConverter, RawConverter, Store};
use async_trait::async_trait;
use std::any::Any;
use std::fmt;
use std::marker::Send;
use std::sync::Arc;
use std::sync::Mutex;

pub trait PropatyValue: fmt::Debug {
    fn get(&self) -> Arc<dyn Any + Send + Sync>;
    fn eq_value(&self, rhs: Arc<dyn Any + Send + Sync>) -> bool;
    fn clone_value(&self) -> Arc<Mutex<dyn PropatyValue + Send + Sync>>;
}

pub struct Propaty<KeyType> {
    pub key: KeyType,
    pub value: Arc<Mutex<dyn PropatyValue + Send + Sync>>,
}

impl<KeyType: 'static + Clone> Clone for Propaty<KeyType> {
    fn clone(&self) -> Self {
        {
            let v_raw = self.value.lock().unwrap();
            Propaty {
                key: self.key.clone(),
                value: v_raw.clone_value(),
            }
        }
    }
}

impl<KeyType: 'static + Clone + fmt::Debug> fmt::Debug for Propaty<KeyType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.value.lock().unwrap();
        f.debug_struct("Propaty")
            .field("key", &self.key)
            .field("value", &value)
            .finish()
    }
}
impl<KeyType: 'static + Clone + Send + Sync + PartialEq> PartialEq for Propaty<KeyType> {
    fn eq(&self, rhs: &Propaty<KeyType>) -> bool {
        {
            let value = self.value.lock().unwrap();
            self.key.clone() == rhs.key.clone() && value.eq_value(rhs.get())
        }
    }
}

impl<KeyType: 'static + Clone + Send + Sync + PartialEq> Propaty<KeyType> {
    pub fn new<T: 'static + Clone + Send + Sync + fmt::Debug + PartialEq + Any>(
        key: KeyType,
        value: T,
    ) -> Propaty<KeyType> {
        Propaty {
            key: key.clone(),
            value: Arc::new(Mutex::new(value)),
        }
    }
}

impl<T: 'static + Clone + Send + Sync + fmt::Debug + PartialEq + Any> PropatyValue for T {
    fn get(&self) -> Arc<dyn Any + Send + Sync> {
        Arc::new(self.clone())
    }
    fn eq_value(&self, rhs: Arc<dyn Any + Send + Sync>) -> bool {
        match rhs.downcast_ref::<T>() {
            Some(rv) => rv == self,
            None => false,
        }
    }
    fn clone_value(&self) -> Arc<Mutex<dyn PropatyValue + Send + Sync>> {
        Arc::new(Mutex::new(self.clone()))
    }
}

impl<KeyType: 'static + Clone + Send + Sync> Propaty<KeyType> {
    pub fn get(&self) -> Arc<dyn Any + Send + Sync> {
        {
            let value = self.value.lock().unwrap();
            value.get()
        }
    }
    pub fn rename(&self, new_key: &KeyType) -> Propaty<KeyType> {
        Propaty {
            key: new_key.clone(),
            value: self.value.clone(),
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
    RawConverter<T, Vec<Propaty<String>>> for Named
{
    async fn to(&self, src: T) -> Option<Vec<Propaty<String>>> {
        Some(vec![Propaty::new(self.name.clone(), src)])
    }
    async fn from(&self, _old: Option<T>, dist: Vec<Propaty<String>>) -> Option<T> {
        dist.get_value(&self.name)
    }
}

pub fn named<T: Clone + Send + Sync + fmt::Debug + PartialEq + Any>(
    name: &str,
) -> Converter<T, Vec<Propaty<String>>> {
    Converter::new(Arc::new(Named {
        name: name.to_string(),
    }))
}

pub struct GetValue {
    name: String,
}

pub fn get_value<T: Clone + Send + Sync + fmt::Debug + PartialEq + Any>(
    name: &str,
) -> Converter<Vec<Propaty<String>>, T> {
    Converter::new(Arc::new(GetValue {
        name: name.to_string(),
    }))
}

#[async_trait]
impl<T: 'static + Clone + Send + Sync + fmt::Debug + PartialEq + Any>
    RawConverter<Vec<Propaty<String>>, T> for GetValue
{
    async fn to(&self, src: Vec<Propaty<String>>) -> Option<T> {
        src.get_value(&self.name)
    }
    async fn from(
        &self,
        _old: Option<Vec<Propaty<String>>>,
        dist: T,
    ) -> Option<Vec<Propaty<String>>> {
        Some(vec![Propaty::new(self.name.clone(), dist)])
    }
}

#[derive(Clone, Copy)]
pub enum UniqueOrder {
    First,
    Last,
}

pub trait PropatyMap<KeyType> {
    fn get_value<T: 'static + Clone>(&self, key: &KeyType) -> Option<T>;
    fn unique(&self, order: UniqueOrder) -> Vec<Propaty<KeyType>>;
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

    fn unique(&self, order: UniqueOrder) -> Vec<Propaty<KeyType>> {
        let ref_vec = match order {
            UniqueOrder::First => self.clone(),
            UniqueOrder::Last => {
                let mut r = self.clone();
                r.reverse();
                r
            }
        };
        ref_vec
            .iter()
            .zip(
                ref_vec
                    .iter()
                    .map(|p| ref_vec.iter().position(|rp| rp.key == p.key)),
            )
            .enumerate()
            .filter(|(i, (_v, r))| match r {
                Some(r) => r == i,
                None => true,
            })
            .map(|(_i, (v, _p))| v.clone())
            .collect()
    }
}

pub struct Unique {
    order: UniqueOrder,
}

pub fn unique_porpaty<KeyType: 'static + PartialEq + Clone + Send + Sync>(
    order: UniqueOrder,
) -> Converter<Vec<Propaty<KeyType>>, Vec<Propaty<KeyType>>> {
    Converter::new(Arc::new(Unique { order }))
}

#[async_trait]
impl<KeyType: 'static + PartialEq + Clone + Send + Sync>
    RawConverter<Vec<Propaty<KeyType>>, Vec<Propaty<KeyType>>> for Unique
{
    async fn to(&self, src: Vec<Propaty<KeyType>>) -> Option<Vec<Propaty<KeyType>>> {
        Some(src.unique(self.order))
    }
    async fn from(
        &self,
        _odl: Option<Vec<Propaty<KeyType>>>,
        dist: Vec<Propaty<KeyType>>,
    ) -> Option<Vec<Propaty<KeyType>>> {
        Some(dist)
    }
}

pub fn temporary_object(name: &str) -> Store<Vec<Propaty<String>>> {
    temporary::<Vec<Propaty<String>>>() ^ named::<Vec<Propaty<String>>>(name)
}

pub struct FlattenPropaty;

pub fn flatten_porpaties<KeyType: 'static + PartialEq + Clone + Send + Sync>(
) -> Converter<Vec<Vec<Propaty<KeyType>>>, Vec<Propaty<KeyType>>> {
    Converter::new(Arc::new(FlattenPropaty))
}

#[async_trait]
impl<KeyType: 'static + PartialEq + Clone + Send + Sync>
    RawConverter<Vec<Vec<Propaty<KeyType>>>, Vec<Propaty<KeyType>>> for FlattenPropaty
{
    async fn to(&self, src: Vec<Vec<Propaty<KeyType>>>) -> Option<Vec<Propaty<KeyType>>> {
        Some(src.iter().cloned().flatten().collect())
    }
    async fn from(
        &self,
        _old: Option<Vec<Vec<Propaty<KeyType>>>>,
        dist: Vec<Propaty<KeyType>>,
    ) -> Option<Vec<Vec<Propaty<KeyType>>>> {
        Some(vec![dist])
    }
}

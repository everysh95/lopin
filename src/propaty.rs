use crate::core::{Store, Converter};
use std::any::Any;
use std::fmt;
use std::ops::ShlAssign;

pub trait PropatyValue {
    fn clone_box(&self) -> Box<dyn PropatyValue>;
    fn get(&self) -> Box<dyn Any>;
    fn eq_value(&self,rhs: &Box<dyn Any>) -> bool;
}

pub struct Propaty<KeyType> {
    pub key: KeyType,
    pub value: Box<dyn PropatyValue>
}

impl<KeyType : 'static + Clone> Clone for Propaty<KeyType> {
    fn clone(&self)-> Self {
        Propaty {
            key: self.key.clone(),
            value: self.value.clone_box()
        }
    }
}

impl<KeyType : 'static + Clone + fmt::Debug> fmt::Debug for Propaty<KeyType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Propaty")
         .field("key", &self.key)
         .finish()
    }
}
impl<KeyType: 'static + Clone + PartialEq> PartialEq for Propaty<KeyType> {
    fn eq(&self, rhs: &Propaty<KeyType>) -> bool{
        self.key.clone() == rhs.key.clone() && self.value.eq_value(&rhs.get())
    }
}

impl<KeyType: 'static + Clone + PartialEq> Propaty<KeyType> {
    pub fn new<T: 'static + Clone + PartialEq + Any>(key: KeyType,value: T) -> Propaty<KeyType> {
        Propaty {
            key: key.clone(),
            value: Box::new(value)
        }
    }
}

impl<T: 'static + Clone + PartialEq + Any> PropatyValue for T {
    fn clone_box(&self) -> Box<dyn PropatyValue> {
        Box::new(self.clone())
    }
    fn get(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
    fn eq_value(&self,rhs: &Box<dyn Any>) -> bool {
        match rhs.downcast_ref::<T>() {
            Some(rv) => rv == self,
            None => false
        }
    }
}

impl<KeyType : 'static + Clone> Propaty<KeyType> {
    fn get(&self) -> Box<dyn Any> {
        self.value.get()
    }
    pub fn rename(&self,new_key: &KeyType) -> Propaty<KeyType>{
        Propaty {
            key: new_key.clone(),
            value: self.value.clone_box()
        }
    }
}


impl<KeyType : 'static + PartialEq + Clone> ShlAssign<& mut Store<Vec<Propaty<KeyType>>>> for Vec<Propaty<KeyType>> {

    fn shl_assign(& mut self, rhs: & mut Store<Vec<Propaty<KeyType>>>) {
        if let Some(value) = rhs.get() {
            for v in value.iter() {
                match self.iter().position(|p| p.key == v.key) {
                    Some(pos) => {
                        self[pos] = v.clone();
                    },
                    None => {
                        self.push(v.clone());
                    }
                }
            }
        }
    }
}

pub fn create_propaty<KeyType : 'static + PartialEq + Clone>(store : Store<Vec<Propaty<KeyType>>>) -> Vec<Propaty<KeyType>> {
    let mut store_ref = store;
    match store_ref.get() {
        Some(v) => v,
        None => vec![]
    }
}

pub struct Named {
    name: String
}

impl<T: 'static + Clone + PartialEq + Any> Converter<T,Propaty<String>> for Named {
    fn to(&self,src: T) -> Option<Propaty<String>> {
        Some(Propaty {
            key: self.name.clone(),
            value: Box::new(src)
        })
    }
    fn from(&self,dist:Propaty<String>) -> Option<T> {
        if self.name == dist.key {
            let value = dist.value.get();
            match value.downcast_ref::<T>() {
                Some(v) => Some(v.clone()),
                None => None
            }
        } else {
            None
        }
    }
}

pub fn named(name: &str) -> Box<Named> {
    Box::new(Named{
        name: name.to_string()
    })
}

pub struct GetValue {
    name: String
}

pub fn get_value(name: &str) -> Box<GetValue> {
    Box::new(GetValue{
        name: name.to_string()
    })
}

impl<T: 'static + Clone + PartialEq + Any> Converter<Vec<Propaty<String>>,T> for GetValue {
    fn to(&self,src: Vec<Propaty<String>>) -> Option<T> {
        src.get_value(&self.name)
    }
    fn from(&self,dist:T) -> Option<Vec<Propaty<String>>> {
        Some(vec![Propaty {
            key: self.name.clone(),
            value: Box::new(dist)
        }])
    }
}

pub trait PropatyMap<KeyType> {
    fn get_value<T : 'static + Clone>(&self, key: &KeyType) -> Option<T>;
}

impl<KeyType : 'static + PartialEq + Clone> PropatyMap<KeyType> for Vec<Propaty<KeyType>> {
    fn get_value<T : 'static + Clone>(&self, key: &KeyType) -> Option<T> {
        match self.iter().find(|p| p.key == key.clone()) {
            Some(v) => match v.get().downcast_ref::<T>() {
                Some(v) => Some(v.clone()),
                None => None
            },
            None => None
        }

    }
}
use crate::core::Converter;
use std::any::Any;

pub trait PropatyValue {
    fn clone_box(&self) -> Box<dyn PropatyValue>;
    fn get(&self) -> Box<dyn Any>;
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

impl<T: 'static + Clone + Any> PropatyValue for T {
    fn clone_box(&self) -> Box<dyn PropatyValue> {
        Box::new(self.clone())
    }
    fn get(&self) -> Box<dyn Any> {
        Box::new(self.clone())
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

pub struct Named {
    name: String
}

impl<T: 'static + Clone + Any> Converter<T,Propaty<String>> for Named {
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

impl<T: 'static + Clone + Any> Converter<Vec<Propaty<String>>,T> for GetValue {
    fn to(&self,src: Vec<Propaty<String>>) -> Option<T> {
        match src.iter().find(|p| p.key == self.name) {
            Some(v) => match v.get().downcast_ref::<T>() {
                Some(v) => Some(v.clone()),
                None => None
            },
            None => None
        }
    }
    fn from(&self,dist:T) -> Option<Vec<Propaty<String>>> {
        Some(vec![Propaty {
            key: self.name.clone(),
            value: Box::new(dist)
        }])
    }
}


use std::ops::BitAnd;
use crate::core::{Store,RawStore};

pub trait Condition<T> {
    fn validation(&self,value: &T) -> bool;
}

struct Select<T: Clone> {
    store: Store<T>,
    condition: Box<dyn Condition<T>>,
}

impl<T: Clone> RawStore<T> for Select<T> {
    fn get(&mut self) -> Option<T> {
        let value = self.store.get();
        match value {
            Some(v) => if self.condition.validation(&v) {Some(v)} else {None},
            None => None
        }
    }
    fn put(&mut self, value: &T) {
        if self.condition.validation(value) {
            self.store.put(value);
        }
    }
}

impl<T: Clone + 'static> BitAnd<Box<dyn Condition<T>>> for Store<T> {
    type Output = Store<T>;
    fn bitand(self, rhs: Box<dyn Condition<T>>) -> Self::Output {
        return Store::new(Box::new(Select {
            store: self,
            condition: rhs,
        }));
    }
}

pub struct SimpleSelect<T> {
    reference: T
}

impl<T: Clone + PartialEq> Condition<T> for SimpleSelect<T> {
    fn validation(&self,value: &T) -> bool {
        self.reference == value.clone()
    }
}

pub fn select<T : Clone + PartialEq + 'static>(reference : &T) -> Box<dyn Condition<T>> {
    Box::new(SimpleSelect{
        reference: reference.clone()
    })
}
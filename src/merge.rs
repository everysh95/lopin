
use crate::core::{RawStore , Store};
use crate::propaty::Propaty;
use std::ops::BitOr;

struct MergeSeed<KeyType: 'static + Clone> {
    store1: Store<Propaty<KeyType>>,
    store2: Store<Propaty<KeyType>>,
    key1: Option<KeyType>,
    key2: Option<KeyType>
}

impl<KeyType: 'static + Clone + PartialEq> RawStore<Vec<Propaty<KeyType>>> for MergeSeed<KeyType> {
    fn get(&mut self) -> Option<Vec<Propaty<KeyType>>> {
        let value1 = self.store1.get();
        let value2 = self.store2.get();
        let mut result : Vec<Propaty<KeyType>> = vec![];
        if let Some(v) = value1 {
            self.key1 = Some(v.key.clone());
            result.push(v);
        }
        if let Some(v) = value2 {
            self.key2 = Some(v.key.clone());
            result.push(v);
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
    fn put(&mut self, value: &Vec<Propaty<KeyType>>) {
        if let Some(k) = self.key1.clone() {
            if let Some(p) = value.iter().find(|p| p.key == k) {
                self.store1.put(p);
            }
        }
        if let Some(k) = self.key2.clone() {
            if let Some(p) = value.iter().find(|p| p.key == k) {
                self.store2.put(p);
            }
        }
    }
}


struct MergeVec<KeyType: 'static + Clone> {
    base: Store<Vec<Propaty<KeyType>>>,
    add: Store<Propaty<KeyType>>,
    add_key: Option<KeyType>,
}

impl<KeyType: 'static + Clone + PartialEq> RawStore<Vec<Propaty<KeyType>>> for MergeVec<KeyType> {
    fn get(&mut self) -> Option<Vec<Propaty<KeyType>>> {
        let base_value = self.base.get();
        let add_value = self.add.get();
        let mut result : Vec<Propaty<KeyType>> = match base_value {
            Some(v) => v,
            None => vec![]
        };
        if let Some(v) = add_value {
            self.add_key = Some(v.key.clone());
            result.push(v);
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
    fn put(&mut self, value: &Vec<Propaty<KeyType>>) {
        if let Some(k) = self.add_key.clone() {
            if let Some(p) = value.iter().find(|p| p.key == k) {
                self.add.put(p);
            }
        }
        self.base.put(value);
    }
}


impl<KeyType: 'static + Clone + PartialEq> BitOr<Store<Propaty<KeyType>>> for Store<Vec<Propaty<KeyType>>> {
    type Output = Store<Vec<Propaty<KeyType>>>;
    fn bitor(self, rhs: Store<Propaty<KeyType>>) -> Self::Output {
        return Store::new(Box::new(MergeVec {
            base: self,
            add: rhs,
            add_key: None,
        }));
    }
}

impl<KeyType: 'static + Clone + PartialEq> BitOr<Store<Propaty<KeyType>>> for Store<Propaty<KeyType>> {
    type Output = Store<Vec<Propaty<KeyType>>>;
    fn bitor(self, rhs: Store<Propaty<KeyType>>) -> Self::Output {
        return Store::new(Box::new(MergeSeed {
            store1: self,
            store2: rhs,
            key1: None,
            key2: None,
        }));
    }
}
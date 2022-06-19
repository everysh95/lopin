use crate::core::{RawStore, Store};

pub struct AssertEqStore<T : Clone + std::fmt::Debug  + std::cmp::PartialEq> {
   value: T ,
   init: T,
}

impl<T : Clone  + std::cmp::PartialEq + std::fmt::Debug> RawStore<T> for AssertEqStore<T> {
    fn get(&mut self) -> Option<T> {
        Some(self.init.clone())   
    }
    fn put(&mut self, value: &T) {
        assert_eq!(&self.value, value)
    }
}

pub fn assert_eq_store<T : Clone  + std::fmt::Debug  + std::cmp::PartialEq + 'static>(value: T, init: T) -> Store<T> {
    Store::new(
        Box::new(
            AssertEqStore {
                value: value,
                init: init,
            }
        )
    )
}
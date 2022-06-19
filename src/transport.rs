use crate::core::Store;
use crate::propaty::Propaty;
use std::ops::ShlAssign;

pub trait Transport<T : Clone> {
    fn transport(&self, store: &mut Store<T>);
}

impl<T : Clone> ShlAssign<Box<dyn Transport<T>>> for Store<T> {
    fn shl_assign(& mut self, rhs: Box<dyn Transport<T>>) {
        rhs.transport(self);
    }
}

pub struct SimpleTransport{
    from: String,
    to: String
}

impl Transport<Vec<Propaty<String>>> for SimpleTransport {
    fn transport(&self, store: &mut Store<Vec<Propaty<String>>>) {
        match store.get() {
            Some(value) => {
                match value.iter().find(|p| p.key == self.from) {
                    Some(v) => {
                        let result : Vec<Propaty<String>> = value.iter().cloned().map(|p| if p.key == self.to {
                            Propaty {
                                key: p.key,
                                value: v.value.clone_box()
                            }
                        } else {
                            v.clone()
                        }).collect();
                        store.put(&result);
                    },
                    None => {}
                }
            },
            None => {}
        }
    }
}

pub fn transport(from: &str,to: &str) -> Box<dyn Transport<Vec<Propaty<String>>>> {
    Box::new(
        SimpleTransport{
            from: from.to_string(),
            to: to.to_string()
        }
    )
}

pub struct Swap{
    from: String,
    to: String
}

impl Transport<Vec<Propaty<String>>> for Swap {
    fn transport(&self, store: &mut Store<Vec<Propaty<String>>>) {
        if let Some(value) = store.get() {
            let mut result: Vec<Propaty<String>> = value.clone();
            match value.iter().position(|p| p.key == self.from) {
                Some(index_form) => match value.iter().position(|p| p.key == self.to) {
                    Some(index_to) => {
                        result[index_form] = value[index_to].rename(&self.from);
                        result[index_to] = value[index_form].rename(&self.to);
                    },
                    None => {
                        result.push(value[index_form].rename(&self.to))
                    }
                },
                None => match value.iter().position(|p| p.key == self.to) {
                    Some(index_to) => {
                        result.push(value[index_to].rename(&self.from))
                    },
                    None => {}
                }
            }
            store.put(&result);
        }
    }
}

pub fn swap(from: &str,to: &str) -> Box<dyn Transport<Vec<Propaty<String>>>> {
    Box::new(
        Swap{
            from: from.to_string(),
            to: to.to_string()
        }
    )
}
use std::ops::BitXor;

pub struct Store<T: Clone> {
    raw: Box<dyn RawStore<T>>,
}

impl<T: Clone> Store<T> {
    pub fn get(&mut self) -> Option<T> {
        self.raw.get()
    }
    pub fn put(&mut self, value: &T) {
        self.raw.put(value)
    }
}

impl<T: Clone> Store<T> {
    pub fn new(raw: Box<dyn RawStore<T>>) -> Store<T> {
        Store { raw: raw }
    }
}

pub trait RawStore<T: Clone> {
    fn get(&mut self) -> Option<T>;
    fn put(&mut self, value: &T);
}



pub trait Converter<ST, DT> {
    fn to(&self,src:ST) -> Option<DT>;
    fn from(&self,dist:DT) -> Option<ST>;
}


struct Convert<ST: Clone, DT: Clone> {
    store: Store<ST>,
    convert: Box<dyn Converter<ST, DT>>,
}

impl<ST: Clone, DT: Clone> RawStore<DT> for Convert<ST, DT> {
    fn get(&mut self) -> Option<DT> {
        let value = self.store.get();
        match value {
            Some(v) => self.convert.to(v),
            None => None
        }
    }
    fn put(&mut self, value: &DT) {
        let put_value = self.convert.from(value.clone());
        match put_value {
            Some(v) => self.store.put(&v),
            None => {}
        }
    }
}

impl<ST: Clone + 'static, DT: Clone + 'static> BitXor<Box<dyn Converter<ST, DT>>> for Store<ST> {
    type Output = Store<DT>;
    fn bitxor(self, rhs: Box<dyn Converter<ST, DT>>) -> Self::Output {
        return Store::new(Box::new(Convert {
            store: self,
            convert: rhs,
        }));
    }
}

struct SimpleStore<T: Clone> {
    data: T,
}

impl<T: Clone> RawStore<T> for SimpleStore<T> {
    fn get(&mut self) -> Option<T> {
        Some(self.data.clone())
    }
    fn put(&mut self, value: &T) {
        self.data = value.clone();
    }
}

impl<T: Clone> SimpleStore<T> {
    fn new(data: T) -> SimpleStore<T> {
        SimpleStore { data: data }
    }
}

pub fn store<T: Clone + 'static>(data: T) -> Store<T> {
    Store::new(Box::new(SimpleStore::new(data)))
}

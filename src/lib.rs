use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Shl;
use std::marker::PhantomData;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 1 << 2;
        // x = store('x') | (x) -> x.a & (x) -> x == 'xx' << store('x') & (x) -> x
        assert_eq!(result, 4);
    }
}



pub trait Store<T : Clone> {
    fn get(&self) -> Vec<T>;
}

pub struct Select<T> {
    store: Box<dyn Store<T>>,
    condition: dyn Fn(&T) -> bool,
}

impl<T : Clone> Store<T> for Select<T> {
    fn get(&self) -> Vec<T> {
        let condition = &self.condition;
        return  self.store.get().iter().cloned().filter(|x| condition(&x)).collect();
    }
}

pub struct Merge<T> {
    store: Vec<Box<dyn Store<T>>>,
    phantom: PhantomData<T>,
}

impl<T : Clone> Store<T> for Merge<T> {
    fn get(&self) -> Vec<T> {
        return  self.store.iter().map(|x| x.get()).flatten().collect();
    }
}

pub struct Convert<ST,DT> {
    store: Box<dyn Store<ST>>,
    convert: dyn Fn(ST) -> DT,
}

impl<ST : Clone,DT : Clone> Store<DT> for Convert<ST, DT> {
    fn get(&self) -> Vec<DT> {
        let convert = &self.convert;
        return  self.store.get().iter().map(|x| convert(x.clone())).collect();
    }
}
use async_trait::async_trait;
use std::marker::Send;
use std::ops::{Add, Mul, ShlAssign};

#[async_trait]
pub trait RawStore<Type>
where
    Type: Send + Sync + Clone,
{
    fn push(&mut self, value: Vec<Type>);
    fn pull(&self) -> Vec<Type>;
}


#[derive(Clone)]
pub struct ValueStore<Type>
where
    Type: Send + Sync + Clone,
{
    value: Vec<Type>,
}

pub fn pull<Type>(store: &Box<dyn RawStore<Type>>) -> Box<dyn RawStore<Type>>
where
    Type: Send + Sync + Clone + 'static,
{
    ValueStore::new(store.pull())
}

impl<Type> ValueStore<Type>
where
    Type: Send + Sync + Clone + 'static,
{
    pub fn new(value:Vec<Type>) -> Box<dyn RawStore<Type>> {
        Box::new(
            ValueStore { value }
        )
    }
}

impl<Type> RawStore<Type> for ValueStore<Type>
where
    Type: Sync + Send + Clone
{
    fn pull(&self) -> Vec<Type> {
        self.value.clone()
    }
    fn push(&mut self,value:Vec<Type>) {
        self.value = value;
    }
}

impl<Type> Mul<Box<dyn Fn(&Type) -> bool>> for Box<dyn RawStore<Type>>
where
    Type: Sync + Send + Clone + 'static,
{
    type Output = Box<dyn RawStore<Type>>;

    fn mul(self, rhs: Box<dyn Fn(&Type) -> bool>) -> Self::Output {
        ValueStore::new(self.pull().iter().filter(|&v| rhs(v)).cloned().collect())
    }
}

impl<Type> Add<Type> for Box<dyn RawStore<Type>>
where
    Type: Sync + Send + Clone + 'static,
{
    type Output = Box<dyn RawStore<Type>>;

    fn add(self, rhs: Type) -> Self::Output {
        let mut new_value = self.pull();
        new_value.push(rhs);
        ValueStore::new(new_value)
    }
}

impl<Type> ShlAssign<Box<dyn RawStore<Type>>> for Box<dyn RawStore<Type>>
where
    Type: Sync + Send + Clone,
{
    fn shl_assign(&mut self, rhs: Box<dyn RawStore<Type>>) {
        self.push(rhs.pull())
    }
}
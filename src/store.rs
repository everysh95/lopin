use async_trait::async_trait;
use std::marker::Send;
use std::ops::{Add, Mul, ShlAssign, Div, Sub};

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

impl<Type> Div<Box<dyn Fn(&Type) -> bool>> for Box<dyn RawStore<Type>>
where
    Type: Sync + Send + Clone + 'static,
{
    type Output = Box<dyn RawStore<Type>>;

    fn div(self, rhs: Box<dyn Fn(&Type) -> bool>) -> Self::Output {
        ValueStore::new(self.pull().iter().filter(|&v| rhs(v)).cloned().collect())
    }
}

impl<Type> Mul<Box<dyn Fn(&Type) -> Type>> for Box<dyn RawStore<Type>>
where
    Type: Sync + Send + Clone + 'static,
{
    type Output = Box<dyn RawStore<Type>>;

    fn mul(self, rhs: Box<dyn Fn(&Type) -> Type>) -> Self::Output {
        ValueStore::new(self.pull().iter().map(|v| rhs(v)).collect())
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

impl<Type> Sub<Type> for Box<dyn RawStore<Type>>
where
    Type: Sync + Send + Clone + PartialEq + 'static,
{
    type Output = Box<dyn RawStore<Type>>;

    fn sub(self, rhs: Type) -> Self::Output {
        ValueStore::new(self.pull().iter().filter(|&v| v != &rhs).cloned().collect())
    }
}

impl<Type> Add<Box<dyn RawStore<Type>>> for Box<dyn RawStore<Type>>
where
    Type: Sync + Send + Clone + 'static,
{
    type Output = Box<dyn RawStore<Type>>;

    fn add(self, rhs: Box<dyn RawStore<Type>>) -> Self::Output {
        let mut old_value = self.pull();
        let mut new_value = rhs.pull();
        old_value.append(&mut new_value);
        ValueStore::new(old_value)
    }
}

impl<Type> Sub<Box<dyn RawStore<Type>>> for Box<dyn RawStore<Type>>
where
    Type: Sync + Send + Clone + PartialEq + 'static,
{
    type Output = Box<dyn RawStore<Type>>;

    fn sub(self, rhs: Box<dyn RawStore<Type>>) -> Self::Output {
        let remove_value = rhs.pull();
        ValueStore::new(self.pull().iter().filter(|&v| remove_value.iter().all(|r| v != r)).cloned().collect())
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
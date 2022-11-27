use crate::{RawPuller, Puller, RawPusher, Store, Pusher, RawStore};
use std::fmt::Debug;
use async_trait::async_trait;

struct ExpectEqPuller<Type>
where
    Type: Send + Sync + Clone + PartialEq,
{
    expect: Type
}

#[async_trait]
impl<Type> RawPuller<Type> for ExpectEqPuller<Type>
where
    Type: Send + Sync + Clone + PartialEq + Debug,
{
    async fn pull(&mut self,value: Type) {
        assert_eq!(self.expect, value);
    }
}

pub fn expect_eq<Type>(expect: Type) -> Puller<Type>
where
    Type: Send + Sync + Clone + PartialEq + Debug + 'static,
{
    Puller::new(
        ExpectEqPuller {
            expect
        }
    )
}

struct ExpectNePuller<Type>
where
    Type: Send + Sync + Clone + PartialEq,
{
    expect: Type
}

#[async_trait]
impl<Type> RawPuller<Type> for ExpectNePuller<Type>
where
    Type: Send + Sync + Clone + PartialEq + Debug,
{
    async fn pull(&mut self,value: Type) {
        assert_ne!(self.expect, value);
    }
}

pub fn expect_ne<Type>(expect: Type) -> Puller<Type>
where
    Type: Send + Sync + Clone + PartialEq + Debug + 'static,
{
    Puller::new(
        ExpectNePuller {
            expect
        }
    )
}

struct DirectPusher<Type>
where
    Type: Send + Sync + Clone,
{
    value: Type,
}

#[async_trait]
impl<Type> RawPusher<Type> for DirectPusher<Type>
where
    Type: Send + Sync + Clone,
{
    async fn awake(&mut self, store: &mut Store<Type>) {
        store.push(self.value.clone()).await;
    }
}

pub fn direct<Type>(value: Type) -> Pusher<Type>
where
    Type: Send + Sync + Clone + 'static,
{
    Pusher::new(DirectPusher { value })
}

struct ValueStore<Type>
where
    Type: Send + Sync + Clone + PartialEq,
{
    value: Option<Type>,
}

#[async_trait]
impl<Type> RawStore<Type> for ValueStore<Type>
where
    Type: Send + Sync + Clone + PartialEq,
{
    async fn push(&mut self, value: Type, puller_list: &mut Vec<Puller<Type>>) {
        if self.value != Some(value.clone()) {
            self.value = Some(value.clone());
            for p in puller_list.iter_mut() {
                p.pull(value.clone()).await;
            }
        }
    }
}

pub fn use_value<Type>(init_value: Option<Type>) -> Store<Type>
where
    Type: Send + Sync + Clone + PartialEq + 'static,
{
    Store::new(ValueStore { value: init_value })
}
use crate::{RawPuller, Puller};
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
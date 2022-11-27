use crate::Puller;
use async_trait::async_trait;
use std::marker::Send;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::ops::Shr;

#[async_trait]
pub trait RawStore<Type>
where
    Type: Sync + Send,
{
    async fn push(&mut self, value: Type, puller_list: &mut Vec<Puller<Type>>);
}

pub struct Store<Type>
where
    Type: Sync + Send,
{
    pullers: Vec<Puller<Type>>,
    raw: Arc<Mutex<dyn RawStore<Type> + Send + Sync>>,
}

impl<Type> Clone for Store<Type>
where
    Type: Sync + Send,
{
    fn clone(&self) -> Self {
        Store {
            pullers: self.pullers.clone(),
            raw: self.raw.clone(),
        }
    }
}

impl<Type> Store<Type>
where
    Type: Sync + Send,
{
    pub fn new<RawStoreType>(raw_store: RawStoreType) -> Self
    where
        RawStoreType: RawStore<Type> + Send + Sync + 'static,
    {
        Store {
            pullers: vec![],
            raw: Arc::new(Mutex::new(raw_store)),
        }
    }

    pub fn register(&mut self, puller: Puller<Type>) -> Self {
        self.pullers.push(puller);
        self.clone()
    }

    pub async fn push(&mut self, value: Type) {
        let mut mutable_raw = self.raw.lock().await;
        mutable_raw.push(value,&mut self.pullers).await;
    }
}

impl<Type> Shr<Puller<Type>> for Store<Type>
where
    Type: Sync + Send,
{
    type Output = Store<Type>;

    fn shr(self, rhs: Puller<Type>) -> Self::Output {
        let mut lhs = self.clone();
        lhs.register(rhs)
    }
}
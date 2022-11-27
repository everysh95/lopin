use async_trait::async_trait;
use std::marker::Send;
use std::ops::BitXor;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::Store;


#[async_trait]
pub trait RawPusher<Type> 
    where Type : Sync + Send
{
    async fn awake(&mut self,store: &mut Store<Type>);
}

pub struct  Pusher<Type> 
    where Type : Sync + Send
{
    store: Option<Store<Type>>,
    raw: Arc<Mutex<dyn RawPusher<Type> + Send + Sync>>
}

impl<Type> Clone for Pusher<Type>
    where Type : Sync + Send
{
    fn clone(&self) -> Self {
        Pusher { 
            store: self.store.clone(),
            raw: self.raw.clone()
        }
    }    
}

impl<Type> Pusher<Type>
    where Type : Sync + Send
{
    pub fn new<RawPusherType>(raw_puller: RawPusherType) -> Pusher<Type>
        where RawPusherType: RawPusher<Type> + Send + Sync + 'static
    {
        Pusher {
            store: None,
            raw: Arc::new(Mutex::new(raw_puller))
        }
    }

    pub fn register(&mut self,store: Store<Type>) -> Self{
        self.store = Some(store);
        self.clone()
    }

    pub async fn awake(&mut self){
        if let Some(mut store) = self.store.clone() {
            let mut mutable_raw = self.raw.lock().await;
            mutable_raw.awake(&mut store).await;
        }
    }
}

impl<Type> BitXor<Store<Type>> for Pusher<Type>
where
    Type: Sync + Send,
{
    type Output = Pusher<Type>;

    fn bitxor(self, rhs: Store<Type>) -> Self::Output {
        let mut mutable = self.clone();
        mutable.register(rhs)
    }
}


impl<Type> BitXor<Pusher<Type>> for  Store<Type>
where
    Type: Sync + Send,
{
    type Output = Pusher<Type>;

    fn bitxor(self, rhs: Pusher<Type>) -> Self::Output {
        let mut mutable = rhs.clone();
        mutable.register(self)
    }
}

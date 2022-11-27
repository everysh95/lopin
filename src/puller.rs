use async_trait::async_trait;
use std::marker::Send;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait RawPuller<Type> 
    where Type : Sync + Send
{
    async fn pull(&mut self,value: Type) ;
}

pub struct  Puller<Type> 
    where Type : Sync + Send
{
    raw: Arc<Mutex<dyn RawPuller<Type> + Send + Sync>>
}

impl<Type> Clone for Puller<Type>
    where Type : Sync + Send
{
    fn clone(&self) -> Self {
        Puller { raw: self.raw.clone() }
    }    
}

impl<Type> Puller<Type>
    where Type : Sync + Send
{
    pub fn new<RawPullerType>(raw_puller: RawPullerType) -> Puller<Type>
        where RawPullerType: RawPuller<Type> + Send + Sync + 'static
    {
        Puller {
            raw: Arc::new(Mutex::new(raw_puller))
        }
    }

    pub async fn pull(&mut self,value: Type) {
        let mut mutable_raw = self.raw.lock().await;
        mutable_raw.pull(value).await;
    }
}

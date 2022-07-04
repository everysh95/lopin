use async_trait::async_trait;
use std::marker::Send;
use std::ops::BitXor;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::pin::Pin;

pub struct Store<T: Clone + Send + Sync> {
    raw: Arc<Mutex<dyn RawStore<T> + Send + Sync>>,
}

impl<T: Clone + Send + Sync> Store<T> {
    pub fn new(raw: Arc<Mutex<dyn RawStore<T> + Send + Sync>>) -> Store<T> {
        Store { raw: raw }
    }
    pub async fn get(&self) -> Option<T> {
        let mut raw = self.raw.lock().await;
        raw.get().await
    }
    pub async fn put(&self, value: T) {
        let mut raw = self.raw.lock().await;
        raw.put(value).await
    }
    pub async fn put_and_get(&self,value: T)-> Option<T> {
        self.put(value).await;
        self.get().await
    }
    pub async fn get_and_put(&self) {
        if let Some(value) = self.get().await {
            self.put(value).await;
        }
    }
}

impl<T: Clone + Send + Sync>  Clone for Store<T> {
    fn clone(&self) -> Self {
        Store::new(self.raw.clone())
    }
}

#[async_trait]
pub trait RawStore<T: Clone + Send + Sync> {
    async fn get(&mut self) -> Option<T>;
    async fn put(&mut self, value: T);
}

#[async_trait]
pub trait Converter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    async fn to(&self, src: ST) -> Option<DT>;
    async fn from(&self, dist: DT) -> Option<ST>;
}

struct Convert<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    store: Store<ST>,
    convert: Arc<dyn Converter<ST, DT> + Send + Sync>,
}

#[async_trait]
impl<ST: Clone + Send + Sync, DT: Clone + Send + Sync> RawStore<DT> for Convert<ST, DT> {
    async fn get(&mut self) -> Option<DT> {
        let value = self.store.get().await;
        match value {
            Some(v) => self.convert.to(v).await,
            None => None,
        }
    }
    async fn put(&mut self, value: DT) {
        let put_value = self.convert.from(value.clone()).await;
        match put_value {
            Some(v) => self.store.put(v).await,
            None => {}
        }
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitXor<Arc<dyn Converter<ST, DT> + Send + Sync>> for Store<ST>
{
    type Output = Store<DT>;
    fn bitxor(self, rhs: Arc<dyn Converter<ST, DT> + Send + Sync>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new( Convert {
            store: self,
            convert: rhs,
        })));
    }
}

struct SimpleStore<T: Clone + Send + Sync> {
    data: T,
}

#[async_trait]
impl<T: Clone + Send + Sync> RawStore<T> for SimpleStore<T> {
    async fn get(&mut self) -> Option<T> {
        Some(self.data.clone())
    }
    async fn put(&mut self, value: T) {
        self.data = value.clone();
    }
}

impl<T: Clone + Send + Sync> SimpleStore<T> {
    fn new(data: T) -> SimpleStore<T> {
        SimpleStore { data: data }
    }
}

pub fn store<T: Clone + Send + Sync + 'static>(data: T) -> Store<T> {
    Store::new(Arc::new(Mutex::new(SimpleStore::new(data))))
}

use async_trait::async_trait;
use std::marker::Send;
use std::ops::BitXor;
use std::sync::Arc;

pub struct Store<T: Clone + Send + Sync> {
    raw: Arc<dyn RawStore<T> + Send + Sync>,
}

impl<T: Clone + Send + Sync> Store<T> {
    pub async fn get(&self) -> Option<T> {
        self.raw.get().await
    }
    pub async fn put(&mut self, value: T) {
        Arc::get_mut(&mut self.raw).unwrap().put(value).await
    }
    pub fn new(raw: Arc<dyn RawStore<T> + Send + Sync>) -> Store<T> {
        Store { raw: raw }
    }
}

#[async_trait]
pub trait RawStore<T: Clone + Send + Sync> {
    async fn get(&self) -> Option<T>;
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
    async fn get(&self) -> Option<DT> {
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
        return Store::new(Arc::new(Convert {
            store: self,
            convert: rhs,
        }));
    }
}

struct SimpleStore<T: Clone + Send + Sync> {
    data: T,
}

#[async_trait]
impl<T: Clone + Send + Sync> RawStore<T> for SimpleStore<T> {
    async fn get(&self) -> Option<T> {
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
    Store::new(Arc::new(SimpleStore::new(data)))
}

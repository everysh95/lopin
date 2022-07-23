use async_trait::async_trait;
use std::marker::Send;
use std::sync::Arc;
use tokio::sync::Mutex;

/// wrapped store
/// 
/// # Examples
///
/// ```
/// use lopin::{store, Store};
/// 
/// #[tokio::main]
/// async fn main() {
///     // create simple store
///     let my_store : Store<String> = store("text".to_string());
/// 
///     // get stored value
///     let text : Option<String> = my_store.get().await;
/// 
///     // store value into "store"
///     my_store.put("new text".to_string()).await;
/// }
/// ```
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

/// store trait
/// 
/// # Examples
///
/// ```
/// use lopin::{RawStore, Store};
/// use async_trait::async_trait;
/// use std::sync::Arc;
/// use tokio::sync::Mutex;
/// 
/// struct MyStore {
///     // fields
/// }
/// 
/// #[derive(Clone)]
/// struct SomeType {
///     // fields
/// }
/// 
/// #[async_trait]
/// impl RawStore<SomeType> for MyStore {
///     async fn get(&mut self) -> Option<SomeType> {
///         // request get to MyStore
///         None
///     }
///     async fn put(&mut self, value: SomeType) {
///         // request put to MyStore
///     }
/// }
/// 
/// fn my_store() -> Store<SomeType> {
///     Store::new(Arc::new(Mutex::new(MyStore {
///         // init field
///     })))
/// }
/// 
/// ```
#[async_trait]
pub trait RawStore<T: Clone + Send + Sync> {
    /// get request process
    async fn get(&mut self) -> Option<T>;
    /// put request process
    async fn put(&mut self, value: T);
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

struct TempStore<T: Clone + Send + Sync> {
    data: Option<T>,
}

#[async_trait]
impl<T: Clone + Send + Sync> RawStore<T> for TempStore<T> {
    async fn get(&mut self) -> Option<T> {
        self.data.clone()
    }
    async fn put(&mut self, value: T) {
        self.data = Some(value.clone());
    }
}

pub fn temporary<T: Clone + Send + Sync + 'static>() -> Store<T> {
    Store::new(Arc::new(Mutex::new(TempStore {
        data: None
    })))
}

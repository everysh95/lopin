use async_trait::async_trait;
use std::marker::Send;
use std::sync::Arc;
use tokio::sync::Mutex;

/// wrapped store
/// 
/// # Examples
///
/// ```
/// // create simple store
/// let my_store : Store<String> = store("text".to_string());
/// 
/// // get stored value
/// let text : String = my_store.get().await;
/// 
/// // store value into "store"
/// my_store.put("new text").await;
/// 
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
/// struct MyStore {
/// /** fields */
/// }
/// 
/// #[async_trait]
/// impl RawStore<Type> for MyStore {
///     async fn get(&mut self) -> Option<T> {
///         /** request get to MyStore */
///     }
///     async fn put(&mut self, value: T) {
///         /** request put to MyStore */
///     }
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

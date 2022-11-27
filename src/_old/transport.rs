use crate::core::Store;

pub async fn transport<T: 'static + Clone + Send + Sync>(from: Store<T>, to: Store<T>) {
    if let Some(value) = from.get().await {
        to.put(value).await;
    }
}

pub async fn swap<T: 'static + Clone + Send + Sync>(a: Store<T>, b: Store<T>) {
    match a.get().await {
        Some(value_a) => match b.get().await {
            Some(value_b) => {
                a.put(value_b).await;
                b.put(value_a).await;
            }
            None => {
                b.put(value_a).await;
            }
        },
        None => match b.get().await {
            Some(value_b) => {
                a.put(value_b).await;
            }
            None => {}
        },
    }
}

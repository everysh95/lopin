use crate::core::{Store, Transport};
use crate::propaty::Propaty;
use async_trait::async_trait;
use std::sync::Arc;

pub struct SimpleTransport {
    from: String,
    to: String,
}

#[async_trait]
impl Transport<Vec<Propaty<String>>> for SimpleTransport {
    async fn transport(&self, store: &mut Store<Vec<Propaty<String>>>) {
        match store.get().await {
            Some(value) => match value.iter().find(|p| p.key == self.from) {
                Some(v) => {
                    let result = match value.iter().find(|p| p.key == self.to) {
                        Some(_v_to) => value
                            .iter()
                            .cloned()
                            .map(|p| {
                                if p.key == self.to {
                                    Propaty {
                                        key: p.key,
                                        value: v.value.clone_arc(),
                                    }
                                } else {
                                    v.clone()
                                }
                            })
                            .collect(),
                        None => vec![
                            value.clone(),
                            vec![Propaty {
                                key: self.to.clone(),
                                value: v.value.clone_arc(),
                            }],
                        ]
                        .concat(),
                    };
                    store.put(result).await;
                }
                None => {}
            },
            None => {}
        }
    }
}

pub fn transport(from: &str, to: &str) -> Arc<dyn Transport<Vec<Propaty<String>>> + Send + Sync> {
    Arc::new(SimpleTransport {
        from: from.to_string(),
        to: to.to_string(),
    })
}

pub struct Swap {
    from: String,
    to: String,
}

#[async_trait]
impl Transport<Vec<Propaty<String>>> for Swap {
    async fn transport(&self, store: &mut Store<Vec<Propaty<String>>>) {
        if let Some(value) = store.get().await {
            let mut result: Vec<Propaty<String>> = value.clone();
            match value.iter().position(|p| p.key == self.from) {
                Some(index_form) => match value.iter().position(|p| p.key == self.to) {
                    Some(index_to) => {
                        result[index_form] = value[index_to].rename(&self.from);
                        result[index_to] = value[index_form].rename(&self.to);
                    }
                    None => result.push(value[index_form].rename(&self.to)),
                },
                None => match value.iter().position(|p| p.key == self.to) {
                    Some(index_to) => result.push(value[index_to].rename(&self.from)),
                    None => {}
                },
            }
            store.put(result).await;
        }
    }
}

pub fn swap(from: &str, to: &str) -> Arc<dyn Transport<Vec<Propaty<String>>> + Send + Sync> {
    Arc::new(Swap {
        from: from.to_string(),
        to: to.to_string(),
    })
}

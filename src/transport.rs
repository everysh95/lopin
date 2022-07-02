use crate::core::Store;
use crate::propaty::Propaty;

pub async fn transport(store: Store<Vec<Propaty<String>>>, from: &str, to: &str) {
    match store.get().await {
        Some(value) => match value.iter().find(|p| p.key == from.to_string()) {
            Some(v) => {
                let result = match value.iter().find(|p| p.key == to.to_string()) {
                    Some(_v_to) => value
                        .iter()
                        .cloned()
                        .map(|p| {
                            if p.key == to.to_string() {
                                v.clone().rename(&to.to_string())
                            } else {
                                v.clone()
                            }
                        })
                        .collect(),
                    None => {
                        vec![
                            value.clone(),
                            vec![v.clone().rename(&to.to_string())],
                        ]
                        .concat()
                    }
                };
                store.put(result).await;
            }
            None => {}
        },
        None => {}
    }
}

pub async fn swap(store: Store<Vec<Propaty<String>>>, from: &str, to: &str) {
    if let Some(value) = store.get().await {
        let mut result: Vec<Propaty<String>> = value.clone();
        match value.iter().position(|p| p.key == from.to_string()) {
            Some(index_form) => match value.iter().position(|p| p.key == to.to_string()) {
                Some(index_to) => {
                    result[index_form] = value[index_to].rename(&from.to_string());
                    result[index_to] = value[index_form].rename(&to.to_string());
                }
                None => result.push(value[index_form].rename(&to.to_string())),
            },
            None => match value.iter().position(|p| p.key == to.to_string()) {
                Some(index_to) => result.push(value[index_to].rename(&to.to_string())),
                None => {}
            },
        }
        store.put(result).await;
    }
}

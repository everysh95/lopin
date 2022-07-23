mod core;
pub mod http;

pub use self::core::{create, delete, read, update, Error};

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test::assert_eq_store;
    use crate::{store, unwarp_or, Propaty, Store};

    #[tokio::test]
    async fn it_create() {
        let pipe: Store<Vec<Propaty<String>>> = assert_eq_store::<Vec<Propaty<i32>>>(
            vec![
                Propaty::new(
                    -1,
                    Result::<Vec<Propaty<String>>, Error>::Ok(vec![Propaty::new(
                        String::from("id"),
                        0,
                    )]),
                ),
                Propaty::new(0, vec![Propaty::new(String::from("id"), 0)]),
            ],
            vec![],
        ) ^ create(String::from("id"), -1)
            ^ unwarp_or(vec![]);
        pipe.put(vec![Propaty::new(String::from("id"), 0)]).await;
    }
    #[tokio::test]
    async fn it_update() {
        let pipe: Store<Vec<Propaty<String>>> = assert_eq_store::<Vec<Propaty<i32>>>(
            vec![
                Propaty::new(
                    -1,
                    Result::<Vec<Propaty<String>>, Error>::Ok(vec![
                        Propaty::new(String::from("id"), 0),
                        Propaty::new(String::from("data"), 1),
                    ]),
                ),
                Propaty::new(
                    0,
                    vec![
                        Propaty::new(String::from("id"), 0),
                        Propaty::new(String::from("data"), 1),
                    ],
                ),
            ],
            vec![
                Propaty::new(-1, Option::<Vec<Propaty<String>>>::None),
                Propaty::new(
                    0,
                    vec![
                        Propaty::new(String::from("id"), 0),
                        Propaty::new(String::from("data"), 0),
                    ],
                ),
            ],
        ) ^ update(String::from("id"), -1)
            ^ unwarp_or(vec![]);
        pipe.put(vec![
            Propaty::new(String::from("id"), 0),
            Propaty::new(String::from("data"), 1),
        ])
        .await;
    }
    #[tokio::test]
    async fn it_read() {
        let pipe = store::<Vec<Propaty<i32>>>(
            vec![
                Propaty::new(
                    -1,
                    Option::<Vec<Propaty<String>>>::None,
                ),
                Propaty::new(0, vec![Propaty::new(String::from("id"), 0), Propaty::new(String::from("data"), 0)]),
            ],
        ) ^ read::<String,i32>(-1)
            ^ unwarp_or(vec![]);
        pipe.put(vec![vec![Propaty::new(String::from("id"), 0)]]).await;
        assert_eq!(pipe.get().await,Some(vec![vec![Propaty::new(String::from("id"), 0), Propaty::new(String::from("data"), 0)]]));
    }
}

//! # lopin - library of pipeline input
//!
//! `lopin` is a Web API framework featuring a two-way pipeline and resources abstracted as stores. 
//!

// main module
mod core;
mod merge;
mod convert;
mod propaty;
mod select;
mod transport;
mod multiop;
// addional module
pub mod http;
pub mod io;
pub mod json;
pub mod test;
pub mod crud;
// re-export
pub use self::core::{store, RawStore, Store};
pub use self::convert::{RawConverter, Converter, BroadcastConverter, put_only, get_only, dummy};
pub use self::propaty::{create_propaty, get_value, named, Propaty, PropatyMap, PropatyValue, unique_porpaty};
pub use self::select::{select, RawCondition, Condition};
pub use self::transport::{swap, transport};

#[cfg(test)]
mod tests {

    use super::test::{assert_eq_store, print_store};
    use super::*;

    #[tokio::test]
    async fn it_basic() {
        transport(
            store("hoge".to_string()) & select(&"hoge".to_string()),
            assert_eq_store("hoge".to_string(), "".to_string()),
        )
        .await;
    }
    #[tokio::test]
    async fn it_print() {
        transport(store("hoge".to_string()), print_store::<String>()).await;
    }
    #[tokio::test]
    async fn it_swap() {
        swap(
            assert_eq_store("b".to_string(), "a".to_string()),
            assert_eq_store("a".to_string(), "b".to_string()),
        )
        .await;
    }
    #[tokio::test]
    async fn it_prop() {
        let prop =
            create_propaty(store(10) ^ named("num") | store("text".to_string()) ^ named("text"))
                .await;
        transport(store(prop.clone()), assert_eq_store(prop.clone(), vec![])).await;
    }
}

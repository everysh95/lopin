// main module
mod core;
mod merge;
mod propaty;
mod select;
mod transport;
// addional module
pub mod http;
pub mod test;
pub mod json;
pub mod io;
// re-export
pub use self::core::{store, Store, RawStore, Converter};
pub use self::propaty::{create_propaty, get_value, named, Propaty, PropatyMap, PropatyValue};
pub use self::select::{Condition, select};
pub use self::transport::{swap, transport};

#[cfg(test)]
mod tests {

    use super::test::{assert_eq_store, print_store};
    use super::*;

    #[tokio::test]
    async fn it_basic() {
        let pipe = store("hoge".to_string()) & select(&"hoge".to_string()) ^ named("from")
            | assert_eq_store("hoge", "") ^ named("to");
        transport(pipe,"from", "to").await;
    }
    #[tokio::test]
    async fn it_print() {
        let pipe =
            store("hoge".to_string()) ^ named("from") | print_store::<String>() ^ named("to");
        transport(pipe,"from", "to").await;
    }
    #[tokio::test]
    async fn it_swap() {
        let pipe =
            store("hoge".to_string()) ^ named("from") | assert_eq_store("hoge", "") ^ named("to");
        transport(pipe,"from", "to").await;
    }
    #[tokio::test]
    async fn it_prop() {
        let prop =
            create_propaty(store(10) ^ named("num") | store("text".to_string()) ^ named("text"))
                .await;
        let pipe = store(prop.clone()) ^ named("from") | assert_eq_store(prop.clone(),vec![]) ^ named("to");
        transport(pipe,"from", "to").await;
    }

}

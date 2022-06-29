// main module
pub mod core;
pub mod merge;
pub mod propaty;
pub mod select;
pub mod transport;
// addional module
pub mod http;
pub mod test;
pub use self::core::*;
pub use self::merge::*;
pub use self::propaty::*;
pub use self::select::*;
pub use self::transport::*;

#[cfg(test)]
mod tests {

    use super::http::{http_store, to_utf8};
    use super::test::{assert_eq_store, print_store};
    use super::*;

    #[tokio::test]
    async fn it_basic() {
        let mut pipe = store("hoge".to_string()) & select(&"hoge".to_string()) ^ named("from")
            | assert_eq_store("hoge", "") ^ named("to");
        pipe.transport(transport("from", "to")).await;
    }
    #[tokio::test]
    async fn it_print() {
        let mut pipe =
            store("hoge".to_string()) ^ named("from") | print_store::<String>() ^ named("to");
        pipe.transport(transport("from", "to")).await;
    }
    #[tokio::test]
    async fn it_swap() {
        let mut pipe =
            store("hoge".to_string()) ^ named("from") | assert_eq_store("hoge", "") ^ named("to");
        pipe.transport(transport("from", "to")).await;
    }
    #[tokio::test]
    async fn it_prop() {
        let prop =
            create_propaty(store(10) ^ named("num") | store("text".to_string()) ^ named("text"))
                .await;
        let mut pipe = store(prop.clone()) ^ named("from") | assert_eq_store(prop.clone(),vec![]) ^ named("to");
        pipe.transport(transport("from", "to")).await;
    }

    #[tokio::test]
    async fn it_client() {
        let req = create_propaty(
            store("https://httpbin.org/get".to_string()) ^ named("uri")
                | store(5000) ^ named("timeout"),
        )
        .await;
        let mut pipe_cliant =
            print_store::<String>() ^ named("to") | http_store(req) ^ to_utf8() ^ named("from");
        pipe_cliant.transport(transport("from", "to")).await;
    }
}

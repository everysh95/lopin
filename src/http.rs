pub mod util;
pub mod server;
pub mod client;

pub use server::{bind_http, http_get, http_post, http_put, status_ok, status_created, status_bad_request, status_not_found, status_unauthorized};
pub use client::{http_store, HttpCliantStoreWithTimeOut};
pub use util::*;

#[cfg(test)]
mod tests {
    use crate::{create_propaty, store, named, transport};
    use crate::test::{print_store, assert_eq_store};
    use super::{http_store, to_utf8, from_utf8, bind_http, http_get, http_put, status_ok};
    use std::net::SocketAddr;

    #[tokio::test]
    async fn it_client() {
        let req = create_propaty(
            store("https://httpbin.org/get".to_string()) ^ named("uri")
                | store(5000) ^ named("timeout"),
        )
        .await;
        let pipe_cliant =
            print_store::<String>() ^ named("to") | http_store(req) ^ to_utf8() ^ named("from");
        transport(pipe_cliant,"from", "to").await;
    }

    #[tokio::test]
    async fn it_server() {
        let test_store = store("test".to_string());
        let pipe_server =
            test_store.clone() ^ from_utf8() ^ status_ok() ^ http_get() | test_store.clone() ^ from_utf8() ^ status_ok() ^ http_put();
        tokio::spawn(async {
            bind_http(pipe_server, SocketAddr::from(([127, 0, 0, 1], 3000))).await;
        });
        let req = create_propaty(
            store("http://127.0.0.1:3000".to_string()) ^ named("uri")
                | store(5000) ^ named("timeout"),
        )
        .await;
        let pipe_cliant =
            assert_eq_store("test".to_string(), "test".to_string()) ^ named("to") | http_store(req) ^ to_utf8() ^ named("from");
        transport(pipe_cliant.clone(), "to","from").await;
        transport(pipe_cliant.clone(),"from", "to").await;
    }

}
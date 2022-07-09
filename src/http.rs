pub mod client;
pub mod server;
pub mod util;

pub use client::{http_store, HttpCliantStoreWithTimeOut};
pub use server::{
    http_with, http_get, http_post, http_put, status_bad_request, status_created, status_not_found,
    status_ok, status_unauthorized,
};
pub use util::*;

#[cfg(test)]
mod tests {
    use super::{http_with, from_utf8, http_get, http_put, http_store, status_ok, to_utf8};
    use crate::json::to_json;
    use crate::test::assert_eq_store;
    use crate::{create_propaty, named, store, transport, put_only, get_only};

    #[tokio::test]
    async fn it_client() {
        let req = create_propaty(
            store("https://httpbin.org/get".to_string()) ^ named("uri")
                | store(5000) ^ named("timeout"),
        )
        .await;
        transport(
            http_store(req) ^ to_utf8(),
            assert_eq_store("https://httpbin.org/get".to_string(), "".to_string())
                ^ named("url")
                ^ to_json(),
        )
        .await;
    }

    #[tokio::test]
    async fn it_server() {
        let test_store = store("test".to_string());
        let pipe_server = (test_store ^ from_utf8() ^ status_ok()) & (http_get() ^ get_only() | http_put() ^ put_only());
        tokio::spawn(async {
            http_with("127.0.0.1:3000",pipe_server).await;
        });
        let req = create_propaty(
            store("http://127.0.0.1:3000".to_string()) ^ named("uri")
                | store(5000) ^ named("timeout"),
        )
        .await;
        let pipe_assert = assert_eq_store("test".to_string(), "test".to_string());
        let pipe_cliant = http_store(req) ^ to_utf8();
        transport(pipe_assert.clone(), pipe_cliant.clone()).await;
        transport(pipe_cliant.clone(), pipe_assert.clone()).await;
    }
}

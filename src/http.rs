pub mod util;
pub mod server;
pub mod client;

pub use server::{bind_http, http_get, http_post, http_put, status_ok, status_created, status_bad_request, status_not_found, status_unauthorized};
pub use client::{http_store, HttpCliantStoreWithTimeOut};
pub use util::*;

#[cfg(test)]
mod tests {
    use crate::{create_propaty, store, named, transport};
    use crate::test::{print_store};
    use super::{http_store, to_utf8};

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

}
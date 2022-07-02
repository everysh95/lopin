use crate::{Propaty, PropatyMap, RawStore, Store};
use async_trait::async_trait;
use hyper::body::{to_bytes, Bytes};
use hyper::client::Client;
use hyper::header::{HeaderMap, HeaderName, HeaderValue};
use hyper::{Body, Method, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct HttpCliantStoreWithTimeOut {
    uri: String,
    put_method: Option<Method>,
    headers: HeaderMap,
}

#[async_trait]
impl RawStore<Bytes> for HttpCliantStoreWithTimeOut {
    async fn get(&self) -> Option<Bytes> {
        if self.uri.clone().split_at(5).0 == "https".to_string() {
            let https = hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .build();
            let client = Client::builder().build::<_, hyper::Body>(https);
            let mut builder = Request::builder();
            for (key, value) in self.headers.iter() {
                builder = builder.header(key, value);
            }
            if let Ok(raw_resp) = client
                .request(
                    builder
                        .method(Method::GET)
                        .uri(self.uri.clone())
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
            {
                if let Ok(bytes) = to_bytes(raw_resp.into_body()).await {
                    return Some(bytes);
                }
            }
        } else {
            let client = Client::new();
            let mut builder = Request::builder();
            for (key, value) in self.headers.iter() {
                builder = builder.header(key, value);
            }
            if let Ok(raw_resp) = client
                .request(
                    builder
                        .method(Method::GET)
                        .uri(self.uri.clone())
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
            {
                if let Ok(bytes) = to_bytes(raw_resp.into_body()).await {
                    return Some(bytes);
                }
            }
        }
        None
    }
    async fn put(&mut self, value: Bytes) {
        if let Some(put_method) = self.put_method.clone() {
            let client = Client::new();
            let mut builder = Request::builder();
            for (key, value) in self.headers.iter() {
                builder = builder.header(key, value);
            }
            client
                .request(
                    builder
                        .method(put_method)
                        .uri(self.uri.clone())
                        .body(Body::from(value.clone()))
                        .unwrap(),
                )
                .await
                .unwrap();
        }
    }
}

pub fn http_store(config: Vec<Propaty<String>>) -> Store<Bytes> {
    let uri: String = match config.get_value::<String>(&"uri".to_string()) {
        Some(v) => v,
        None => "".to_string(),
    };
    let put_method: Option<Method> = config.get_value::<Method>(&"put_method".to_string());
    let headers: Vec<Propaty<String>> =
        match config.get_value::<Vec<Propaty<String>>>(&"headers".to_string()) {
            Some(v) => v,
            None => vec![],
        };
    let mut raw_headers = HeaderMap::new();
    for header in headers.iter() {
        let key = header.key.clone();
        if let Ok(header_key) = HeaderName::from_bytes(&key.as_bytes()) {
            if let Some(value) = headers.get_value::<String>(&key) {
                if let Ok(header_value) = HeaderValue::from_str(&value.clone()) {
                    raw_headers.insert(header_key, header_value);
                }
            }
        }
    }

    Store::new(Arc::new(Mutex::new(HttpCliantStoreWithTimeOut {
        uri: uri.to_string(),
        put_method: put_method,
        headers: HeaderMap::new(),
    })))
}

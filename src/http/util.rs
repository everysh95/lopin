use crate::{RawConverter, Converter, temporary, Propaty, named, Store};
use async_trait::async_trait;
use hyper::body::Bytes;
use std::sync::Arc;

use super::HttpData;

pub struct FromUtf8 {}

#[async_trait]
impl RawConverter<String, Bytes> for FromUtf8 {
    async fn to(&self, src: String) -> Option<Bytes> {
        Some(Bytes::from(src.clone()))
    }
    async fn from(&self, _src: Option<String>, dist: Bytes) -> Option<String> {
        if let Ok(res) = String::from_utf8(dist.to_vec()) {
            Some(res)
        } else {
            None
        }
    }
}

pub fn from_utf8() -> Converter<String, Bytes> {
    Converter::new(Arc::new(FromUtf8 {}))
}

pub struct ToUtf8 {}

#[async_trait]
impl RawConverter<Bytes, String> for ToUtf8 {
    async fn to(&self, src: Bytes) -> Option<String> {
        if let Ok(res) = String::from_utf8(src.to_vec()) {
            Some(res)
        } else {
            None
        }
    }
    async fn from(&self, _src: Option<Bytes>, dist: String) -> Option<Bytes> {
        Some(Bytes::from(dist.clone()))
    }
}

pub fn to_utf8() -> Converter<Bytes, String> {
    Converter::new(Arc::new(ToUtf8 {}))
}

pub fn temporary_header(name: &str) -> Store<Vec<Propaty<String>>> {
    temporary::<HttpData>() ^ named(name)
}
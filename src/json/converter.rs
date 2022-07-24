use crate::{RawConverter, Converter, Propaty};
use super::core::Record;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::sync::Arc;

struct ToRecord;

#[async_trait]
impl RawConverter<Vec<Propaty<String>>, Record> for ToRecord{
    async fn to(&self, src: Vec<Propaty<String>>) -> Option<Record> {
        Some(Record::new(src))
    }
    async fn from(&self, _old: Option<Vec<Propaty<String>>>, dist: Record) -> Option<Vec<Propaty<String>>> {
        Some(dist.props)
    }
}

pub fn to_record() -> Converter<Vec<Propaty<String>>, Record> {
    Converter::new(Arc::new(ToRecord))
}

struct ToJson;

#[async_trait]
impl<'de,T : serde::Serialize + DeserializeOwned + Clone + Send + Sync + 'static> RawConverter<T, String> for ToJson{
    async fn to(&self, src: T) -> Option<String> {
        match serde_json::to_string(&src) {
            Ok(json_str) => Some(json_str),
            Err(_) => None
        }
    }
    async fn from(&self, _old: Option<T>, dist:String) -> Option<T> {
        match serde_json::from_str::<T>(dist.as_str()) {
            Ok(src) => Some(src),
            Err(_) => None
        }
    }
}

pub fn to_json<T : serde::Serialize + DeserializeOwned + Clone + Send + Sync + 'static>() -> Converter<T, String> {
    Converter::new(Arc::new(ToJson))
}

struct FromRecord;

#[async_trait]
impl RawConverter<Record,Vec<Propaty<String>>> for FromRecord{
    async fn to(&self, dist: Record) -> Option<Vec<Propaty<String>>> {
        Some(dist.props)
    }
    async fn from(&self, _old: Option<Record>, src: Vec<Propaty<String>>) -> Option<Record> {
        Some(Record::new(src))
    }
}

pub fn from_record() -> Converter<Record,Vec<Propaty<String>>> {
    Converter::new(Arc::new(FromRecord))
}

struct FromJson;

#[async_trait]
impl<T : serde::Serialize + DeserializeOwned + Clone + Send + Sync + 'static> RawConverter<String,T> for FromJson{
    async fn to(&self, dist: String) -> Option<T> {
        match serde_json::from_str::<T>(dist.as_str()) {
            Ok(record) => Some(record),
            Err(_) => None
        }
    }
    async fn from(&self, _old: Option<String>, src: T) -> Option<String> {
        match serde_json::to_string(&src) {
            Ok(json_str) => Some(json_str),
            Err(_) => None
        }
    }
}

pub fn from_json<T : serde::Serialize + DeserializeOwned + Clone + Send + Sync + 'static>() -> Converter<String,T> {
    Converter::new(Arc::new(FromJson))
}
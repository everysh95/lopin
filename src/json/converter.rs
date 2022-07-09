use crate::{RawConverter, Converter, Propaty};
use super::core::Record;
use async_trait::async_trait;
use std::sync::Arc;

struct ToJson;

#[async_trait]
impl RawConverter<Vec<Propaty<String>>, String> for ToJson{
    async fn to(&self, src: Vec<Propaty<String>>) -> Option<String> {
        match serde_json::to_string(&Record::new(src)) {
            Ok(json_str) => Some(json_str),
            Err(_) => None
        }
    }
    async fn from(&self, dist: String) -> Option<Vec<Propaty<String>>> {
        match serde_json::from_str::<Record>(&dist) {
            Ok(record) => Some(record.props),
            Err(_) => None
        }
    }
}

pub fn to_json() -> Converter<Vec<Propaty<String>>, String> {
    Converter::new(Arc::new(ToJson))
}

struct FromJson;

#[async_trait]
impl RawConverter<String,Vec<Propaty<String>>> for FromJson{
    async fn to(&self, dist: String) -> Option<Vec<Propaty<String>>> {
        match serde_json::from_str::<Record>(&dist) {
            Ok(record) => Some(record.props),
            Err(_) => None
        }
    }
    async fn from(&self, src: Vec<Propaty<String>>) -> Option<String> {
        match serde_json::to_string(&Record::new(src)) {
            Ok(json_str) => Some(json_str),
            Err(_) => None
        }
    }
}

pub fn from_json() -> Converter<String,Vec<Propaty<String>>> {
    Converter::new(Arc::new(FromJson))
}
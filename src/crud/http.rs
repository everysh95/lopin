use crate::crud::Error;
use crate::http::{HttpData, StatusCode};
use crate::{Converter, Propaty, PropatyMap, RawConverter};
use async_trait::async_trait;
use std::sync::Arc;

struct ErrorHandle {
    error: Error,
    status: StatusCode,
    header_selecter: String,
    data_selecter: String,
}

#[async_trait]
impl RawConverter<Vec<Propaty<String>>, HttpData> for ErrorHandle {
    async fn to(&self, value: Vec<Propaty<String>>) -> Option<HttpData> {
        match value.clone().get_value::<HttpData>(&self.header_selecter) {
            Some(header) => match value.get_value::<Error>(&self.data_selecter) {
                Some(value) => {
                    if value.clone() == self.error {
                        Some(HttpData {
                            uri: header.uri.clone(),
                            method: header.method.clone(),
                            data: Some(hyper::body::Bytes::from(vec![])),
                            code: Some(self.status.clone()),
                        })
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        }
    }
    async fn from(
        &self,
        _old: Option<Vec<Propaty<String>>>,
        _value: HttpData,
    ) -> Option<Vec<Propaty<String>>> {
        None
    }
}

pub fn error_handle(
    header_selecter: &str,
    data_selecter: &str,
    error: Error,
    status: StatusCode,
) -> Converter<Vec<Propaty<String>>, HttpData> {
    Converter::new(Arc::new(ErrorHandle {
        error,
        status,
        header_selecter: header_selecter.to_string(),
        data_selecter: data_selecter.to_string(),
    }))
}

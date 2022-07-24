use crate::json::{from_json, to_json, from_record, Record, to_record};
use crate::{store, Converter, Propaty, RawConverter, Store, PropatyMap, unwarp_or};
use async_trait::async_trait;
use hyper::body::{to_bytes, Bytes};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, Uri};
pub use hyper::StatusCode;
use std::convert::Infallible;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use super::{from_utf8, to_utf8};

#[derive(Clone, Debug, PartialEq)]
pub struct HttpData {
    pub method: Method,
    pub code: Option<StatusCode>,
    pub uri: Uri,
    pub data: Option<Bytes>,
}

impl HttpData {
    pub fn new(method: Method, uri: Uri, data: Option<Bytes>, code: Option<StatusCode>) -> HttpData {
        return HttpData {
            code,
            method,
            uri,
            data,
        };
    }
}

#[derive(Clone)]
struct StoreContext {
    store: Store<Vec<HttpData>>,
}

impl StoreContext {
    async fn proc_request(
        &mut self,
        req: Request<Body>,
    ) -> Result<Response<Body>, hyper::http::Error> {
        let method = req.method().clone();
        let uri = req.uri().clone();
        if let Ok(bytes) = to_bytes(req.into_body()).await {
            match self
                .store
                .put_and_get(vec![HttpData::new(
                    method.clone(),
                    uri.clone(),
                    Some(bytes),
                    None,
                )])
                .await
            {
                Some(res_props) => match res_props
                    .iter()
                    .find(|d| d.method.clone() == method.clone() && d.uri.clone() == uri.clone())
                {
                    Some(res) => Response::builder()
                        .status(match res.code {
                            Some(code) => code,
                            None => StatusCode::INTERNAL_SERVER_ERROR,
                        })
                        .body(match res.data.clone() {
                            Some(data) => data.into(),
                            None => Body::empty(),
                        }),
                    None => Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(Body::empty()),
                },
                None => {
                    return Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(Body::empty())
                }
            }
        } else {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
        }
    }
}

async fn stores_http_handler(
    context: StoreContext,
    req: Request<Body>,
) -> Result<Response<Body>, hyper::http::Error> {
    let mut context = context;
    context.proc_request(req).await
}

pub async fn http_with(address: &str, store: Store<Vec<HttpData>>) {
    let context = StoreContext { store };
    let raw_address: SocketAddr = address
        .parse::<SocketAddr>()
        .unwrap_or(SocketAddr::from(([0, 0, 0, 0], 8080)));
    let make_service = make_service_fn(move |_conn: &AddrStream| {
        let context = context.clone();
        let service = service_fn(move |req| stores_http_handler(context.clone(), req));
        async move { Ok::<_, Infallible>(service) }
    });
    let server = Server::bind(&raw_address).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

struct FilterMehod {
    method: Method,
    allow_get: bool,
    allow_put: bool,
}

#[async_trait]
impl RawConverter<HttpData, HttpData> for FilterMehod {
    async fn to(&self, src: HttpData) -> Option<HttpData> {
        if src.method.clone() == self.method.clone() {
            let mut res = src;
            if !self.allow_get {
                res.data = None
            }
            Some(res)
        } else {
            None
        }
    }
    async fn from(&self, _old: Option<HttpData>, dist: HttpData) -> Option<HttpData> {
        if dist.method.clone() == self.method.clone() {
            let mut res = dist;
            if !self.allow_put {
                res.data = None
            }
            Some(res)
        } else {
            None
        }
    }
}

pub fn method(
    ref_method: Method,
    allow_get: bool,
    allow_put: bool,
) -> Converter<HttpData, HttpData> {
    Converter::new(Arc::new(FilterMehod {
        method: ref_method.clone(),
        allow_get,
        allow_put,
    }))
}

pub fn http_get(allow_get: bool, allow_put: bool) -> Converter<HttpData, HttpData> {
    method(Method::GET, allow_get, allow_put)
}

pub fn http_put(allow_get: bool, allow_put: bool) -> Converter<HttpData, HttpData> {
    method(Method::PUT, allow_get, allow_put)
}

pub fn http_post(allow_get: bool, allow_put: bool) -> Converter<HttpData, HttpData> {
    method(Method::POST, allow_get, allow_put)
}

struct SetStatus {
    code: StatusCode,
}

#[async_trait]
impl RawConverter<HttpData, HttpData> for SetStatus {
    async fn to(&self, src: HttpData) -> Option<HttpData> {
        let mut src = src;
        src.code = Some(self.code.clone());
        Some(src)
    }
    async fn from(&self, _old: Option<HttpData>, dist: HttpData) -> Option<HttpData> {
        Some(dist)
    }
}

pub fn status(code: StatusCode) -> Converter<HttpData, HttpData> {
    Converter::new(Arc::new(SetStatus { code }))
}

pub fn status_ok() -> Converter<HttpData, HttpData> {
    status(StatusCode::OK)
}

pub fn status_created() -> Converter<HttpData, HttpData> {
    status(StatusCode::CREATED)
}

pub fn status_bad_request() -> Converter<HttpData, HttpData> {
    status(StatusCode::BAD_REQUEST)
}

pub fn status_unauthorized() -> Converter<HttpData, HttpData> {
    status(StatusCode::UNAUTHORIZED)
}

pub fn status_not_found() -> Converter<HttpData, HttpData> {
    status(StatusCode::NOT_FOUND)
}

struct ToHttpData {
    header_selecter: String,
    data_selecter: String,
}

#[async_trait]
impl RawConverter<Vec<Propaty<String>>,HttpData> for ToHttpData {
    async fn to(&self, value: Vec<Propaty<String>>) -> Option<HttpData> {
        match value.clone().get_value::<HttpData>(&self.header_selecter) {
            Some(header) => match value.get_value::<Bytes>(&self.data_selecter) {
                Some(value) => 
                    Some(HttpData {
                        uri: header.uri.clone(),
                        method: header.method.clone(),
                        data: Some(value),
                        code: None,
                    }),
                None => None,
            },
            None => None,
        }
    }
    async fn from(&self,_old: Option<Vec<Propaty<String>>>, value: HttpData) -> Option<Vec<Propaty<String>>> {
        match value.clone().data {
            Some(data) => Some(vec![Propaty::new(self.header_selecter.clone(), value),Propaty::new(self.data_selecter.clone(), data)]),
            None => Some(vec![Propaty::new(self.header_selecter.clone(), value)])
        }
    }
}

pub fn to_http_data(header_selecter: &str,data_selecter: &str) -> Converter<Vec<Propaty<String>>,HttpData> {
    Converter::new(Arc::new(ToHttpData {
        header_selecter: header_selecter.to_string(),
        data_selecter: data_selecter.to_string(),
    }))
}

struct FillFromParam {
    params: Vec<String>,
}

#[async_trait]
impl RawConverter<HttpData, HttpData> for FillFromParam {
    async fn to(&self, src: HttpData) -> Option<HttpData> {
        Some(src)
    }
    async fn from(&self, _old: Option<HttpData>, dist: HttpData) -> Option<HttpData> {
        let mut dist = dist;
        if let Some(query) = dist.clone().uri.query() {
            let query_props: Vec<Propaty<String>> = serde_urlencoded::de::from_str::<Vec<(String,String)>>(query)
                .unwrap_or(vec![])
                .iter()
                .map(|p| {
                    Propaty::new(p.0.clone(), p.1.clone())
                })
                .filter(|p| self.params.clone().contains(&p.key))
                .collect();
            dist.data = ((store(dist.data.clone().unwrap_or(Bytes::from(b"{}".to_vec()))) ^ to_utf8() ^ from_json::<Record>() ^ from_record()
                | store(query_props))
                ^ to_record()
                ^ to_json()
                ^ from_utf8())
            .get()
            .await;
        }
        Some(dist)
    }
}

pub fn from_param(params: Vec<&str>) -> Converter<HttpData, HttpData> {
    Converter::new(Arc::new(FillFromParam {
        params: params.iter().map(|p| p.to_string()).collect(),
    }))
}

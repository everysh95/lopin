use crate::{Converter, RawConverter, RawStore, Store};
use async_trait::async_trait;
use hyper::body::{to_bytes, Bytes};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Uri};
use std::convert::Infallible;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, PartialEq)]
pub struct HttpData {
    method: Method,
    code: Option<StatusCode>,
    uri: Uri,
    data: Option<Bytes>,
}

impl HttpData {
    fn new(method: Method, uri: Uri, data: Option<Bytes>, code: Option<StatusCode>) -> HttpData {
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
    async fn from(&self, dist: HttpData) -> Option<HttpData> {
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

pub fn method(ref_method: Method,allow_get: bool, allow_put: bool) -> Converter<HttpData, HttpData> {
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
    async fn from(&self, dist: HttpData) -> Option<HttpData> {
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

struct BindWrap {
    uri: Option<Uri>,
    method: Option<Method>,
    store: Store<Bytes>,
}

#[async_trait]
impl RawStore<HttpData> for BindWrap {
    async fn get(&mut self) -> Option<HttpData> {
        match self.store.get().await {
            Some(value) => match &self.uri {
                Some(uri) => match &self.method {
                    Some(method) => Some(HttpData {
                        uri: uri.clone(),
                        method: method.clone(),
                        data: Some(value),
                        code: None,
                    }),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
    async fn put(&mut self, value: HttpData) {
        self.uri = Some(value.uri.clone());
        self.method = Some(value.method.clone());
        if let Some(data) = value.data {
            self.store.put(data).await;
        }
    }
}

pub fn http_data_bind(store: Store<Bytes>) -> Store<HttpData> {
    Store::new(Arc::new(Mutex::new(BindWrap {
        uri: None,
        method: None,
        store,
    })))
}

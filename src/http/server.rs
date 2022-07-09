use crate::{Store, RawConverter, RawCondition, Condition, Converter};
use hyper::body::{to_bytes, Bytes};
use async_trait::async_trait;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Uri};
use std::net::SocketAddr;
use hyper::server::conn::AddrStream;
use std::convert::Infallible;
use std::sync::Arc;
use std::fmt::Debug;
use std::sync::Mutex;

#[derive(Clone, Debug, PartialEq)]
pub struct HttpData {
    method: Method,
    code: Option<StatusCode>,
    uri: Uri,
    data: Bytes
}

impl HttpData {
    fn new(method: Method, uri: Uri, data: Bytes, code: Option<StatusCode>) -> HttpData{
        return HttpData {
            code,
            method,
            uri,
            data
        };
    }
}

#[derive(Clone)]
struct StoreContext {
    store: Store<Vec<HttpData>>
}

impl StoreContext {
    async fn proc_request(&mut self,req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
        let method = req.method().clone();
        let uri = req.uri().clone();
        if let Ok(bytes) = to_bytes(req.into_body()).await {
            match self.store.put_and_get(vec![HttpData::new(method.clone(),uri.clone(), bytes, None)]).await {
                Some(res_props) =>  match res_props.iter().find(|d| d.method.clone() == method.clone() && d.uri.clone() == uri.clone() ) {
                    Some(res) => Response::builder().status( match res.code {
                        Some(code) => code,
                        None => StatusCode::INTERNAL_SERVER_ERROR
                    }).body(res.data.clone().into()),
                    None => Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(Body::empty())
                }
                None => return
                    Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(Body::empty())
            }
        } else {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
        }
    }
}


async fn stores_http_handler(context: StoreContext, req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    let mut context = context;
    context.proc_request(req).await
}


pub async fn http_with(address: &str,store: Store<Vec<HttpData>>) {
    let context = StoreContext {
        store
    };
    let raw_address: SocketAddr = address.parse::<SocketAddr>().unwrap_or(SocketAddr::from(([0,0,0,0], 8080)));
    let make_service = make_service_fn(move |_conn: &AddrStream| {
        let context = context.clone();
        let service = service_fn(move |req| {
            stores_http_handler(context.clone(), req)
        });
        async move { Ok::<_, Infallible>(service) }
    });
    let server = Server::bind(&raw_address).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

struct FilterMehod {
    method: Method
}

#[async_trait]
impl RawCondition<HttpData> for FilterMehod
{
    async fn validation(&self, src: HttpData) -> bool {
        src.method.clone() == self.method.clone()
    }
}

pub fn method(ref_method: Method) -> Condition<HttpData> {
    Condition::new(Arc::new(FilterMehod {
        method: ref_method.clone(),
    }))
}

pub fn http_get() -> Condition<HttpData> {
    method(Method::GET)
}

pub fn http_put() -> Condition<HttpData> {
    method(Method::PUT)
}

pub fn http_post() -> Condition<HttpData> {
    method(Method::POST)
}


struct SetStatus {
    code: StatusCode
}

#[async_trait]
impl RawConverter<Bytes, HttpData> for SetStatus
{
    async fn to(&self, src: Bytes) -> Option<HttpData> {
        // Some(HttpData {code:self.code.clone(),data:src, method: todo!(), uri: todo!() })
        None
    }
    async fn from(&self, dist: HttpData) -> Option<Bytes> {
        Some(dist.data)
    }
}

pub fn status(code: StatusCode) -> Converter<Bytes ,HttpData> {
    Converter::new(Arc::new(SetStatus {
        code: code,
    }))
}

pub fn status_ok() -> Converter<Bytes ,HttpData> {
    status(StatusCode::OK)
}

pub fn status_created() -> Converter<Bytes ,HttpData> {
    status(StatusCode::CREATED)
}

pub fn status_bad_request() -> Converter<Bytes ,HttpData> {
    status(StatusCode::BAD_REQUEST)
}

pub fn status_unauthorized() -> Converter<Bytes ,HttpData> {
    status(StatusCode::UNAUTHORIZED)
}

pub fn status_not_found() -> Converter<Bytes ,HttpData> {
    status(StatusCode::NOT_FOUND)
}
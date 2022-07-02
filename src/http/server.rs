use crate::{Store, Propaty, PropatyMap, Converter};
use hyper::body::{to_bytes, Bytes};
use async_trait::async_trait;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;
use hyper::server::conn::AddrStream;
use std::convert::Infallible;
use std::sync::Arc;
use std::fmt::Debug;
use std::sync::Mutex;

#[derive(Clone, Debug, PartialEq)]
pub struct CodeAndBody {
    code: StatusCode,
    data: Bytes
}

#[derive(Clone)]
struct StoreContext {
    //stores: Vec<MethodAndStore>
    store: Store<Vec<Propaty<Method>>>
}

impl StoreContext {
    async fn proc_request(&mut self,req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
        let method = req.method().clone();
        if let Ok(bytes) = to_bytes(req.into_body()).await {
            match self.store.put_and_get(vec![Propaty::new(method.clone(), bytes)]).await {
                Some(res_props) =>  match res_props.get_value::<CodeAndBody>(&method) {
                    Some(res) => Response::builder().status(res.code).body(res.data.into()),
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
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
        }
    }
}


async fn stores_http_handler(context: StoreContext, req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    let mut context = context;
    context.proc_request(req).await
}


pub async fn bind_http(store: Store<Vec<Propaty<Method>>>, address: SocketAddr) {
    let context = StoreContext {
        store: store
    };
    let make_service = make_service_fn(move |_conn: &AddrStream| {
        let context = context.clone();
        let service = service_fn(move |req| {
            stores_http_handler(context.clone(), req)
        });
        async move { Ok::<_, Infallible>(service) }
    });
    let server = Server::bind(&address).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

struct SetMehod {
    method: Method
}

#[async_trait]
impl Converter<CodeAndBody, Vec<Propaty<Method>>> for SetMehod
{
    async fn to(&self, src: CodeAndBody) -> Option<Vec<Propaty<Method>>> {
        Some(vec![Propaty {
            key: self.method.clone(),
            value: Arc::new(Mutex::new(src)),
        }])
    }
    async fn from(&self, dist: Vec<Propaty<Method>>) -> Option<CodeAndBody> {
        dist.get_value(&self.method)
    }
}

pub fn method(ref_method: Method) -> Arc<dyn Converter<CodeAndBody, Vec<Propaty<Method>>> + Send + Sync> {
    Arc::new(SetMehod {
        method: ref_method.clone(),
    })
}

pub fn http_get() -> Arc<dyn Converter<CodeAndBody, Vec<Propaty<Method>>> + Send + Sync> {
    method(Method::GET)
}

pub fn http_put() -> Arc<dyn Converter<CodeAndBody, Vec<Propaty<Method>>> + Send + Sync> {
    method(Method::PUT)
}

pub fn http_post() -> Arc<dyn Converter<CodeAndBody, Vec<Propaty<Method>>> + Send + Sync> {
    method(Method::POST)
}


struct SetStatus {
    code: StatusCode
}

#[async_trait]
impl Converter<Bytes, CodeAndBody> for SetStatus
{
    async fn to(&self, src: Bytes) -> Option<CodeAndBody> {
        Some(CodeAndBody {
            code: self.code.clone(),
            data: src
        })
    }
    async fn from(&self, dist: CodeAndBody) -> Option<Bytes> {
        Some(dist.data)
    }
}

pub fn status(code: StatusCode) -> Arc<dyn Converter<Bytes ,CodeAndBody> + Send + Sync> {
    Arc::new(SetStatus {
        code: code,
    })
}

pub fn status_ok() -> Arc<dyn Converter<Bytes ,CodeAndBody> + Send + Sync> {
    status(StatusCode::OK)
}

pub fn status_created() -> Arc<dyn Converter<Bytes ,CodeAndBody> + Send + Sync> {
    status(StatusCode::CREATED)
}

pub fn status_bad_request() -> Arc<dyn Converter<Bytes ,CodeAndBody> + Send + Sync> {
    status(StatusCode::BAD_REQUEST)
}

pub fn status_unauthorized() -> Arc<dyn Converter<Bytes ,CodeAndBody> + Send + Sync> {
    status(StatusCode::UNAUTHORIZED)
}

pub fn status_not_found() -> Arc<dyn Converter<Bytes ,CodeAndBody> + Send + Sync> {
    status(StatusCode::NOT_FOUND)
}
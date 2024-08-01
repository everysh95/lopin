use std::convert::Infallible;

use crate::{filter, pipeline, AsyncFramework, AsyncPipeline, Framework, Pipeline, RawAsyncFramework, RawAsyncPipeline};
use http_body_util::{BodyExt, Full};
use hyper::{body::{Bytes, Incoming}, server::conn::http1, service::service_fn, Method, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use async_trait::async_trait;

struct HttpServer {
  address: String
}

#[async_trait]
impl RawAsyncFramework<Request<Incoming>,Response<Full<Bytes>>,Response<Full<Bytes>>> for HttpServer {
  async fn run(&self, pipeline: Pipeline<Request<Incoming>,Response<Full<Bytes>>,Response<Full<Bytes>>>) {
    let listener = TcpListener::bind(&self.address).await.unwrap();
    loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);
      if let Err(err) = http1::Builder::new()
          .serve_connection(io, service_fn(|req| {
            async {
              Ok::<_,Infallible>(match Ok(req) & pipeline.clone() {
                Ok(a) => a.clone(),
                Err(a) => a.clone(),
              })
            }
          }))
          .await
      {
          println!("Error serving connection: {:?}", err);
      }
    }
  }
}

pub fn http_server(address: &str) -> Framework<Request<Incoming>,Response<Full<Bytes>>,Response<Full<Bytes>>> {
  AsyncFramework::new(HttpServer {
    address: address.to_string()
  })
}

pub fn method_is(method: Method) -> Pipeline<Request<Incoming>, Request<Incoming>, Response<Full<Bytes>>>{
  filter(move|r : &Request<Incoming>| r.method() == &method, Response::builder().status(405).body(Full::new(Bytes::from(""))).unwrap())
}

pub fn http_get() -> Pipeline<Request<Incoming>, Request<Incoming>, Response<Full<Bytes>>> {
  method_is(Method::GET)
}

pub fn http_post() -> Pipeline<Request<Incoming>, Request<Incoming>, Response<Full<Bytes>>> {
  method_is(Method::POST)
}

pub fn http_put() -> Pipeline<Request<Incoming>, Request<Incoming>, Response<Full<Bytes>>> {
  method_is(Method::PUT)
}

pub fn http_delete() -> Pipeline<Request<Incoming>, Request<Incoming>, Response<Full<Bytes>>> {
  method_is(Method::DELETE)
}

struct FromBody;

#[async_trait]
impl RawAsyncPipeline<Request<Incoming>, String, Response<Full<Bytes>>> for FromBody {
  async fn run(&self,r: Request<Incoming>) -> Result<String, Response<Full<Bytes>>> {
    match r.into_body().collect().await {
      Ok(r) => match String::from_utf8(r.to_bytes().to_vec()) {
        Ok(s) => Ok(s),
        Err(e) => Err(Response::builder().status(400).body(Full::new(Bytes::from(e.to_string()))).unwrap()),
      },
      Err(_) => Err(Response::builder().status(400).body(Full::new(Bytes::from(""))).unwrap()),
    }
  }
}

pub fn from_body() -> Pipeline<Request<Incoming>, String, Response<Full<Bytes>>> {
  AsyncPipeline::new(FromBody)
}

pub fn to_body() -> Pipeline<String, Response<Full<Bytes>>, Response<Full<Bytes>>> {
  pipeline(|s| Ok(Response::builder().status(200).body(Full::new(Bytes::from(s))).unwrap()))
}
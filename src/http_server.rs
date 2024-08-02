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
impl RawAsyncFramework<Request<Bytes>,Response<Full<Bytes>>,Response<Full<Bytes>>> for HttpServer {
  async fn run(&self, pipeline: Pipeline<Request<Bytes>,Response<Full<Bytes>>,Response<Full<Bytes>>>) {
    let listener = TcpListener::bind(&self.address).await.unwrap();
    loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);
      if let Err(err) = http1::Builder::new()
          .serve_connection(io, service_fn(|req| {
            async {
              Ok::<_,Infallible>(match Ok(req) & to_bytes() & pipeline.clone() {
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

pub fn http_server(address: &str) -> Framework<Request<Bytes>,Response<Full<Bytes>>,Response<Full<Bytes>>> {
  AsyncFramework::new(HttpServer {
    address: address.to_string()
  })
}

pub fn method_is<T: Clone + 'static>(method: Method) -> Pipeline<Request<T>, Request<T>, Response<Full<Bytes>>>{
  filter(move|r : &Request<T>| r.method() == &method, Response::builder().status(405).body(Full::new(Bytes::from(""))).unwrap())
}

pub fn http_get<T: Clone + 'static>() -> Pipeline<Request<T>, Request<T>, Response<Full<Bytes>>> {
  method_is::<T>(Method::GET)
}

pub fn http_post<T: Clone + 'static>() -> Pipeline<Request<T>, Request<T>, Response<Full<Bytes>>> {
  method_is::<T>(Method::POST)
}

pub fn http_put<T: Clone + 'static>() -> Pipeline<Request<T>, Request<T>, Response<Full<Bytes>>> {
  method_is::<T>(Method::PUT)
}

pub fn http_delete<T: Clone + 'static>() -> Pipeline<Request<T>, Request<T>, Response<Full<Bytes>>> {
  method_is::<T>(Method::DELETE)
}

struct ToByte;

#[async_trait]
impl RawAsyncPipeline<Request<Incoming>, Request<Bytes>, Response<Full<Bytes>>> for ToByte {
  async fn run(&self,r: Request<Incoming>) -> Result<Request<Bytes>, Response<Full<Bytes>>> {
    match r.into_body().collect().await {
      Ok(r) => Ok(Request::new(r.to_bytes())),
      Err(_) => Err(Response::builder().status(400).body(Full::new(Bytes::from(""))).unwrap()),
    }
  }
}

pub fn to_bytes() -> Pipeline<Request<Incoming>,Request<Bytes>, Response<Full<Bytes>>> {
  AsyncPipeline::new(ToByte)
}

struct FromBody;

#[async_trait]
impl RawAsyncPipeline<Request<Bytes>, String, Response<Full<Bytes>>> for FromBody {
  async fn run(&self,r: Request<Bytes>) -> Result<String, Response<Full<Bytes>>> {
    match String::from_utf8(r.into_body().to_vec()) {
      Ok(s) => Ok(s),
      Err(e) => Err(Response::builder().status(400).body(Full::new(Bytes::from(e.to_string()))).unwrap()),
    }
  }
}

pub fn from_body() -> Pipeline<Request<Bytes>, String, Response<Full<Bytes>>> {
  AsyncPipeline::new(FromBody)
}

pub fn to_body() -> Pipeline<String, Response<Full<Bytes>>, Response<Full<Bytes>>> {
  pipeline(|s| Ok(Response::builder().status(200).body(Full::new(Bytes::from(s))).unwrap()))
}
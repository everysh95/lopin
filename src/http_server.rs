use std::{collections::HashMap, convert::Infallible, error::Error, marker::PhantomData};

use crate::{filter, pipeline, util::from_utf8, AsyncFramework, AsyncPipeline, Pipeline, RawAsyncFramework, RawAsyncPipeline, RawPipeline};
use http_body_util::{BodyExt, Full};
use hyper::{body::{Body, Bytes, Incoming}, server::conn::http1, service::service_fn, Method, Request, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use tokio::net::TcpListener;
use async_trait::async_trait;
use serde_urlencoded::from_str;

struct HttpServer {
  address: String
}

#[async_trait]
impl RawAsyncFramework<Request<Bytes>,Response<Full<Bytes>>,Response<Full<Bytes>>> for HttpServer {
  async fn run(&self, pipeline: AsyncPipeline<Request<Bytes>,Response<Full<Bytes>>,Response<Full<Bytes>>>) {
    let listener = TcpListener::bind(&self.address).await.unwrap();
    loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);
      if let Err(err) = http1::Builder::new()
          .serve_connection(io, service_fn(|req| {
            async {
              Ok::<_,Infallible>(match (Ok(req) & to_bytes() & pipeline.clone()).await {
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

pub fn http_server(address: &str) -> AsyncFramework<Request<Bytes>,Response<Full<Bytes>>,Response<Full<Bytes>>> {
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
  async fn async_run(&self,r: Request<Incoming>) -> Result<Request<Bytes>, Response<Full<Bytes>>> {
    match r.into_body().collect().await {
      Ok(r) => Ok(Request::new(r.to_bytes())),
      Err(_) => Err(Response::builder().status(400).body(Full::new(Bytes::from(""))).unwrap()),
    }
  }
}

pub fn to_bytes() -> AsyncPipeline<Request<Incoming>,Request<Bytes>, Response<Full<Bytes>>> {
  AsyncPipeline::new(ToByte)
}

pub fn to_string() -> Pipeline<Request<Bytes>, Request<String>, Response<Full<Bytes>>> {
  request(pipeline(|b: Bytes| Ok(b.to_vec())) & from_utf8())
}

pub fn from_body<T: Body + Send + Sync + 'static>() -> Pipeline<Request<T>, T, Response<Full<Bytes>>> {
  pipeline(|bv: Request<T>| Ok(bv.into_body()))
}

pub fn request<VT : Send + Sync + 'static,RT:Send + Sync + 'static,ET: Send + Sync + Error + 'static>(pipline: Pipeline<VT,RT, ET>) -> Pipeline<Request<VT>,Request<RT>, Response<Full<Bytes>>> {
  pipeline(move |r: Request<VT>| {
    let params = r.into_body();
    match Ok(params) & pipline.clone() {
      Ok(v) => Ok(Request::new(v)),
      Err(e) => Err(Response::builder().status(400).body(Full::new(Bytes::from(e.to_string()))).unwrap())
    }
  })
}

pub fn with_query() -> Pipeline<Request<HashMap<String,String>>, Request<HashMap<String,String>>, Response<Full<Bytes>>> {
  pipeline(|r: Request<HashMap<String,String>>| {
    let rr = &r;
    let params = rr.body();
    let new_params = match from_str::<HashMap<String,String>>(rr.uri().query().unwrap_or_default()) {
      Ok(v) => {
        let mut new_paramas = params.clone();
        for (k,v) in v.iter() {
          new_paramas.insert(k.clone(), v.clone());
        }
        Ok(new_paramas)
      },
      Err(e) => Err(Response::builder().status(400).body(Full::new(Bytes::from(e.to_string()))).unwrap()),
    };

    match new_params {
      Ok(params) => Ok(Request::new(params)),
      Err(e) => Err(e)
    }
  })
}


pub fn to_body<T: Into<Bytes> + Send + Sync + 'static>() -> Pipeline<T, Response<Full<Bytes>>, Response<Full<Bytes>>> {
  pipeline(|s: T| Ok(Response::builder().status(200).body(Full::new(s.into())).unwrap()))
}
use std::{collections::HashMap, convert::Infallible, error::Error};

use crate::{filter, pipeline, util::from_utf8, AsyncFramework, AsyncPipeline, Pipeline, RawAsyncFramework, RawAsyncPipeline};
use http_body_util::{BodyExt, Full};
use hyper::{body::{Body, Bytes, Incoming}, server::conn::http1, service::service_fn, Method, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use tokio::net::TcpListener;
use async_trait::async_trait;
use serde_urlencoded::from_str;

pub use hyper::Request;

struct HttpServer {
  address: String
}

#[async_trait]
impl RawAsyncFramework<Request<HttpContext<Bytes>>,Response<Full<Bytes>>,Response<Full<Bytes>>> for HttpServer {
  async fn run(&self, pipeline: AsyncPipeline<Request<HttpContext<Bytes>>,Response<Full<Bytes>>,Response<Full<Bytes>>>) {
    let listener = TcpListener::bind(&self.address).await.unwrap();
    loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);
      if let Err(err) = http1::Builder::new()
          .serve_connection(io, service_fn(|req| {
            async {
              Ok::<_,Infallible>(match (Ok(req) & to_bytes() & wrap_context() & pipeline.clone()).await {
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

pub fn http_server(address: &str) -> AsyncFramework<Request<HttpContext<Bytes>>,Response<Full<Bytes>>,Response<Full<Bytes>>> {
  AsyncFramework::new(HttpServer {
    address: address.to_string()
  })
}

pub type HttpResponse = Response<Full<Bytes>>;
// pub type HttpAsyncPipeline<VT,RT> = AsyncPipeline<Request<HttpContext<VT>>, RT, HttpResponse>;

#[derive(Debug)]
pub struct HttpContext<T> {
  pub params: HashMap<String,String>,
  pub body: T
}

impl<T: Clone> Clone for HttpContext<T>  {
    fn clone(&self) -> Self {
        Self { params: self.params.clone(), body: self.body.clone() }
    }
}

impl<T> HttpContext<T> {
  pub fn new(params: HashMap<String,String>, body: T) -> HttpContext<T>{
    HttpContext {
      params,
      body
    }
  }
}

pub fn method_is<T: Clone + 'static>(method: Method) -> Pipeline<Request<HttpContext<T>>, Request<HttpContext<T>>, Response<Full<Bytes>>>{
  filter(move|r : &Request<HttpContext<T>>| r.method() == &method, Response::builder().status(405).body(Full::new(Bytes::from(""))).unwrap())
}

pub fn http_get<T: Clone + 'static>() -> Pipeline<Request<HttpContext<T>>, Request<HttpContext<T>>, Response<Full<Bytes>>> {
  method_is::<T>(Method::GET)
}

pub fn http_post<T: Clone + 'static>() -> Pipeline<Request<HttpContext<T>>, Request<HttpContext<T>>, Response<Full<Bytes>>> {
  method_is::<T>(Method::POST)
}

pub fn http_put<T: Clone + 'static>() -> Pipeline<Request<HttpContext<T>>, Request<HttpContext<T>>, Response<Full<Bytes>>> {
  method_is::<T>(Method::PUT)
}

pub fn http_delete<T: Clone + 'static>() -> Pipeline<Request<HttpContext<T>>, Request<HttpContext<T>>, Response<Full<Bytes>>> {
  method_is::<T>(Method::DELETE)
}

pub fn http_error<T: Clone + 'static>(status: u16, message: &'static str) -> Result<T, Response<Full<Bytes>>> {
  Err(Response::builder().status(status).body(Full::new(Bytes::from(message))).unwrap())
}

pub fn http_ok<ET: Clone + 'static>(status: u16, message: &'static str) -> Result<Response<Full<Bytes>>, ET> {
  Ok(Response::builder().status(status).body(Full::new(Bytes::from(message))).unwrap())
}


struct ToByte;

#[async_trait]
impl RawAsyncPipeline<Request<Incoming>, Request<Bytes>, Response<Full<Bytes>>> for ToByte {
  async fn async_run(&self,r: Request<Incoming>) -> Result<Request<Bytes>, Response<Full<Bytes>>> {
    let mut body: Option<Incoming> = None;
    let r = r.map(|b| body = Some(b));
    match body.unwrap().collect().await {
      Ok(rr) => {
        Ok(r.map(|_| rr.to_bytes()))
      },
      Err(_) => Err(Response::builder().status(400).body(Full::new(Bytes::from(""))).unwrap()),
    }
  }
}

fn to_bytes() -> AsyncPipeline<Request<Incoming>,Request<Bytes>, Response<Full<Bytes>>> {
  AsyncPipeline::new(ToByte)
}

fn wrap_context<T: Send  + 'static>() -> Pipeline<Request<T>,Request<HttpContext<T>>, Response<Full<Bytes>>> {
  pipeline(|r: Request<T>| Ok(r.map(|body| HttpContext::new(HashMap::new(), body))))
}

pub fn to_string() -> Pipeline<Request<HttpContext<Bytes>>, Request<HttpContext<String>>, Response<Full<Bytes>>> {
  request(pipeline(|b: Bytes| Ok(b.to_vec())) & from_utf8())
}

pub fn from_body<T: Body + Send + Sync + 'static>() -> Pipeline<Request<HttpContext<T>>, HttpContext<T>, Response<Full<Bytes>>> {
  pipeline(|bv: Request<HttpContext<T>>| Ok(bv.into_body()))
}

pub fn request<VT : Send + Sync + 'static,RT:Send + Sync + 'static,ET: Send + Sync + Error + 'static>(pipline: Pipeline<VT,RT, ET>) -> Pipeline<Request<HttpContext<VT>>,Request<HttpContext<RT>>, Response<Full<Bytes>>> {
  pipeline(move |r: Request<HttpContext<VT>>| {
    let req = r.map(|v: HttpContext<VT>| HttpContext::new(v.params,Ok(v.body) & pipline.clone()));
    match &req.body().body {
      Ok(_) => Ok(req.map(|r| HttpContext::new(r.params, r.body.unwrap()))),
      Err(e) => Err(Response::builder().status(400).body(Full::new(Bytes::from(e.to_string()))).unwrap())
    }
  })
}

pub fn from_path<T: 'static>(path: &str) -> Pipeline<Request<HttpContext<T>>, Request<HttpContext<T>>, Response<Full<Bytes>>> {
  let path_re_base = Regex::new("/:(\\w+)").unwrap();
  let path_params: Vec<String> = path_re_base.clone().captures_iter(path).map(|m| m.get(1).unwrap().as_str().to_string()).collect();
  let re_text = path_params.iter().fold(format!("^{path}$"),|p: String,pp| p.replace(&format!(":{pp}"), &format!("(?<{pp}>[^/]+)")));
  let path_re = Regex::new(&re_text).unwrap();
  pipeline(move |r: Request<HttpContext<T>>| {
    let rr = &r;
    let params = rr.body().params.clone();
    let new_params = match path_re.clone().captures(rr.uri().path()) {
        Some(r) => {
          let mut new_paramas = params.clone();
          for pp in path_params.clone().iter() {
            if let Some(r) = r.name(pp) {
              new_paramas.insert(pp.clone(), r.as_str().to_string());
            }
          }
          Ok(new_paramas)
        },
        None => Err(Response::builder().status(404).body(Full::new(Bytes::from(""))).unwrap()),
    };
    match new_params {
      Ok(params) => Ok(r.map(|old| HttpContext::new(params, old.body))),
      Err(e) => Err(e)
    }
  })
}

pub fn from_query<T: 'static>() -> Pipeline<Request<HttpContext<T>>, Request<HttpContext<T>>, Response<Full<Bytes>>> {
  pipeline(|r: Request<HttpContext<T>>| {
    let rr = &r;
    let params = rr.body().params.clone();
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
      Ok(params) => Ok(r.map(|old| HttpContext::new(params, old.body))),
      Err(e) => Err(e)
    }
  })
}


pub fn to_body<T: Into<Bytes> + Send + Sync + 'static>() -> Pipeline<T, Response<Full<Bytes>>, Response<Full<Bytes>>> {
  pipeline(|s: T| Ok(Response::builder().status(200).body(Full::new(s.into())).unwrap()))
}
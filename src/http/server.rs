use crate::{Transport, Propaty, Store, PropatyMap};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::collections::HashMap;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::body::{to_bytes, Bytes};

pub struct HttpTransporter {
    address: SocketAddr,
    keys: HashMap<Method, String>,
}


impl Transport<Vec<Propaty<String>>> for HttpTransporter {
    fn transport(&self, store: &mut Store<Vec<Propaty<String>>>) {
        let addr = self.address.clone();

        let make_service = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(|req| async {
                match self.keys.get(req.method()) {
                    Some(key) => {
                        if let Ok(bytes) = to_bytes(req.into_body()).await {
                            store.put(vec![Propaty::new(key,bytes)]);
                            match store.get() {
                                Some(res) => {
                                    match res.get_value::<Bytes>(key) {
                                        Some(res_bytes) => Response::builder().body(res_bytes.into()),
                                        None => Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()),
                                    }
                                },
                                None =>  Response::builder().body(Body::empty())
                            }
                        } else {
                            Response::builder().status(StatusCode::BAD_REQUEST).body(Body::empty())
                        }
                    },
                    None => Response::builder().status(StatusCode::METHOD_NOT_ALLOWED).body(Body::empty())
                }
            }))
        });

        let server = Server::bind(&addr).serve(make_service);

    }
}
use crate::{Propaty, PropatyMap, Store, Transport};
use async_trait::async_trait;
use hyper::body::{to_bytes, Bytes};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct HttpServerTransporter {
    address: SocketAddr,
    keys: HashMap<Method, String>,
}

#[async_trait]
impl Transport<Vec<Propaty<String>>> for HttpServerTransporter {
    async fn transport(&self, store: &mut Store<Vec<Propaty<String>>>) {
        let addr = self.address.clone();
        let make_service = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(|req| async {
                match self.keys.get(req.method()) {
                    Some(key) => {
                        if let Ok(bytes) = to_bytes(req.into_body()).await {
                            let props = vec![Propaty::new(key.clone(), bytes.clone())];
                            store.put(props).await;
                            match store.get().await {
                                Some(res) => match res.get_value::<Bytes>(key) {
                                    Some(res_bytes) => Response::builder().body(res_bytes.into()),
                                    None => Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(Body::empty()),
                                },
                                None => Response::builder().body(Body::empty()),
                            }
                        } else {
                            Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::empty())
                        }
                    }
                    None => Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(Body::empty()),
                }
            }))
        });

        let server = Server::bind(&addr).serve(make_service);
    }
}

pub fn http_bind(
    address: SocketAddr,
    keys: HashMap<Method, String>,
) -> Arc<dyn Transport<Vec<Propaty<String>>> + Send + Sync> {
    Arc::new(HttpServerTransporter {
        address: address,
        keys: keys,
    })
}

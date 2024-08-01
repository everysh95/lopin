use std::{future::Future, pin::Pin, sync::Arc};
use async_trait::async_trait;
use crate::{Framework, Pipeline, RawFramework, RawPipeline};

#[async_trait]
pub trait RawAsyncPipeline<VT, RT, ET> {
  async fn run(&self,value: VT) -> Result<RT, ET>;
}
pub struct AsyncPipeline<VT, RT, ET> {
  raw: Arc<dyn RawAsyncPipeline<VT, RT, ET> + Sync + Send + 'static>,
}

impl<VT: 'static,RT: 'static,ET: 'static> AsyncPipeline<VT,RT,ET> {
  pub fn new<EfffectorT: RawAsyncPipeline<VT,RT,ET> + Sync + Send + 'static>(raw: EfffectorT) -> Pipeline<VT,RT,ET> {
    return Pipeline::new(AsyncPipeline{
      raw: Arc::new(raw)
    });
  }
}

impl<VT,RT,ET> RawPipeline<VT,RT,ET> for AsyncPipeline<VT,RT,ET> {
  fn run(&self,value: VT) -> Result<RT, ET> {
    let rt  = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
      self.raw.run(value).await
    })
  }
}

#[async_trait]
pub trait RawAsyncFramework<VT,RT,ET> {
  async fn run(&self, pipeline: Pipeline<VT,RT,ET>);
}

pub struct AsyncFramework<VT,RT,ET> {
  raw: Arc<dyn RawAsyncFramework<VT,RT,ET>>
}

impl<VT: 'static,RT: 'static,ET: 'static> AsyncFramework<VT,RT,ET> {
  pub fn new<FT: RawAsyncFramework<VT,RT,ET> + 'static>(f: FT) -> Framework<VT, RT, ET> {
    Framework::new(
      AsyncFramework {
        raw: Arc::new(f)
      }
    ) 
  }
}

impl<VT,RT,ET> RawFramework<VT,RT,ET> for AsyncFramework<VT,RT,ET> {
  fn run(&self, pipeline: Pipeline<VT,RT,ET>) {
    let rt  = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
      self.raw.run(pipeline).await
    });
  }
}


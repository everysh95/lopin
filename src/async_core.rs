use std::{future::Future, ops::{BitAnd, BitOr, BitXor}, pin::Pin, sync::Arc};
use async_trait::async_trait;

use crate::{Pipeline, RawPipeline};

#[async_trait]
pub trait RawAsyncPipeline<VT, RT, ET> {
  async fn async_run(&self,value: VT) -> Result<RT, ET>;
}

#[async_trait]
impl<VT: Send + 'static,RT: Send + 'static,ET: Send + 'static> RawAsyncPipeline<VT,RT,ET> for Pipeline<VT,RT,ET> {
  async fn async_run(&self,value: VT) -> Result<RT, ET> {
    self.run(value)
  }
}

pub struct AsyncPipeline<VT, RT, ET> {
  raw: Arc<dyn RawAsyncPipeline<VT, RT, ET> + Sync + Send + 'static>,
}

impl<VT: Send + 'static, RT: Send + 'static, ET: Send + 'static> AsyncPipeline<VT,RT,ET> {
  pub fn new<EfffectorT: RawAsyncPipeline<VT,RT,ET> + Sync + Send + 'static>(raw: EfffectorT) -> AsyncPipeline<VT,RT,ET> {
    return AsyncPipeline{
      raw: Arc::new(raw)
    };
  }
}

impl<VT,RT,ET> Clone for AsyncPipeline<VT,RT,ET> {
  fn clone(&self) -> Self {
    Self { raw: self.raw.clone() }
  }
}

#[async_trait]
impl<VT: Send + 'static,RT: Send + 'static,ET: Send + 'static> RawAsyncPipeline<VT,RT,ET> for AsyncPipeline<VT,RT,ET> {
  async fn async_run(&self,value: VT) -> Result<RT, ET> {
    self.raw.async_run(value).await
  }
}

impl<VT: Send + 'static, RT: Send + 'static, ET: Send + 'static> BitAnd<AsyncPipeline<VT,RT,ET>> for Result<VT,ET> {
    type Output = Pin<Box<dyn Future<Output = Result<RT,ET>> + Send + 'static>>;

    fn bitand(self, rhs: AsyncPipeline<VT,RT,ET>) -> Self::Output {
      Box::pin(async move {
        match self {
            Ok(v) => rhs.async_run(v).await,
            Err(e) => Err(e),
        }
      })
    }
}

struct AndAsyncPipeline<VT, MT, RT, ET> {
  lhs: AsyncPipeline<VT, MT, ET>,
  rhs: AsyncPipeline<MT, RT, ET>
}

#[async_trait]
impl<VT: Send + 'static, MT: Send + 'static, RT: Send + 'static, ET: Send + 'static> RawAsyncPipeline<VT,RT,ET> for AndAsyncPipeline<VT,MT,RT,ET> {
  async fn async_run(&self,value: VT) -> Result<RT, ET> {
    match self.lhs.async_run(value).await {
      Ok(v) => self.rhs.async_run(v).await,
      Err(e) => Err(e),
    }  
  }
}

impl<VT: Send + 'static, MT:Send + 'static, RT: Send + 'static, ET: Send + 'static> BitAnd<AsyncPipeline<MT,RT,ET>> for AsyncPipeline<VT,MT,ET> {
    type Output = AsyncPipeline<VT,RT,ET>;

    fn bitand(self, rhs: AsyncPipeline<MT,RT,ET>) -> Self::Output {
      AsyncPipeline::new(AndAsyncPipeline {
        lhs: self,
        rhs
      })
    }
}

impl<VT: Send + 'static, MT:Send + 'static, RT: Send + 'static, ET: Send + 'static> BitAnd<Pipeline<MT,RT,ET>> for AsyncPipeline<VT,MT,ET> {
    type Output = AsyncPipeline<VT,RT,ET>;

    fn bitand(self, rhs: Pipeline<MT,RT,ET>) -> Self::Output {
      AsyncPipeline::new(AndAsyncPipeline {
        lhs: self,
        rhs: AsyncPipeline::new(rhs)
      })
    }
}

impl<VT: Send + 'static, MT:Send + 'static, RT: Send + 'static, ET: Send + 'static> BitAnd<AsyncPipeline<MT,RT,ET>> for Pipeline<VT,MT,ET> {
    type Output = AsyncPipeline<VT,RT,ET>;

    fn bitand(self, rhs: AsyncPipeline<MT,RT,ET>) -> Self::Output {
      AsyncPipeline::new(AndAsyncPipeline {
        lhs: AsyncPipeline::new(self),
        rhs
      })
    }
}

struct OrAsyncPipeline<VT, RT, ET> {
  lhs: AsyncPipeline<VT, RT, ET>,
  rhs: AsyncPipeline<VT, RT, ET>
}

#[async_trait]
impl<VT: Clone + Send + 'static, RT: Send + 'static, ET: Send + 'static> RawAsyncPipeline<VT,RT,ET> for OrAsyncPipeline<VT,RT,ET> {
  async fn async_run(&self,value: VT) -> Result<RT, ET> {
    match self.lhs.async_run(value.clone()).await {
      Ok(v) => Ok(v),
      Err(_) => self.rhs.async_run(value.clone()).await,
    }  
  }
}

impl<VT: Clone + Send + 'static, RT: Send + 'static, ET: Send + 'static> BitOr<AsyncPipeline<VT,RT,ET>> for AsyncPipeline<VT,RT,ET> {
    type Output = AsyncPipeline<VT,RT,ET>;

    fn bitor(self, rhs: AsyncPipeline<VT,RT,ET>) -> Self::Output {
      AsyncPipeline::new(OrAsyncPipeline {
        lhs: self,
        rhs
      })
    }
}

impl<VT: Clone + Send + 'static, RT: Send + 'static, ET: Send + 'static> BitOr<Pipeline<VT,RT,ET>> for AsyncPipeline<VT,RT,ET> {
    type Output = AsyncPipeline<VT,RT,ET>;

    fn bitor(self, rhs: Pipeline<VT,RT,ET>) -> Self::Output {
      AsyncPipeline::new(OrAsyncPipeline {
        lhs: self,
        rhs: AsyncPipeline::new(rhs)
      })
    }
}

impl<VT: Clone + Send + 'static, RT: Send + 'static, ET: Send + 'static> BitOr<AsyncPipeline<VT,RT,ET>> for Pipeline<VT,RT,ET> {
    type Output = AsyncPipeline<VT,RT,ET>;

    fn bitor(self, rhs: AsyncPipeline<VT,RT,ET>) -> Self::Output {
      AsyncPipeline::new(OrAsyncPipeline {
        lhs: AsyncPipeline::new(self),
        rhs
      })
    }
}



impl<VT: Send + 'static, RT: Send + 'static, ET: Send + 'static> BitAnd<AsyncPipeline<VT,RT,ET>> for Pin<Box<dyn Future<Output = Result<VT,ET>> + Send + 'static>> {
    type Output = Pin<Box<dyn Future<Output = Result<RT,ET>> + Send + 'static>>;

    fn bitand(self, rhs: AsyncPipeline<VT,RT,ET>) -> Self::Output {
      Box::pin(async move {
        match self.await {
            Ok(v) => rhs.async_run(v).await,
            Err(e) => Err(e),
        }
      })
    }
}


#[async_trait]
pub trait RawAsyncFramework<VT,RT,ET> {
  async fn run(&self, pipeline: AsyncPipeline<VT,RT,ET>);
}

pub struct AsyncFramework<VT,RT,ET> {
  raw: Arc<dyn RawAsyncFramework<VT,RT,ET> + Send + Sync>
}

impl<VT: 'static,RT: 'static,ET: 'static> AsyncFramework<VT,RT,ET> {
  pub fn new<FT: RawAsyncFramework<VT,RT,ET> + Send + Sync + 'static>(f: FT) -> AsyncFramework<VT, RT, ET> {
    AsyncFramework {
      raw: Arc::new(f)
    }
  }
}

#[async_trait]
impl<VT,RT,ET> RawAsyncFramework<VT,RT,ET> for AsyncFramework<VT,RT,ET> {
  async fn run(&self, pipeline: AsyncPipeline<VT,RT,ET>) {
    self.raw.run(pipeline).await
  }
}


impl<VT: 'static,RT: 'static,ET: 'static> BitXor<AsyncFramework<VT,RT,ET>> for AsyncPipeline<VT,RT,ET> {
    type Output = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    fn bitxor(self, rhs: AsyncFramework<VT,RT,ET>) -> Self::Output {
      Box::pin(async move {
        rhs.run(self).await;
      })
    }
}

impl<VT: 'static,RT: 'static,ET: 'static> BitXor<AsyncPipeline<VT,RT,ET>> for AsyncFramework<VT,RT,ET> {
    type Output = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    fn bitxor(self, rhs: AsyncPipeline<VT,RT,ET>) -> Self::Output {
      Box::pin(async move {
        self.run(rhs).await;
      })
    }
}

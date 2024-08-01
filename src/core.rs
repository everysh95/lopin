use std::{ops::{BitAnd, BitOr, BitXor}, sync::Arc};

pub trait RawPipeline<VT, RT, ET> {
  fn run(&self,value: VT) -> Result<RT, ET>;
}
pub struct Pipeline<VT, RT, ET> {
  raw: Arc<dyn RawPipeline<VT, RT, ET> + Sync + Send + 'static>,
}

impl<VT,RT,ET> Pipeline<VT,RT,ET> {
  pub fn new<EfffectorT: RawPipeline<VT,RT,ET> + Sync + Send + 'static>(raw: EfffectorT) -> Pipeline<VT,RT,ET> {
    return Pipeline{
      raw: Arc::new(raw)
    };
  }
}

impl<VT,RT,ET> Clone for Pipeline<VT,RT,ET> {
  fn clone(&self) -> Self {
    Self { raw: self.raw.clone() }
  }
}

impl<VT,RT,ET> RawPipeline<VT,RT,ET> for Pipeline<VT,RT,ET> {
  fn run(&self,value: VT) -> Result<RT, ET> {
    self.raw.run(value)
  }
}

impl<VT: 'static, RT: 'static, ET: 'static> BitAnd<Pipeline<VT,RT,ET>> for Result<VT,ET> {
    type Output = Result<RT,ET>;

    fn bitand(self, rhs: Pipeline<VT,RT,ET>) -> Self::Output {
      match self {
          Ok(v) => rhs.run(v),
          Err(e) => Err(e),
      }
    }
}

struct AndPipeline<VT, MT, RT, ET> {
  lhs: Pipeline<VT, MT, ET>,
  rhs: Pipeline<MT, RT, ET>
}

impl<VT: 'static, MT: 'static, RT: 'static, ET: 'static> RawPipeline<VT,RT,ET> for AndPipeline<VT,MT,RT,ET> {
  fn run(&self,value: VT) -> Result<RT, ET> {
    match self.lhs.run(value) {
      Ok(v) => self.rhs.run(v),
      Err(e) => Err(e),
    }  
  }
}

impl<VT: 'static, MT:'static, RT: 'static, ET: 'static> BitAnd<Pipeline<MT,RT,ET>> for Pipeline<VT,MT,ET> {
    type Output = Pipeline<VT,RT,ET>;

    fn bitand(self, rhs: Pipeline<MT,RT,ET>) -> Self::Output {
      Pipeline::new(AndPipeline {
        lhs: self,
        rhs
      })
    }
}

struct OrPipeline<VT, RT, ET> {
  lhs: Pipeline<VT, RT, ET>,
  rhs: Pipeline<VT, RT, ET>
}

impl<VT: Clone + 'static, RT: 'static, ET: 'static> RawPipeline<VT,RT,ET> for OrPipeline<VT,RT,ET> {
  fn run(&self,value: VT) -> Result<RT, ET> {
    match self.lhs.run(value.clone()) {
      Ok(v) => Ok(v),
      Err(_) => self.rhs.run(value.clone()),
    }  
  }
}

impl<VT: Clone + 'static, RT: 'static, ET: 'static> BitOr<Pipeline<VT,RT,ET>> for Pipeline<VT,RT,ET> {
    type Output = Pipeline<VT,RT,ET>;

    fn bitor(self, rhs: Pipeline<VT,RT,ET>) -> Self::Output {
      Pipeline::new(OrPipeline {
        lhs: self,
        rhs
      })
    }
}

struct SimplePipeline<VT, RT, ET> {
  raw: Arc<dyn Fn(VT) -> Result<RT, ET> + Sync + Send + 'static>
}

impl<VT, RT, ET> RawPipeline<VT,RT, ET> for SimplePipeline<VT, RT, ET> {
    fn run(&self,value: VT) -> Result<RT, ET> {
      (self.raw)(value)
    }
}

pub fn pipeline<VT: 'static, RT: 'static, ET: 'static, F: Fn(VT) -> Result<RT, ET> + Sync + Send + 'static>(f : F) -> Pipeline<VT,RT,ET> {
  return Pipeline::new(SimplePipeline{
    raw: Arc::new(f),
  });
}

struct FilterPipeline<VT,ET: 'static> {
  raw: Arc<dyn Fn(&VT) -> bool + Sync + Send + 'static>,
  error: ET
}

impl<VT,ET : Clone> RawPipeline<VT,VT,ET> for FilterPipeline<VT,ET> {
    fn run(&self,v: VT) -> Result<VT, ET> {
      if (self.raw)(&v) {
        Ok(v)
      } else {
        Err(self.error.clone())
      }
    }
}

pub fn filter<T: 'static, ET: Clone + Sync + Send + 'static, F: Fn(&T) -> bool + Sync + Send + 'static>(f: F, error: ET) -> Pipeline<T, T, ET> {
  return Pipeline::new(FilterPipeline {
    raw: Arc::new(f),
    error
  });
}

pub trait RawFramework<VT,RT,ET> {
  fn run(&self, pipeline: Pipeline<VT,RT,ET>);
}

pub struct Framework<VT,RT,ET> {
  raw: Arc<dyn RawFramework<VT,RT,ET>>
}

impl<VT,RT,ET> Framework<VT,RT,ET> {
  pub fn new<FT: RawFramework<VT,RT,ET> + 'static>(f: FT) -> Framework<VT, RT, ET> {
    Framework {
      raw: Arc::new(f)
    }
  }
}

impl<VT,RT,ET> RawFramework<VT,RT,ET> for Framework<VT,RT,ET> {
  fn run(&self, pipeline: Pipeline<VT,RT,ET>) {
    self.raw.run(pipeline)
  }
}

impl<VT,RT,ET> BitXor<Framework<VT,RT,ET>> for Pipeline<VT,RT,ET> {
    type Output = ();

    fn bitxor(self, rhs: Framework<VT,RT,ET>) -> Self::Output {
      rhs.run(self)
    }
}

impl<VT,RT,ET> BitXor<Pipeline<VT,RT,ET>> for Framework<VT,RT,ET> {
    type Output = ();

    fn bitxor(self, rhs: Pipeline<VT,RT,ET>) -> Self::Output {
      self.run(rhs)
    }
}

struct SimpleFramework<VT,RT,ET> {
  raw: Arc<dyn Fn(Pipeline<VT,RT,ET>) -> ()>
}

impl<VT,RT,ET> RawFramework<VT,RT,ET> for SimpleFramework<VT,RT,ET>  {
  fn run(&self, pipeline: Pipeline<VT,RT,ET>) {
    (self.raw)(pipeline)
  }
}

pub fn framework<VT: 'static,RT: 'static,ET: 'static,FT: Fn(Pipeline<VT,RT,ET>) -> () + 'static>(f: FT) -> Framework<VT,RT,ET> {
  Framework::new(SimpleFramework {
    raw: Arc::new(f)
  })
}


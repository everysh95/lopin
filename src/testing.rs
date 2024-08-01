use std::fmt::Debug;

use crate::{framework, Framework};

pub fn test_ok<VT: Clone + 'static,RT: Clone + PartialEq + Debug + 'static,ET: 'static>(init: VT, result: RT) -> Framework<VT,RT,ET>{
  framework(move |p| {
    assert_eq!((Ok(init.clone()) & p).ok(), Some(result.clone()));
  })
}

pub fn test_error<VT: Clone + 'static,RT: 'static,ET: Clone + PartialEq + Debug + 'static>(init: VT, error: ET) -> Framework<VT,RT,ET>{
  framework(move |p| {
    assert_eq!((Ok(init.clone()) & p).err(), Some(error.clone()));
  })
}
use std::string::FromUtf8Error;

use crate::{framework, pipeline, Framework, Pipeline};

pub fn simple_loop<RT: 'static,ET: 'static>() -> Framework<(),RT,ET> {
  framework(|p| {
    loop {
      let _ = Ok(()) & p.clone();
    }
  })
}

pub fn from_utf8() -> Pipeline<Vec<u8>, String, FromUtf8Error> {
  pipeline(|p: Vec<u8>| String::from_utf8(p))
}
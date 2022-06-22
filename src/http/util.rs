use crate::{Converter};
use hyper::body::Bytes;

pub struct FromUtf8 { }

impl Converter<String,Bytes> for FromUtf8 {
    fn to(&self,src:String) -> Option<Bytes> {
        Some(Bytes::from(src.clone()))
    }
    fn from(&self,dist:Bytes) -> Option<String> {
        if let Ok(res) = String::from_utf8(dist.to_vec()) {
            Some(res)
        } else {
            None
        }
    }

}

pub fn from_utf8() -> Box<dyn Converter<String,Bytes>> {
    Box::new(FromUtf8 { })
}

pub struct ToUtf8 { }

impl Converter<Bytes, String> for ToUtf8 {
    fn to(&self,src:Bytes) -> Option<String> {
        if let Ok(res) = String::from_utf8(src.to_vec()) {
            Some(res)
        } else {
            None
        }
    }
    fn from(&self,dist:String) -> Option<Bytes> {
        Some(Bytes::from(dist.clone()))
    }

}

pub fn to_utf8() -> Box<dyn Converter<Bytes,String>> {
    Box::new(ToUtf8 { })
}
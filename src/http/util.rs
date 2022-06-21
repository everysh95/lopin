use crate::{Converter, Propaty, PropatyMap};
use hyper::body::Bytes;

pub struct Utf8Text {
    key: String
}

impl Converter<Vec<Propaty<String>>,Vec<Propaty<String>>> for Utf8Text {
    fn to(&self,src:Vec<Propaty<String>>) -> Option<Vec<Propaty<String>>> {
        let mut res = src.clone();
        if let Some(index) = src.iter().position(|p| p.key == self.key) {
            if let Some(value) = src.get_value::<String>(&self.key) {
                res[index] = Propaty::new(self.key.clone(), Bytes::from(value));
            }
        }
        Some(res)
    }
    fn from(&self,dist:Vec<Propaty<String>>) -> Option<Vec<Propaty<String>>> {
        let mut res = dist.clone();
        if let Some(index) = dist.iter().position(|p| p.key == self.key) {
            if let Some(value) = dist.get_value::<Bytes>(&self.key) {
                res[index] = Propaty::new(self.key.clone(), String::from_utf8(value.to_vec()));
            }
        }
        Some(res)
    }

}

pub fn utf8_text(key: &str) -> Box<dyn Converter<Vec<Propaty<String>>,Vec<Propaty<String>>>> {
    Box::new(Utf8Text {
        key: key.to_string()
    })
}
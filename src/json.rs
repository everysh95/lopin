use serde::de::DeserializeOwned;

use crate::{pipeline, Pipeline};


pub fn to_json<VT: serde::Serialize + 'static>() -> Pipeline<VT,String,serde_json::Error> {
  pipeline(|value: VT| serde_json::to_string(&value))
}

pub fn from_json<RT: DeserializeOwned + 'static>() -> Pipeline<String,RT,serde_json::Error> {
  pipeline(|value: String| serde_json::from_str(&value))
}
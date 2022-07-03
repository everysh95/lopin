use crate::propaty::PropatyValue;
use crate::Propaty;
use serde::de::*;
use serde::ser::*;
use std::fmt;
use std::marker::Send;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Record {
    pub props: Vec<Propaty<String>>,
}

impl Record {
    pub fn new(props: Vec<Propaty<String>>) -> Record {
        Record { props: props }
    }
}

impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.props.len()))?;
        for props in self.props.iter() {
            let value_lock = props.value.lock().unwrap();
            let value_any = value_lock.get();
            if let Some(value) = value_any.downcast_ref::<String>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<i8>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<i16>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<i32>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<i64>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<i128>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<u8>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<u16>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<u32>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<u64>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<u128>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<f32>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<f64>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<bool>() {
                map.serialize_entry(&props.key, value)?;
            } else if let Some(value) = value_any.downcast_ref::<Vec<Propaty<String>>>() {
                map.serialize_entry(&props.key, &Record::new(value.clone()))?;
            }
        }
        map.end()
    }
}

struct RecordValue {
    pub value: Arc<Mutex<dyn PropatyValue + Send + Sync>>,
}
struct RecordValueVisitor;

impl<'de> Visitor<'de> for RecordValueVisitor {
    type Value = RecordValue;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("illigual format!!!")
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v.to_string())),
        })
    }
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v.to_string())),
        })
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RecordValue {
            value: Arc::new(Mutex::new(v)),
        })
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map_mut = map;
        let mut res: Vec<Propaty<String>> = vec![];
        while let Some(prop) = RecordVisitor::next_to_propaty(&mut map_mut) {
            res.push(prop);
        }
        Ok(RecordValue {
            value: Arc::new(Mutex::new(res)),
        })
    }

}

impl<'de> Deserialize<'de> for RecordValue {
    fn deserialize<D>(deserializer: D) -> Result<RecordValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RecordValueVisitor)
    }
}

struct RecordVisitor;

impl RecordVisitor {
    fn next_to_propaty<'de, A>(map: &mut A) -> Option<Propaty<String>>
    where
        A: MapAccess<'de>,
    {
        if let Ok(kvo) = map.next_entry::<String, RecordValue>() {
            if let Some(kv) = kvo {
                return Some(Propaty {
                    key: kv.0,
                    value: kv.1.value,
                });
            }
        }
        None
    }
}

impl<'de> Visitor<'de> for RecordVisitor {
    type Value = Record;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("illigual format!!!")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map_mut = map;
        let mut res: Vec<Propaty<String>> = vec![];
        while let Some(prop) = RecordVisitor::next_to_propaty(&mut map_mut) {
            res.push(prop);
        }
        Ok(Record { props: res })
    }
}

impl<'de> Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> Result<Record, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RecordVisitor)
    }
}

use std::{collections::HashMap, ops::{BitOr, ShlAssign}};

use serde::{Deserialize, Serialize};

use crate::{Store, store};

#[derive(Debug,Clone)]
pub enum StoreAny {
  IntStore(Store<i64>),
  UintStore(Store<u64>),
  FloatStore(Store<f64>),
  StringStore(Store<String>)
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Table {
    int_store: Store<Store<i64>>,
    uint_store: Store<Store<u64>>,
    float_store: Store<Store<f64>>,
    str_store: Store<Store<String>>,
    col_name: HashMap<String, usize>,
    max_sid: usize,
    max_id: usize
}

impl BitOr<(String, Store<i64>)> for Table {
    type Output = Table;
    fn bitor(self, rhs: (String, Store<i64>)) -> Self::Output {
        let (name, store) = rhs;
        let mut result = self.clone();
        result.max_sid = self.max_sid + 1;
        result.col_name.insert(name,result.max_sid);
        result.int_store = result.int_store | (result.max_sid, store);
        result
    }
}

impl BitOr<(String, Store<u64>)> for Table {
    type Output = Table;
    fn bitor(self, rhs: (String, Store<u64>)) -> Self::Output {
        let (name, store) = rhs;
        let mut result = self.clone();
        result.max_sid = self.max_sid + 1;
        result.col_name.insert(name,result.max_sid);
        result.uint_store = result.uint_store | (result.max_sid, store);
        result
    }
}

impl BitOr<(String, Store<f64>)> for Table {
    type Output = Table;
    fn bitor(self, rhs: (String, Store<f64>)) -> Self::Output {
        let (name, store) = rhs;
        let mut result = self.clone();
        result.max_sid = self.max_sid + 1;
        result.col_name.insert(name,result.max_sid);
        result.float_store = result.float_store | (result.max_sid, store);
        result
    }
}

impl BitOr<(String, Store<String>)> for Table {
    type Output = Table;
    fn bitor(self, rhs: (String, Store<String>)) -> Self::Output {
        let (name, store) = rhs;
        let mut result = self.clone();
        result.max_sid = self.max_sid + 1;
        result.col_name.insert(name,result.max_sid);
        result.str_store = result.str_store | (result.max_sid, store);
        result
    }
}

pub fn table() -> Table {
    Table {
        int_store: Store::new(),
        uint_store: Store::new(),
        float_store: Store::new(),
        str_store: Store::new(),
        col_name: HashMap::new(),
        max_id: 0,
        max_sid: 0,
    }
}

pub fn col_int(name: &str) -> (String, Store<i64>) {
    (name.to_string(), Store::new())
}

pub fn col_uint(name: &str) -> (String, Store<u64>) {
    (name.to_string(), Store::new())
}

pub fn col_float(name: &str) -> (String, Store<f64>) {
    (name.to_string(), Store::new())
}

pub fn col_str(name: &str) -> (String, Store<String>) {
    (name.to_string(), Store::new())
}

#[derive(Debug,Clone)]
pub enum TableAnyValue {
    IntValue(i64),
    UintValue(u64),
    FloatValue(f64),
    StringValue(String)
}

impl BitOr<(&str,i64)> for Store<(String,TableAnyValue)>
{
    type Output = Store<(String,TableAnyValue)>;
    fn bitor(self, rhs: (&str,i64)) -> Self::Output {
        let (k,v) = rhs;
        self | Some((k.to_string(), TableAnyValue::IntValue(v)))
    }
}
impl BitOr<(&str,u64)> for Store<(String,TableAnyValue)>
{
    type Output = Store<(String,TableAnyValue)>;
    fn bitor(self, rhs: (&str,u64)) -> Self::Output {
        let (k,v) = rhs;
        self | Some((k.to_string(), TableAnyValue::UintValue(v)))
    }
}

impl BitOr<(&str,f64)> for Store<(String,TableAnyValue)>
{
    type Output = Store<(String,TableAnyValue)>;
    fn bitor(self, rhs: (&str,f64)) -> Self::Output {
        let (k,v) = rhs;
        self | Some((k.to_string(), TableAnyValue::FloatValue(v)))
    }
}

impl BitOr<(&str,&str)> for Store<(String,TableAnyValue)>
{
    type Output = Store<(String,TableAnyValue)>;
    fn bitor(self, rhs: (&str,&str)) -> Self::Output {
        let (k,v) = rhs;
        self | Some((k.to_string(), TableAnyValue::StringValue(v.to_string())))
    }
}

impl ShlAssign<Store<(String,TableAnyValue)>> for Table {
    fn shl_assign(&mut self, rhs: Store<(String,TableAnyValue)>) {
        self.max_id = self.max_id + 1;
        for index in 0..rhs.get_max_id() {
            let kv = rhs.select(index + 1).unwrap();
            let (name,value) = kv;
            match value {
                TableAnyValue::IntValue(v) => {
                    let id = self.col_name.get(&name).unwrap().clone();
                    let mut s = self.int_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.int_store <<= Store::new() | (id,s);
                },
                TableAnyValue::UintValue(v) => {
                    let id = self.col_name.get(&name).unwrap().clone();
                    let mut s = self.uint_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.uint_store <<= Store::new() | (id,s);
                },
                TableAnyValue::FloatValue(v) => {
                    let id = self.col_name.get(&name).unwrap().clone();
                    let mut s = self.float_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.float_store <<= Store::new() | (id,s);
                },
                TableAnyValue::StringValue(v) => {
                    let id = self.col_name.get(&name).unwrap().clone();
                    let mut s = self.str_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.str_store <<= Store::new() | (id,s);
                },
            };
        }
    }
}
impl BitOr<i64> for Store<TableAnyValue>
{
    type Output = Store<TableAnyValue>;
    fn bitor(self, rhs: i64) -> Self::Output {
        self | Some(TableAnyValue::IntValue(rhs))
    }
}
impl BitOr<u64> for Store<TableAnyValue>
{
    type Output = Store<TableAnyValue>;
    fn bitor(self, rhs: u64) -> Self::Output {
        self | Some(TableAnyValue::UintValue(rhs))
    }
}

impl BitOr<f64> for Store<TableAnyValue>
{
    type Output = Store<TableAnyValue>;
    fn bitor(self, rhs: f64) -> Self::Output {
        self | Some(TableAnyValue::FloatValue(rhs))
    }
}

impl BitOr<&str> for Store<TableAnyValue>
{
    type Output = Store<TableAnyValue>;
    fn bitor(self, rhs: &str) -> Self::Output {
        self | Some(TableAnyValue::StringValue(rhs.to_string()))
    }
}

impl ShlAssign<Store<TableAnyValue>> for Table {
    fn shl_assign(&mut self, rhs: Store<TableAnyValue>) {
        self.max_id = self.max_id + 1;
        for id in rhs.get_ids().iter().cloned() {
            let value = rhs.select(id).unwrap();
            match value {
                TableAnyValue::IntValue(v) => {
                    let mut s = self.int_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.int_store <<= Store::new() | (id,s);
                },
                TableAnyValue::UintValue(v) => {
                    let mut s = self.uint_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.uint_store <<= Store::new() | (id,s);
                },
                TableAnyValue::FloatValue(v) => {
                    let mut s = self.float_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.float_store <<= Store::new() | (id,s);
                },
                TableAnyValue::StringValue(v) => {
                    let mut s = self.str_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.str_store <<= Store::new() | (id,s);
                },
            };
        }
    }
}

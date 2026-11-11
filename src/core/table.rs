use std::{collections::HashMap, ops::{BitOr, ShlAssign}};

use serde::{Deserialize, Serialize};

use crate::Store;

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
enum TableAnyValue {
    IntValue(i64),
    UintValue(u64),
    FloatValue(f64),
    StringValue(String)
}

#[derive(Debug,Clone)]
pub struct NamedValues
{
    map: HashMap<String,TableAnyValue>
}

pub fn named() -> NamedValues
{
    NamedValues { map: HashMap::new() }
}

impl BitOr<(&str,i64)> for NamedValues
{
    type Output = NamedValues;
    fn bitor(self, rhs: (&str,i64)) -> Self::Output {
        let mut res = self.clone();
        let (k,v)= rhs;
        res.map.insert(k.to_string(), TableAnyValue::IntValue(v));
        res
    }
}
impl BitOr<(&str,u64)> for NamedValues
{
    type Output = NamedValues;
    fn bitor(self, rhs: (&str,u64)) -> Self::Output {
        let mut res = self.clone();
        let (k,v)= rhs;
        res.map.insert(k.to_string(), TableAnyValue::UintValue(v));
        res
    }
}
impl BitOr<(&str,f64)> for NamedValues
{
    type Output = NamedValues;
    fn bitor(self, rhs: (&str,f64)) -> Self::Output {
        let mut res = self.clone();
        let (k,v)= rhs;
        res.map.insert(k.to_string(), TableAnyValue::FloatValue(v));
        res
    }
}
impl BitOr<(&str,&str)> for NamedValues
{
    type Output = NamedValues;
    fn bitor(self, rhs: (&str,&str)) -> Self::Output {
        let mut res = self.clone();
        let (k,v)= rhs;
        res.map.insert(k.to_string(), TableAnyValue::StringValue(v.to_string()));
        res
    }
}

impl ShlAssign<NamedValues> for Table {
    fn shl_assign(&mut self, rhs: NamedValues) {
        self.max_id = self.max_id + 1;
        for (name,value) in rhs.map.iter() {
            match value {
                TableAnyValue::IntValue(v) => {
                    let id = self.col_name.get(name).unwrap().clone();
                    let mut s = self.int_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.int_store <<= Store::new() | (id,s);
                },
                TableAnyValue::UintValue(v) => {
                    let id = self.col_name.get(name).unwrap().clone();
                    let mut s = self.uint_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.uint_store <<= Store::new() | (id,s);
                },
                TableAnyValue::FloatValue(v) => {
                    let id = self.col_name.get(name).unwrap().clone();
                    let mut s = self.float_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.float_store <<= Store::new() | (id,s);
                },
                TableAnyValue::StringValue(v) => {
                    let id = self.col_name.get(name).unwrap().clone();
                    let mut s = self.str_store.select(id).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.str_store <<= Store::new() | (id,s);
                },
            };
        }
    }
}

#[derive(Debug,Clone)]
pub struct TableRow
{
    vec: Vec<TableAnyValue>
}

pub fn values() -> TableRow {
    TableRow { vec: vec![] }
}



impl BitOr<i64> for TableRow
{
    type Output = TableRow;
    fn bitor(self, rhs: i64) -> Self::Output {
        let mut res = self.clone();
        res.vec.push(TableAnyValue::IntValue(rhs));
        res
    }
}
impl BitOr<u64> for TableRow
{
    type Output = TableRow;
    fn bitor(self, rhs: u64) -> Self::Output {
        let mut res = self.clone();
        res.vec.push(TableAnyValue::UintValue(rhs));
        res
    }
}

impl BitOr<f64> for TableRow
{
    type Output = TableRow;
    fn bitor(self, rhs: f64) -> Self::Output {
        let mut res = self.clone();
        res.vec.push(TableAnyValue::FloatValue(rhs));
        res
    }
}

impl BitOr<&str> for TableRow
{
    type Output = TableRow;
    fn bitor(self, rhs: &str) -> Self::Output {
        let mut res = self.clone();
        res.vec.push(TableAnyValue::StringValue(rhs.to_string()));
        res
    }
}

impl ShlAssign<TableRow> for Table {
    fn shl_assign(&mut self, rhs: TableRow) {
        self.max_id = self.max_id + 1;
        for (index,value) in (0..self.max_sid).zip(rhs.vec.iter()) {
            match value {
                TableAnyValue::IntValue(v) => {
                    let mut s = self.int_store.select(index + 1).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.int_store <<= Store::new() | (index + 1,s);
                },
                TableAnyValue::UintValue(v) => {
                    let mut s = self.uint_store.select(index + 1).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.uint_store <<= Store::new() | (index + 1,s);
                },
                TableAnyValue::FloatValue(v) => {
                    let mut s = self.float_store.select(index + 1).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.float_store <<= Store::new() | (index + 1,s);
                },
                TableAnyValue::StringValue(v) => {
                    let mut s = self.str_store.select(index + 1).unwrap();
                    s = s | (self.max_id,v.clone());
                    self.str_store <<= Store::new() | (index + 1,s);
                },
            };
        }
    }
}
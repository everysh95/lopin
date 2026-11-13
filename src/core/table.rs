use std::{collections::HashMap, ops::{BitAnd, BitOr, ShlAssign}, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{Store, TableAnyValue::{FloatValue, IntValue, StringValue, UintValue}, core::store::op_fn, store, op_values};


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


#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum TableAnyValue {
    IntValue(i64),
    UintValue(u64),
    FloatValue(f64),
    StringValue(String)
}

impl Table {
    pub fn select(self: &Self, id: usize) -> Store<Option<TableAnyValue>> {
        let mut values: Store<Option<TableAnyValue>> = store();
        for sid in self.int_store.get_ids() {
            values = values | (sid,self.int_store.select(sid).unwrap().select(id).and_then(|v| Some(TableAnyValue::IntValue(v))))
        }
        for sid in self.uint_store.get_ids() {
            values = values | (sid,self.uint_store.select(sid).unwrap().select(id).and_then(|v| Some(TableAnyValue::UintValue(v))))
        }
        for sid in self.float_store.get_ids() {
            values = values | (sid,self.float_store.select(sid).unwrap().select(id).and_then(|v| Some(TableAnyValue::FloatValue(v))))
        }
        for sid in self.str_store.get_ids() {
            values = values | (sid,self.str_store.select(sid).unwrap().select(id).and_then(|v| Some(TableAnyValue::StringValue(v))))
        }
        for sid in (0..self.max_sid).map(|i| i + 1) {
            if values.get_ids().iter().all(|ri| ri != &sid) {
                values = values | (sid,None)
            }
        }
        values
    }
    pub fn view(self: &Self) -> Store<Store<Option<TableAnyValue>>> {
        let mut vtable : Store<Store<Option<TableAnyValue>>> = store();
        for id in (0..self.max_id).map(|i| i + 1) {
            vtable = vtable | (id, self.select(id))
        }
        vtable
    }
}

pub struct OpSelect(usize);

pub fn select(id: usize) -> OpSelect{
    OpSelect(id)
}

impl BitAnd<OpSelect> for Table {
    type Output = Store<Option<TableAnyValue>>;
    fn bitand(self, rhs: OpSelect) -> Self::Output {
        let id =rhs.0;
        let mut values: Store<Option<TableAnyValue>> = store();
        for sid in self.int_store.get_ids() {
            values = values | (sid,self.int_store.select(sid).unwrap().select(id).and_then(|v| Some(TableAnyValue::IntValue(v))))
        }
        for sid in self.uint_store.get_ids() {
            values = values | (sid,self.uint_store.select(sid).unwrap().select(id).and_then(|v| Some(TableAnyValue::UintValue(v))))
        }
        for sid in self.float_store.get_ids() {
            values = values | (sid,self.float_store.select(sid).unwrap().select(id).and_then(|v| Some(TableAnyValue::FloatValue(v))))
        }
        for sid in self.str_store.get_ids() {
            values = values | (sid,self.str_store.select(sid).unwrap().select(id).and_then(|v| Some(TableAnyValue::StringValue(v))))
        }
        for sid in (0..self.max_sid).map(|i| i + 1) {
            if values.get_ids().iter().all(|ri| ri != &sid) {
                values = values | (sid,None)
            }
        }
        values
    }
}

impl BitAnd<Table> for OpSelect {
    type Output = Store<Option<TableAnyValue>>;
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    } 
}

pub struct OpView;

pub fn view() -> OpView {
    OpView
}

impl BitAnd<OpView> for Table {
    type Output = Store<Store<Option<TableAnyValue>>>;

    fn bitand(self, _rhs : OpView) -> Self::Output {
        let mut vtable : Store<Store<Option<TableAnyValue>>> = store();
        for id in (0..self.max_id).map(|i| i + 1) {
            vtable = vtable | (id, self.select(id))
        }
        vtable
    }
}

impl BitAnd<Table> for OpView {
    type Output = Store<Store<Option<TableAnyValue>>>;
    
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpIntStore;

pub fn int_store() -> OpIntStore {
    OpIntStore
}

impl BitAnd<OpIntStore> for Table {
    type Output = Store<Store<i64>>;
    fn bitand(self, _ : OpIntStore) -> Self::Output {
        self.int_store.clone()
    }
}

impl BitAnd<Table> for OpIntStore {
    type Output = Store<Store<i64>>;
    fn bitand(self, rhs : Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpUintStore;

pub fn uint_store() -> OpUintStore {
    OpUintStore
}


impl BitAnd<OpUintStore> for Table {
    type Output = Store<Store<u64>>;
    fn bitand(self, _ : OpUintStore) -> Self::Output {
        self.uint_store.clone()
    }
}

impl BitAnd<Table> for OpUintStore {
    type Output = Store<Store<u64>>;
    fn bitand(self, rhs : Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpFloatStore;

pub fn float_store() -> OpFloatStore {
    OpFloatStore
}

impl BitAnd<OpFloatStore> for Table {
    type Output = Store<Store<f64>>;
    fn bitand(self, _ : OpFloatStore) -> Self::Output {
        self.float_store.clone()
    }
}

impl BitAnd<Table> for OpFloatStore {
    type Output = Store<Store<f64>>;
    fn bitand(self, rhs : Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpStrStore;

pub fn str_store() -> OpStrStore {
    OpStrStore
}

impl BitAnd<OpStrStore> for Table {
    type Output = Store<Store<String>>;
    fn bitand(self, _ : OpStrStore) -> Self::Output {
        self.str_store.clone()
    }
}

impl BitAnd<Table> for OpStrStore {
    type Output = Store<Store<String>>;
    fn bitand(self, rhs : Table) -> Self::Output {
        rhs & self
    }
}



pub fn to_json_value(v: Option<TableAnyValue>) -> serde_json::Value {
    match v {
        Some(v) => match v {
            IntValue(v) => serde_json::Value::Number(v.into()),
            UintValue(v) => serde_json::Value::Number(v.into()),
            FloatValue(v) => serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap()),
            StringValue(v) => serde_json::Value::String(v.clone()),
        },
        None => serde_json::Value::Null
    }
}

pub fn json_value() -> Arc<dyn Fn(Option<TableAnyValue>) -> serde_json::Value>
{
    op_fn(|v| to_json_value(v))
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
        for id in rhs.get_ids() {
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

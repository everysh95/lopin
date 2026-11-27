use std::
    ops::{BitAnd, BitOr, BitOrAssign, ShlAssign, }
;

use crate::{
    Store, TableAnyValue::{FloatValue, IntValue, StringValue, UintValue}, op_set, op_where, store,
};

#[derive(Debug, Clone)]
pub struct Table {
    table_name: String,
    int_store: Store<Store<i64>>,
    uint_store: Store<Store<u64>>,
    float_store: Store<Store<f64>>,
    str_store: Store<Store<String>>,
    col_name: Store<String>,
    max_sid: usize,
    max_id: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TableAnyValue {
    IntValue(i64),
    UintValue(u64),
    FloatValue(f64),
    StringValue(String),
}

impl PartialEq<i64> for TableAnyValue {
    fn eq(&self, other: &i64) -> bool {
        match self.clone() {
           IntValue(v)  => &v == other,
           _ => false
        }
    }
}
impl PartialEq<u64> for TableAnyValue {
    fn eq(&self, other: &u64) -> bool {
        match self.clone() {
           UintValue(v)  => &v == other,
           _ => false
        }
    }
}
impl PartialEq<f64> for TableAnyValue {
    fn eq(&self, other: &f64) -> bool {
        match self.clone() {
           FloatValue(v)  => &v == other,
           _ => false
        }
    }
}
impl PartialEq<String> for TableAnyValue {
    fn eq(&self, other: &String) -> bool {
        match self.clone() {
           StringValue(v)  => &v == other,
           _ => false
        }
    }
}

impl PartialEq<&str> for TableAnyValue {
    fn eq(&self, other: &&str) -> bool {
        self == &other.to_string()
    }
}

impl Table {
    pub fn select(self: &Self, id: usize) -> Store<TableAnyValue> {
        let mut values: Store<TableAnyValue> = store();
        for sid in self.int_store.get_ids() {
            values = values
                | (
                    sid,
                    IntValue(
                        self.int_store
                        .select(sid)
                        .unwrap()
                        .select(id)
                        .unwrap(),
                    )
                )
        }
        for sid in self.uint_store.get_ids() {
            values = values
                | (
                    sid,
                    UintValue(
                        self.uint_store
                        .select(sid)
                        .unwrap()
                        .select(id)
                        .unwrap(),
                    )
                )
        }
        for sid in self.float_store.get_ids() {
            values = values
                | (
                    sid,
                    TableAnyValue::FloatValue(
                        self.float_store
                        .select(sid)
                        .unwrap()
                        .select(id)
                        .unwrap(),
                    )
                )
        }
        for sid in self.str_store.get_ids() {
            values = values
                | (
                    sid,
                    TableAnyValue::StringValue(
                        self.str_store
                        .select(sid)
                        .unwrap()
                        .select(id)
                        .unwrap()
                    )
                )
        }
        values
    }
    pub fn view(self: &Self) -> Store<Store<TableAnyValue>> {
        let mut vtable: Store<Store<TableAnyValue>> = store();
        for id in (0..self.max_id).map(|i| i + 1) {
            vtable = vtable | (id, self.select(id))
        }
        vtable
    }
}

pub struct OpSelect(usize);

pub fn select(id: usize) -> OpSelect {
    OpSelect(id)
}

impl BitAnd<OpSelect> for Table {
    type Output = Store<TableAnyValue>;
    fn bitand(self, rhs: OpSelect) -> Self::Output {
        self.select(rhs.0)
    }
}

impl BitAnd<Table> for OpSelect {
    type Output = Store<TableAnyValue>;
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpView;

pub fn view() -> OpView {
    OpView
}

impl BitAnd<OpView> for Table {
    type Output = Store<Store<TableAnyValue>>;

    fn bitand(self, _rhs: OpView) -> Self::Output {
        self.view()
    }
}

impl BitAnd<Table> for OpView {
    type Output = Store<Store<TableAnyValue>>;

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
    fn bitand(self, _: OpIntStore) -> Self::Output {
        self.int_store.clone()
    }
}

impl BitAnd<Table> for OpIntStore {
    type Output = Store<Store<i64>>;
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpUintStore;

pub fn uint_store() -> OpUintStore {
    OpUintStore
}

impl BitAnd<OpUintStore> for Table {
    type Output = Store<Store<u64>>;
    fn bitand(self, _: OpUintStore) -> Self::Output {
        self.uint_store.clone()
    }
}

impl BitAnd<Table> for OpUintStore {
    type Output = Store<Store<u64>>;
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpFloatStore;

pub fn float_store() -> OpFloatStore {
    OpFloatStore
}

impl BitAnd<OpFloatStore> for Table {
    type Output = Store<Store<f64>>;
    fn bitand(self, _: OpFloatStore) -> Self::Output {
        self.float_store.clone()
    }
}

impl BitAnd<Table> for OpFloatStore {
    type Output = Store<Store<f64>>;
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpStrStore;

pub fn str_store() -> OpStrStore {
    OpStrStore
}

impl BitAnd<OpStrStore> for Table {
    type Output = Store<Store<String>>;
    fn bitand(self, _: OpStrStore) -> Self::Output {
        self.str_store.clone()
    }
}

impl BitAnd<Table> for OpStrStore {
    type Output = Store<Store<String>>;
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    }
}

pub struct OpColNames;

pub fn col_names() -> OpColNames {
    OpColNames
}

impl BitAnd<OpColNames> for Table {
    type Output = Store<String>;
    fn bitand(self, _rhs: OpColNames) -> Self::Output {
        self.col_name.clone()
    }
}

impl BitAnd<Table> for OpColNames {
    type Output = Store<String>;
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    }
}
pub struct OpTableName;

pub fn table_name() -> OpTableName {
    OpTableName
}

impl BitAnd<OpTableName> for Table {
    type Output = String;
    fn bitand(self, _rhs: OpTableName) -> Self::Output {
        self.table_name.clone()
    }
}

impl BitAnd<Table> for OpTableName {
    type Output = String;
    fn bitand(self, rhs: Table) -> Self::Output {
        rhs & self
    }
}

impl BitOr<Store<Store<i64>>> for Table {
    type Output = Table;
    fn bitor(self, rhs: Store<Store<i64>>) -> Self::Output {
        let mut res = self;
        res.max_sid = if res.max_sid < rhs.get_max_id() {
            rhs.get_max_id()
        } else {
            res.max_sid
        };
        for id in rhs.get_ids() {
            res.int_store = res.int_store | (id, rhs.select(id).unwrap());
            res.max_id = if res.max_id < rhs.select(id).unwrap().get_max_id() {
                rhs.get_max_id()
            } else {
                res.max_id
            };
        }
        res
    }
}

impl BitOr<Store<Store<u64>>> for Table {
    type Output = Table;
    fn bitor(self, rhs: Store<Store<u64>>) -> Self::Output {
        let mut res = self;
        res.max_sid = if res.max_sid < rhs.get_max_id() {
            rhs.get_max_id()
        } else {
            res.max_sid
        };
        for id in rhs.get_ids() {
            res.uint_store = res.uint_store | (id, rhs.select(id).unwrap());
            res.max_id = if res.max_id < rhs.select(id).unwrap().get_max_id() {
                rhs.get_max_id()
            } else {
                res.max_id
            };
        }
        res
    }
}

impl BitOr<Store<Store<f64>>> for Table {
    type Output = Table;
    fn bitor(self, rhs: Store<Store<f64>>) -> Self::Output {
        let mut res = self;
        res.max_sid = if res.max_sid < rhs.get_max_id() {
            rhs.get_max_id()
        } else {
            res.max_sid
        };
        for id in rhs.get_ids() {
            res.float_store = res.float_store | (id, rhs.select(id).unwrap());
            res.max_id = if res.max_id < rhs.select(id).unwrap().get_max_id() {
                rhs.get_max_id()
            } else {
                res.max_id
            };
        }
        res
    }
}

impl BitOr<Store<Store<String>>> for Table {
    type Output = Table;
    fn bitor(self, rhs: Store<Store<String>>) -> Self::Output {
        let mut res = self;
        res.max_sid = if res.max_sid < rhs.get_max_id() {
            rhs.get_max_id()
        } else {
            res.max_sid
        };
        for id in rhs.get_ids() {
            res.str_store = res.str_store | (id, rhs.select(id).unwrap());
            res.max_id = if res.max_id < rhs.select(id).unwrap().get_max_id() {
                rhs.get_max_id()
            } else {
                res.max_id
            };
        }
        res
    }
}

impl BitOr<Store<String>> for Table {
    type Output = Table;
    fn bitor(self, rhs: Store<String>) -> Self::Output {
        let mut res = self;
        res.max_sid = if res.max_sid < rhs.get_max_id() {
            rhs.get_max_id()
        } else {
            res.max_sid
        };
        for id in rhs.get_ids() {
            res.col_name = res.col_name | (id, rhs.select(id).unwrap());
        }
        res
    }
}

impl BitOr<(String, Store<i64>)> for Table {
    type Output = Table;
    fn bitor(self, rhs: (String, Store<i64>)) -> Self::Output {
        let (name, store) = rhs;
        let mut result = self;
        result.max_sid = result.max_sid + 1;
        result.col_name = result.col_name | (result.max_sid, name);
        result.int_store = result.int_store | (result.max_sid, store);
        result
    }
}

impl BitOr<(String, Store<u64>)> for Table {
    type Output = Table;
    fn bitor(self, rhs: (String, Store<u64>)) -> Self::Output {
        let (name, store) = rhs;
        let mut result = self;
        result.max_sid = result.max_sid + 1;
        result.col_name = result.col_name | (result.max_sid, name);
        result.uint_store = result.uint_store | (result.max_sid, store);
        result
    }
}

impl BitOr<(String, Store<f64>)> for Table {
    type Output = Table;
    fn bitor(self, rhs: (String, Store<f64>)) -> Self::Output {
        let (name, store) = rhs;
        let mut result = self;
        result.max_sid = result.max_sid + 1;
        result.col_name = result.col_name | (result.max_sid, name);
        result.float_store = result.float_store | (result.max_sid, store);
        result
    }
}

impl BitOr<(String, Store<String>)> for Table {
    type Output = Table;
    fn bitor(self, rhs: (String, Store<String>)) -> Self::Output {
        let (name, store) = rhs;
        let mut result = self;
        result.max_sid = result.max_sid + 1;
        result.col_name = result.col_name | (result.max_sid, name);
        result.str_store = result.str_store | (result.max_sid, store);
        result
    }
}

pub fn table(name: &str) -> Table {
    Table {
        table_name: name.to_string(),
        int_store: Store::new(),
        uint_store: Store::new(),
        float_store: Store::new(),
        str_store: Store::new(),
        col_name: Store::new(),
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

impl BitOr<(&str, i64)> for Store<(String, TableAnyValue)> {
    type Output = Store<(String, TableAnyValue)>;
    fn bitor(self, rhs: (&str, i64)) -> Self::Output {
        let (k, v) = rhs;
        self | Some((k.to_string(), TableAnyValue::IntValue(v)))
    }
}
impl BitOr<(&str, u64)> for Store<(String, TableAnyValue)> {
    type Output = Store<(String, TableAnyValue)>;
    fn bitor(self, rhs: (&str, u64)) -> Self::Output {
        let (k, v) = rhs;
        self | Some((k.to_string(), TableAnyValue::UintValue(v)))
    }
}

impl BitOr<(&str, f64)> for Store<(String, TableAnyValue)> {
    type Output = Store<(String, TableAnyValue)>;
    fn bitor(self, rhs: (&str, f64)) -> Self::Output {
        let (k, v) = rhs;
        self | Some((k.to_string(), TableAnyValue::FloatValue(v)))
    }
}

impl BitOr<(&str, &str)> for Store<(String, TableAnyValue)> {
    type Output = Store<(String, TableAnyValue)>;
    fn bitor(self, rhs: (&str, &str)) -> Self::Output {
        let (k, v) = rhs;
        self | Some((k.to_string(), TableAnyValue::StringValue(v.to_string())))
    }
}

impl BitOrAssign<Store<(String, TableAnyValue)>> for Table {
    fn bitor_assign(&mut self, rhs: Store<(String, TableAnyValue)>) {
        self.max_id = self.max_id + 1;
        for index in 0..rhs.get_max_id() {
            let kv = rhs.select(index + 1).unwrap();
            let (name, value) = kv;
            match value {
                TableAnyValue::IntValue(v) => {
                    self.int_store <<= self.col_name.clone()
                        & op_where(|s| &s == &name)
                        & self.int_store.clone()
                        << op_set(|s| s | (self.max_id, v.clone()));
                }
                TableAnyValue::UintValue(v) => {
                    self.uint_store <<= self.col_name.clone()
                        & op_where(|s| &s == &name)
                        & self.uint_store.clone()
                        << op_set(|s| s | (self.max_id, v.clone()));
                }
                TableAnyValue::FloatValue(v) => {
                    self.float_store <<= self.col_name.clone()
                        & op_where(|s| &s == &name)
                        & self.float_store.clone()
                        << op_set(|s| s | (self.max_id, v.clone()));
                }
                TableAnyValue::StringValue(v) => {
                    self.str_store <<= self.col_name.clone()
                        & op_where(|s| &s == &name)
                        & self.str_store.clone()
                        << op_set(|s| s | (self.max_id, v.clone()));
                }
            };
        }
    }
}

impl BitOr<Store<(String, TableAnyValue)>> for Table {
    type Output = Table;
    fn bitor(self, rhs: Store<(String, TableAnyValue)>) -> Self::Output {
        let mut res = self;
        res |= rhs;
        res
    }
}

impl BitOr<i64> for Store<TableAnyValue> {
    type Output = Store<TableAnyValue>;
    fn bitor(self, rhs: i64) -> Self::Output {
        self | Some(TableAnyValue::IntValue(rhs))
    }
}
impl BitOr<u64> for Store<TableAnyValue> {
    type Output = Store<TableAnyValue>;
    fn bitor(self, rhs: u64) -> Self::Output {
        self | Some(TableAnyValue::UintValue(rhs))
    }
}

impl BitOr<f64> for Store<TableAnyValue> {
    type Output = Store<TableAnyValue>;
    fn bitor(self, rhs: f64) -> Self::Output {
        self | Some(TableAnyValue::FloatValue(rhs))
    }
}

impl BitOr<&str> for Store<TableAnyValue> {
    type Output = Store<TableAnyValue>;
    fn bitor(self, rhs: &str) -> Self::Output {
        self | Some(TableAnyValue::StringValue(rhs.to_string()))
    }
}

impl BitOrAssign<Store<TableAnyValue>> for Table {
    fn bitor_assign(&mut self, rhs: Store<TableAnyValue>) {
        self.max_id = self.max_id + 1;
        for id in rhs.get_ids() {
            let value = rhs.select(id).unwrap();
            match value {
                TableAnyValue::IntValue(v) => {
                    let mut s = self.int_store.select(id).unwrap().clone();
                    s = s | (self.max_id, v.clone());
                    self.int_store <<= Store::new() | (id, s);
                }
                TableAnyValue::UintValue(v) => {
                    let mut s = self.uint_store.select(id).unwrap().clone();
                    s = s | (self.max_id, v.clone());
                    self.uint_store <<= Store::new() | (id, s);
                }
                TableAnyValue::FloatValue(v) => {
                    let mut s = self.float_store.select(id).unwrap().clone();
                    s = s | (self.max_id, v.clone());
                    self.float_store <<= Store::new() | (id, s);
                }
                TableAnyValue::StringValue(v) => {
                    let mut s = self.str_store.select(id).unwrap().clone();
                    s = s | (self.max_id, v.clone());
                    self.str_store <<= Store::new() | (id, s);
                }
            };
        }
    }
}

impl BitOr<Store<TableAnyValue>> for Table {
    type Output = Table;
    fn bitor(self, rhs: Store<TableAnyValue>) -> Self::Output {
        let mut res = self;
        res |= rhs;
        res
    }
}

impl ShlAssign<Store<Store<TableAnyValue>>> for Table {
    fn shl_assign(&mut self, rhs: Store<Store<TableAnyValue>>) {
        for id in rhs.get_ids() {
            let row = rhs.select(id).unwrap();
            for sid in row.get_ids() {
                let value = row.select(sid).unwrap();
                match value {
                    TableAnyValue::IntValue(v) => {
                        let mut s = self.int_store.select(sid).unwrap().clone();
                        s <<= store() | (id, v.clone());
                        self.int_store <<= Store::new() | (sid, s);
                    }
                    TableAnyValue::UintValue(v) => {
                        let mut s = self.uint_store.select(sid).unwrap().clone();
                        s <<= store() | (id, v.clone());
                        self.uint_store <<= Store::new() | (sid, s);
                    }
                    TableAnyValue::FloatValue(v) => {
                        let mut s = self.float_store.select(sid).unwrap().clone();
                        s <<= store() | (id, v.clone());
                        self.float_store <<= Store::new() | (sid, s);
                    }
                    TableAnyValue::StringValue(v) => {
                        let mut s = self.str_store.select(sid).unwrap().clone();
                        s <<= store() | (id, v.clone());
                        self.str_store <<= Store::new() | (sid, s);
                    }
                }
            };
        }
    }
}
use std::{fs::{File, create_dir_all}, io::{Read, Write}, ops::{Shl, Shr}, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    Store, Table, TableAnyValue::{self, FloatValue, IntValue, StringValue, UintValue}, col_names, float_store, int_store, json::{OpJsonLoad, OpJsonSave, json_save}, op_fn, str_store, table, uint_store,
};

#[derive(Debug, Deserialize, Serialize)]
struct Helper {
    int_store: Store<Store<i64>>,
    uint_store: Store<Store<u64>>,
    float_store: Store<Store<f64>>,
    str_store: Store<Store<String>>,
    col_name: Store<String>,
}

pub fn to_json_value(v: TableAnyValue) -> serde_json::Value {
    match v {
        IntValue(v) => serde_json::Value::Number(v.into()),
        UintValue(v) => serde_json::Value::Number(v.into()),
        FloatValue(v) => serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap()),
        StringValue(v) => serde_json::Value::String(v.clone()),
    }
}

pub fn json_value() -> Arc<dyn Fn(TableAnyValue) -> serde_json::Value> {
    op_fn(|v| to_json_value(v))
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let kv = Helper::deserialize(deserializer)?;
        let t =
            table("") | kv.col_name | kv.int_store | kv.uint_store | kv.float_store | kv.str_store;
        Ok(t)
    }
}

impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kv: Helper = Helper {
            int_store: self.clone() & int_store(),
            uint_store: self.clone() & uint_store(),
            float_store: self.clone() & float_store(),
            str_store: self.clone() & str_store(),
            col_name: self.clone() & col_names(),
        };
        kv.serialize(serializer)
    }
}

impl Shr<OpJsonSave> for Table {
    type Output = Table;
    fn shr(self, rhs: OpJsonSave) -> Self::Output {
        if let Ok(mut file) = File::create(rhs.path) {
            if let Ok(text) = serde_json::to_string(&self) {
                let _ = file.write(text.as_bytes());
            }
        }
        self
    }
}

impl Shl<OpJsonLoad> for Table {
    type Output = Table;
    fn shl(self, rhs: OpJsonLoad) -> Self::Output {
        if let Ok(mut file) = File::open(rhs.path) {
            let mut text = String::new();
            let _ = file.read_to_string(&mut text);
            let ps : Result<Table,_> = serde_json::from_str(&text);
            return match ps {
                Ok(s) => self
                    | (s.clone() & col_names())
                    | (s.clone() & int_store())
                    | (s.clone() & uint_store())
                    | (s.clone() & float_store())
                    | (s.clone() & str_store()),
                Err(_) => self
            };
        }
        self
    }
}

pub struct OpJsonSaveDir {
    path: String
}

pub fn json_save_dir(path: &str) -> OpJsonSaveDir {
    OpJsonSaveDir { path: path.to_string() }
}


impl Shr<OpJsonSaveDir> for Table {
    type Output = Table;
    fn shr(self, rhs: OpJsonSaveDir) -> Self::Output {
        if create_dir_all(rhs.path.clone()).is_ok() {
            let tis = self.clone() & int_store();
            for sid in tis.get_ids() {
                let path = format!("{}/i-{}.json",rhs.path,sid);
                let _ = tis.select(sid).unwrap() >> json_save(&path) ;
            }
            let tus = self.clone() & uint_store();
            for sid in tus.get_ids() {
                let path = format!("{}/u-{}.json",rhs.path,sid);
                let _ = tus.select(sid).unwrap() >> json_save(&path) ;
            }
            let tfs = self.clone() & float_store();
            for sid in tfs.get_ids() {
                let path = format!("{}/f-{}.json",rhs.path,sid);
                let _ = tfs.select(sid).unwrap() >> json_save(&path) ;
            }
            let tss = self.clone() & str_store();
            for sid in tss.get_ids() {
                let path = format!("{}/s-{}.json",rhs.path,sid);
                let _ = tss.select(sid).unwrap() >> json_save(&path) ;
            }
            {
                let path = format!("{}/names.json",rhs.path);
                let _ = (self.clone() & col_names()) >> json_save(&path);
            }
        }
        self
    }
}
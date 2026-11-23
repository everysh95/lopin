use serde_json::to_string;

use crate::core::db::database;

use super::*;

#[test]
fn it_works_01() {
  let t1 = table("test") | col_float("test01") | col_int("test02") | col_str("test03");
  let t2 = table("test2") | col_float("test01") | col_int("test02") | col_str("test03");
  let db = database() | t1 | t2;
  let mut tx = db.clone() & "test";
  tx |= store() | 15.0f64 | 17i64 | "32";
  let mut tx2 = db.clone() & "test2";
  tx2 |= store() | 16.0f64 | 18i64 | "32";
  assert_ne!(to_string(&tx).unwrap(),to_string(&tx2).unwrap());
}

#[test]
fn it_works_02() {
  let t1 = table("test") | col_float("test01") | col_int("test02") | col_str("test03");
  let t2 = table("test2") | col_float("test01") | col_int("test02") | col_str("test03");
  let mut db = database() | t1 | t2;
  db <<= (db.clone() & "test") | (store() | 15.0f64 | 17i64 | "32");
  db <<= (db.clone() & "test2") | (store() | 16.0f64 | 18i64 | "32");
  assert_ne!(to_string(&(db.clone() & "test")).unwrap(),to_string(&(db.clone() & "test2")).unwrap());
}

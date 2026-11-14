
use std::assert_eq;


use crate::json::store::{self, json_load, json_save};

// use super::*;
use super::super::*;

#[test]
fn it_works_01() {
    let text = r#"[[1,"test1"],[2,"test2"],[3,"test3"],[4,"test3"]]"#;
    let s2: Store<String> = serde_json::from_str(text).unwrap();
    let res_text = serde_json::to_string(&s2).unwrap();
    assert_eq!(text,res_text);
}

#[test]
fn it_works_02() {
    let s2: Store<String> = Store::new()
    | Some("test01".to_string())
    | Some("test02".to_string())
    | Some("test03".to_string())
    ;
    let _ss = s2 >> json_save("./testdoc/test.json");
    let _s3 : Store<String> = store() << json_load("./testdoc/test.json");
}

#[test]
fn it_works_03() {
    let s2: Store<Store<String>> = Store::new()
    | Some(Store::new() | Some("test01".to_string()) | Some("test02".to_string()) | Some("test03".to_string()))
    | Some(Store::new() | Some("test01-02".to_string()) | Some("test02-02".to_string()))
    ;
    let _ss = s2 >> json_save("./testdoc/test.json");
    let _s3 : Store<Store<String>> = store() << json_load("./testdoc/test2.json");
}

#[test]
fn it_works_04() {
    let mut t = table() | col_int("test") | col_float("test2") | col_str("test3");
    t <<= store() | 123i64 | 4.56f64 | "test7";
    t <<= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    let _ss = t >> json_save("./testdoc/test3.json");
    let _t2 = table() << json_load("./testdoc/test3.json");
}

#[test]
fn it_works_05() {
    let mut t = table() | col_int("test") | col_float("test2") | col_str("test3");
    t <<= store() | 123i64 | 4.56f64 | "test7";
    t <<= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    let _ = (t.clone() & int_store()) >> json_save("./testdoc/test3_int.json");
    let _ = (t.clone() & uint_store()) >> json_save("./testdoc/test3_uint.json");
    let _ = (t.clone() & float_store()) >> json_save("./testdoc/test3_float.json");
    let _ = (t.clone() & str_store()) >> json_save("./testdoc/test3_str.json");
    let _ = (t.clone() & col_names()) >> json_save("./testdoc/test3_names.json");
    let si : Store<Store<i64>> = store() << json_load("./testdoc/test3_int.json");
    let su : Store<Store<u64>> = store() << json_load("./testdoc/test3_uint.json");
    let sf : Store<Store<f64>> = store() << json_load("./testdoc/test3_float.json");
    let ss : Store<Store<String>> = store() << json_load("./testdoc/test3_str.json");
    let sn : Store<String> = store() << json_load("./testdoc/test3_names.json");
    let _t2 = table() | si | su | sf | ss | sn;
}

use std::{assert_eq, println};

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
    let ss = s2.write_json_file("./testdoc/test.json").unwrap();
    let s3 : Store<String> = Store::read_json_file("./testdoc/test.json").unwrap();
}

#[test]
fn it_works_03() {
    let s2: Store<Store<String>> = Store::new()
    | Some(Store::new() | Some("test01".to_string()) | Some("test02".to_string()) | Some("test03".to_string()))
    | Some(Store::new() | Some("test01-02".to_string()) | Some("test02-02".to_string()))
    ;
    let ss = s2.write_json_file("./testdoc/test2.json").unwrap();
    let s3 : Store<String> = Store::read_json_file("./testdoc/test2.json").unwrap();
}
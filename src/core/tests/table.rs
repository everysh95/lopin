use serde_json::to_string;

use crate::{TableAnyValue::StringValue, core::store::{op_map, op_values}, json::table::json_value};

use super::*;

#[test]
fn it_works_01() {
    let mut t = table("test") | col_int("test") | col_float("test2") | col_str("test3");
    t |= store() | 123i64 | 4.56f64 | "test7";
    t |= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    // println!("{}",to_string(&t).unwrap());
}


#[test]
fn it_works_02() {
    let mut t = table("test") | col_int("test") | col_float("test2") | col_str("test3");
    t |= store() | 123i64 | 4.56f64 | "test7";
    t |= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    let vt = (t & view()) << op_map(json_value()) << map_values() & op_values();
    assert_eq!(to_string(&vt).unwrap(),"[[123,4.56,\"test7\"],[456,12.3,\"test7\"]]");
}

#[test]
fn it_works_03() {
    let mut t = table("test") | col_int("test") | col_float("test2") | col_str("test3");
    t |= store() | 123i64 | 4.56f64 | "test7";
    t |= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    let ti = t.clone() & int_store();
    let tu = t.clone() & uint_store();
    let tf  = t.clone() & float_store();
    let ts = t.clone() & str_store();
    let tc = t.clone() & col_names();
    let t2 = table("test") | ti | tu | tf | ts | tc;
    let vt = t.view() << op_map(json_value()) << map_values() & op_values();
    let vt2 = t2.view() << op_map(json_value()) << map_values() & op_values();
    assert_eq!(to_string(&vt).unwrap(),to_string(&vt2).unwrap());
}

#[test]
fn it_works_04() {
    let mut t = table("test") | col_int("test") | col_float("test2") | col_str("test3");
    t |= store() | 123i64 | 4.56f64 | "test7";
    t |= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    t <<=  t.view() & op_where(|p: Store<TableAnyValue>| {
        p.select(3).is_some() && p.select(3).unwrap() == "test7"
    }) & t.view() << op_set(|r| r << (store() | (3,StringValue("test8".to_string()))));
    let vt = (t & view()) << op_map(json_value()) << map_values() & op_values();
    assert_eq!(to_string(&vt).unwrap(),"[[123,4.56,\"test8\"],[456,12.3,\"test8\"]]");
}
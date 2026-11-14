use serde_json::to_string;

use crate::core::store::{op_map, op_values};

use super::*;

#[test]
fn it_works_01() {
    let mut t = table() | col_int("test") | col_float("test2") | col_str("test3");
    t <<= store() | 123i64 | 4.56f64 | "test7";
    t <<= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    // println!("{}",to_string(&t).unwrap());
}


#[test]
fn it_works_02() {
    let mut t = table() | col_int("test") | col_float("test2") | col_str("test3");
    t <<= store() | 123i64 | 4.56f64 | "test7";
    t <<= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    let vt = (t & view()) << op_map(json_value()) << map_values() & op_values();
    assert_eq!(to_string(&vt).unwrap(),"[[123,4.56,\"test7\"],[456,12.3,\"test7\"]]");
}

#[test]
fn it_works_03() {
    let mut t = table() | col_int("test") | col_float("test2") | col_str("test3");
    t <<= store() | 123i64 | 4.56f64 | "test7";
    t <<= store() | ("test2",12.3f64) | ("test3","test7") | ("test",456i64);
    let ti = t.clone() & int_store();
    let tu = t.clone() & uint_store();
    let tf  = t.clone() & float_store();
    let ts = t.clone() & str_store();
    let tc = t.clone() & col_names();
    let t2 = table() | ti | tu | tf | ts | tc;
    let vt = t.view() << op_map(json_value()) << map_values() & op_values();
    let vt2 = t2.view() << op_map(json_value()) << map_values() & op_values();
    assert_eq!(to_string(&vt).unwrap(),to_string(&vt2).unwrap());
}

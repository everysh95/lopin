use super::*;

#[test]
fn it_works_01() {
    let mut s1: Store<String> = Store::new();
    let mut s2: Store<String> = Store::new();
    s1.insert_next(&String::from("test01"));
    s1.insert_next(&String::from("test02"));
    s1.insert_next(&String::from("test03"));

    s2.insert_next(&String::from("test01-02"));
    s2.insert_next(&String::from("test02-02"));
    s2.insert_next(&String::from("test03-02"));

    let s3 = s1.clone() & op_where(|x| x == "test02") & s2.clone()
        ^ op_set(|_v| String::from("testtest"))
        ^ s2.clone();
    assert_eq!(s3.select(2), Some(String::from("testtest")));
}

#[test]
fn it_works_02() {
    let mut s1: Store<String> = Store::new();
    let mut s2: Store<String> = Store::new();
    s1.insert_next(&String::from("test01"));
    s1.insert_next(&String::from("test02"));
    s1.insert_next(&String::from("test03"));

    s2.insert_next(&String::from("test01"));
    s2.insert_next(&String::from("test02"));
    s2.insert_next(&String::from("test03"));

    for (r1, r2) in s1.get_values().iter().zip(s2.get_values().iter()) {
        assert_eq!(r1, r2);
    }
}

#[test]
fn it_works_03() {
    let mut s1: Store<String> = Store::new();
    let mut s2: Store<String> = Store::new();
    s1.insert_next(&String::from("test01"));
    s1.insert_next(&String::from("test02"));
    s1.insert_next(&String::from("test03"));

    s2.insert_next(&String::from("test01-02"));
    s2.insert_next(&String::from("test02-02"));
    s2.insert_next(&String::from("test03-02"));

    s2 ^= s1.clone() & op_where(|x| x == "test02") & s2.clone()
        ^ op_set(|_v| String::from("testtest"));
    assert_eq!(s2.select(1), Some(String::from("test01-02")));
    assert_eq!(s2.select(2), Some(String::from("testtest")));
    assert_eq!(s2.select(3), Some(String::from("test03-02")));
}

#[test]
fn it_works_04() {
    let mut s1: Store<String> = Store::new();
    let mut s2: Store<String> = Store::new();
    s1.insert_next(&String::from("test01"));
    s1.insert_next(&String::from("test02"));
    s1.insert_next(&String::from("test03"));

    s2.insert_next(&String::from("test01-02"));
    s2.insert_next(&String::from("test02-02"));
    s2.insert_next(&String::from("test03-02"));

    s2 ^= op_set(|_v| String::from("testtest"));
    assert_eq!(s2.select(1), Some(String::from("testtest")));
    assert_eq!(s2.select(2), Some(String::from("testtest")));
    assert_eq!(s2.select(3), Some(String::from("testtest")));
}

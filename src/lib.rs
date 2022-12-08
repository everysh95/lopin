//! # lopin - library of pipeline input
//!
//! `lopin` is a Query framework featuring a two-way pipeline and resources abstracted as stores. 
//!

// main module
mod store;

pub use self::store::*;
// addional module
pub mod file;
// pub mod test_util;
// pub mod http;

#[cfg(test)]
mod tests {

    use super::*;
    use super::file;

    #[test]
    fn it_basic() {
        let mut test_store = ValueStore::new(vec!["hello", "world"]);
        // create
        test_store <<= pull(&test_store) + "hoge";
        assert_eq!(test_store.pull(), vec!["hello", "world", "hoge"]);
        // update
        test_store <<= pull(&test_store) / Box::new(|&x| x != "hoge") + "huga";
        assert_eq!(test_store.pull(), vec!["hello", "world", "huga"]);
    }

    #[test]
    fn it_file() {
        let mut test_store = file::FileStore::new("./testdoc/test.json");
        // create
        test_store <<= ValueStore::new(vec![String::from("hoge")]);
        assert_eq!(test_store.pull(), vec!["hoge"]);
        // update
        test_store <<= pull(&test_store) / Box::new(|x| x != &String::from("hoge")) + pull(&test_store) / Box::new(|x| x == &String::from("hoge")) * Box::new(|_v| String::from("huga"));
        assert_eq!(test_store.pull(), vec!["huga"]);
    }
}

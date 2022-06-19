pub mod core;
pub mod propaty;
pub mod merge;
pub mod transport;
pub mod select;
pub mod test;
pub use crate::core::*;
pub use crate::propaty::*;
pub use crate::merge::*;
pub use crate::transport::*;
pub use crate::select::*;

#[cfg(test)]
mod tests {

    use super::*;
    use super::test::assert_eq_store;

    #[test]
    fn it_basic() {
        let mut pipe = store("hoge".to_string()) & select(&"hoge".to_string()) ^ named("hoge") | assert_eq_store("hoge", "") ^ named("huga");
        pipe <<= transport("hoge", "huga");
    }
    #[test]
    fn it_swap() {
        let mut pipe = store("hoge".to_string()) ^ named("hoge") | assert_eq_store("hoge", "") ^ named("huga");
        pipe <<= swap("hoge", "huga");
    }
    #[test]
    fn it_prop() {
        let prop = create_propaty(store(10) ^ named("num") | store("text".to_string()) ^ named("text"));
        let mut pipe = store(prop.clone()) ^ named("from") | assert_eq_store(prop.clone(), vec![]) ^ named("to");
        pipe <<= transport("from", "to");
    }
}

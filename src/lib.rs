pub mod core;
pub mod propaty;
pub mod merge;
pub mod transport;
pub mod test;
pub use crate::core::*;
pub use crate::propaty::*;
pub use crate::merge::*;
pub use crate::transport::*;

#[cfg(test)]
mod tests {

    use super::store;
    use super::named;
    use super::transport;
    use super::swap;
    use super::test::assert_eq_store;

    #[test]
    fn it_basic() {
        let mut pipe = store("hoge".to_string()) & Box::new(|x| x == "hoge") ^ named("hoge") | assert_eq_store("hoge", "") ^ named("huga");
        pipe <<= transport("hoge", "huga");
    }
    #[test]
    fn it_swap() {
        let mut pipe = store("hoge".to_string()) & Box::new(|x| x == "hoge") ^ named("hoge") | assert_eq_store("hoge", "") ^ named("huga");
        pipe <<= swap("hoge", "huga");
    }
}

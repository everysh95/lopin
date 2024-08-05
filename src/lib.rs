//! # lopin - library of pipeline input
//!
//! `lopin` is a Query framework featuring a two-way pipeline and resources abstracted as stores. 
//!

mod core;
mod async_core;
pub mod util;
pub mod testing;
pub mod json;
pub mod http_server;
pub mod command;

pub use core::*;
pub use async_core::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        testing::test_ok(10, 10) ^ filter(|_| true, 0);
    }
    #[test]
    fn it_works2() {
        testing::test_error(10, 0) ^ filter(|_| false, 0);
    }
    #[test]
    fn it_works3() {
        testing::test_ok::<_,_,String>(10, "10".to_string()) ^ pipeline(|v : i32| Ok(v.to_string()));
    }
    #[test]
    fn it_works4() {
        testing::test_error::<_,i32,_>(10, 0) ^ pipeline(|_| Err(0));
    }
    #[test]
    fn it_works5() {
        let pipeline  = filter(|_| true, 1) & filter(|_| true, 2);
        testing::test_ok(10, 10) ^ pipeline;
    }
    #[test]
    fn it_works6() {
        let pipeline  = filter(|_| false, 1) & filter(|_| true, 2);
        testing::test_error(10, 1) ^ pipeline;
    }
    #[test]
    fn it_works7() {
        let pipeline  = filter(|_| true, 1) & filter(|_| false, 2);
        testing::test_error(10, 2) ^ pipeline;
    }
    #[test]
    fn it_works8() {
        let pipeline  = filter(|_| true, 1) | filter(|_| false, 2);
        testing::test_ok(10, 10) ^ pipeline;
    }
    #[test]
    fn it_works9() {
        let pipeline  = filter(|_| true, 1) | filter(|_| false, 2);
        testing::test_ok(10, 10) ^ pipeline;
    }
    #[test]
    fn it_works10() {
        let pipeline  = filter(|v| v % 2 == 0, 1) & pipeline(|v| Ok(v / 2)) | pipeline(|v| Ok(v) );
        testing::test_ok(10, 5) ^ pipeline;
    }
    #[test]
    fn it_works11() {
        let pipeline  = filter(|v| v % 2 == 0, 1) & pipeline(|v| Ok(v / 2)) | pipeline(|v| Ok(v) );
        testing::test_ok(9, 9) ^ pipeline;
    }
}

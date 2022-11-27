//! # lopin - library of pipeline input
//!
//! `lopin` is a Web API framework featuring a two-way pipeline and resources abstracted as stores. 
//!

// main module
mod puller;
mod pusher;
mod store;

pub use self::puller::*;
pub use self::pusher::*;
pub use self::store::*;
// addional module
pub mod console;
pub mod test_util;

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn it_basic() {
        let mut pusher = test_util::direct("test") ^ test_util::use_value(None) & test_util::expect_eq("test");
        // pusher.awake().await;
        pusher.awake().await;
    }
}

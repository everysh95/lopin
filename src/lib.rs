//! # lopin - library of pipeline input
//!
//! `lopin` is a Web API framework featuring a two-way pipeline and resources abstracted as stores. 
//!

// main module
mod puller;
mod pusher;
mod store;
mod in_memory;

pub use self::puller::*;
pub use self::pusher::*;
pub use self::store::*;
pub use self::in_memory::*;
// addional module
pub mod console;
pub mod test_util;

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn it_basic() {
        let mut pusher = direct(String::from("test")) ^ in_memory::<String>(None) & test_util::expect_eq(String::from("test"));
        // pusher.awake().await;
        pusher.awake().await;
    }
}

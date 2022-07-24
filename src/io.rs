mod cache;
mod core;

pub use self::cache::cache_with;
pub use self::core::text_file;
pub use self::core::bin_file;

#[cfg(test)]
mod tests {
    use super::{cache_with, text_file};
    use crate::json::{from_json, from_record, Record};
    use crate::test::assert_eq_store;
    use crate::{named, transport};

    #[tokio::test]
    async fn it_no_cache() {
        let file = text_file("testdoc/test.json");
        let pipe_file = file.clone() ^ from_json::<Record>() ^ from_record();
        let pipe_assert = assert_eq_store("test".to_string(), "test".to_string()) ^ named("text")
            | assert_eq_store(1, 1) ^ named("int")
            | assert_eq_store(1.0, 1.0) ^ named("float");
        transport(pipe_assert.clone(), pipe_file.clone()).await;
        transport(pipe_file.clone(), pipe_assert.clone()).await;
    }
    #[tokio::test]
    async fn it_use_cache() {
        let file = cache_with(text_file("testdoc/test.json"));
        let pipe_file = file.clone() ^ from_json::<Record>() ^ from_record();
        let pipe_assert = assert_eq_store("test".to_string(), "test".to_string()) ^ named("text")
            | assert_eq_store(1, 1) ^ named("int")
            | assert_eq_store(1.0, 1.0) ^ named("float");
        transport(pipe_assert.clone(), pipe_file.clone()).await;
        transport(pipe_file.clone(), pipe_assert.clone()).await;
    }
}

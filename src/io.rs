mod core;
mod cache;

pub use self::core::text_file;
pub use self::cache::cache_with;

#[cfg(test)]
mod tests {
    use super::{text_file, cache_with};
    use crate::{named, store, transport};
    use crate::json::from_json;
    use crate::test::assert_eq_store;

    #[tokio::test]
    async fn it_no_cache() {
        let file = text_file("testdoc/test.json");
        let pipe_input = file.clone() ^ from_json() ^ named("to")
            | (store("test".to_string()) ^ named("text")
                | store(1) ^ named("int")
                | store(1.0) ^ named("float"))
                ^ named("from");
        let pipe_output = file.clone() ^ from_json() ^ named("from")
            | (assert_eq_store("test".to_string(), "test".to_string()) ^ named("text")
                | assert_eq_store(1, 0) ^ named("int")
                | assert_eq_store(1.0, 0.0) ^ named("float"))
                ^ named("to");
        transport(pipe_input, "from", "to").await;
        transport(pipe_output, "from", "to").await;
    }
    #[tokio::test]
    async fn it_use_cache() {
        let file = cache_with(text_file("testdoc/test.json"));
        let pipe_input = file.clone() ^ from_json() ^ named("to")
            | (store("test".to_string()) ^ named("text")
                | store(1) ^ named("int")
                | store(1.0) ^ named("float"))
                ^ named("from");
        let pipe_output = file.clone() ^ from_json() ^ named("from")
            | (assert_eq_store("test".to_string(), "test".to_string()) ^ named("text")
                | assert_eq_store(1, 0) ^ named("int")
                | assert_eq_store(1.0, 0.0) ^ named("float"))
                ^ named("to");
        transport(pipe_input, "from", "to").await;
        transport(pipe_output, "from", "to").await;
    }
}

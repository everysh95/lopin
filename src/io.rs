mod core;
mod cache;

pub use self::core::text_file;
pub use self::cache::in_memory_cache;

#[cfg(test)]
mod tests {
    use super::{text_file, in_memory_cache};
    use crate::json::from_json;
    use crate::test::assert_eq_store;
    use crate::{named, store, transport};

    #[tokio::test]
    async fn it_from() {
        let file = in_memory_cache(text_file("testdoc/test.json"));
        let pipe = file.clone() ^ from_json() ^ named("to")
            | (store("test".to_string()) ^ named("text")
                | store(1) ^ named("int")
                | store(1.0) ^ named("float"))
                ^ named("from");
        transport(pipe, "from", "to").await;
    }
}

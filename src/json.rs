mod converter;
mod core;

pub use converter::{from_json, to_json};

#[cfg(test)]
mod tests {
    use super::{from_json, to_json};
    use crate::test::assert_eq_store;
    use crate::{named, store, transport};

    #[tokio::test]
    async fn it_from() {
        transport(
            (store("test".to_string()) ^ named("text")
                | store(1) ^ named("integer")
                | store(1.0) ^ named("float"))
                ^ to_json(),
            assert_eq_store(
                "{\"text\":\"test\",\"integer\":1,\"float\":1.0}".to_string(),
                "".to_string(),
            ),
        ).await;
    }

    #[tokio::test]
    async fn it_to() {
        transport(
            store("{\"text\":\"test\",\"integer\":1,\"float\":1.0}".to_string()) ^ from_json(),
            assert_eq_store("test".to_string(), "".to_string()) ^ named("text")
                | assert_eq_store(1, 0) ^ named("integer")
                | assert_eq_store(1.0, 0.0) ^ named("float"),
        ).await;
    }
}

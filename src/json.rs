mod converter;
mod core;

pub use converter::{from_json, to_json};

#[cfg(test)]
mod tests {
    use crate::test::assert_eq_store;
    use crate::{named, store, transport};
    use super::{from_json, to_json};

    #[tokio::test]
    async fn it_from() {
        let pipe = (store("test".to_string()) ^ named("text")
            | store(1) ^ named("integer")
            | store(1.0) ^ named("float"))
            ^ to_json()
            ^ named("from")
            | assert_eq_store("{\"text\":\"test\",\"integer\":1,\"float\":1.0}".to_string(), "".to_string()) ^ named("to");
        transport(pipe, "from", "to").await;
    }

    #[tokio::test]
    async fn it_to() {
        let pipe = store("{\"text\":\"test\",\"integer\":1,\"float\":1.0}".to_string())
            ^ from_json()
            ^ named("from")
            | (assert_eq_store("test".to_string(), "".to_string()) ^ named("text")
                | assert_eq_store(1, 0) ^ named("integer")
                | assert_eq_store(1.0, 0.0) ^ named("float"))
                ^ named("to");
        transport(pipe, "from", "to").await;
    }
}

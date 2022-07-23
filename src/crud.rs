mod core;

pub use self::core::{create, read, update, delete};

#[cfg(test)]
mod tests {

    use crate::{store};
    use crate::test::{assert_eq_store, print_store};
    use super::*;

    #[tokio::test]
    async fn it_basic() {
    }
}
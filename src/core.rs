mod store;
mod table;

pub use store::op_set;
pub use store::op_where;
pub use store::Store;
pub use store::StoreError;
pub use table::*;

#[cfg(test)]
mod tests;
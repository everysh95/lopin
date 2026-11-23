mod store;
mod table;
mod db;

pub use store::op_set;
pub use store::op_fn;
pub use store::op_where;
pub use store::op_map;
pub use store::op_values;
pub use store::map_values;
pub use store::store;
pub use store::Store;
pub use store::StoreError;
pub use table::*;
pub use db::*;

#[cfg(test)]
mod tests;
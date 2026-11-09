use std::{ops::{BitAnd, BitOr, BitXor}, sync::Arc};

mod store;

pub use store::Store;
pub use store::op_set;
pub use store::op_where;
pub use store::StoreError;

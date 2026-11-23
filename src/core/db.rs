use std::ops::{BitAnd, BitOr, BitOrAssign, Shl, ShlAssign};


use crate::{Store, Table, op_set, op_where, store, table_name};


#[derive(Debug,Clone)]
pub struct DataBase {
    tables: Store<Table>,
    table_names: Store<String>
}

pub fn database() -> DataBase {
    DataBase { tables: store(), table_names: store() }
}

impl BitAnd<&str> for DataBase {
   type Output = Table;
   fn bitand(self, rhs: &str) -> Self::Output {
       (self.table_names & op_where(|n| &n == rhs) & self.tables).get_values()[0].clone()
   }
}

impl BitOr<Table> for DataBase {
   type Output = DataBase; 
   fn bitor(self, rhs: Table) -> Self::Output {
        let name = rhs.clone() & table_name();
        let mut res = self.clone();
        res.table_names = res.table_names | Some(name.to_string());
        res.tables = res.tables | (res.table_names.get_max_id(), rhs);
        res
   }
}

impl BitOrAssign<Table> for DataBase  {
    fn bitor_assign(&mut self, rhs: Table) {
        let name = rhs.clone() & table_name();
        self.table_names = self.table_names.clone() | Some(name.to_string());
        self.tables = self.tables.clone() | (self.table_names.get_max_id(), rhs);
    }
}

impl ShlAssign<Table> for DataBase {
    fn shl_assign(&mut self, rhs: Table) {
        let name = rhs.clone() & table_name();
        self.tables <<= self.table_names.clone()
            & op_where(|n| n == name.clone())
            & self.tables.clone()
            << op_set(|_| rhs.clone());
    }
}

impl Shl<Table> for DataBase {
    type Output = DataBase;
    fn shl(self, rhs: Table) -> Self::Output {
        let mut res = self.clone();
        res <<= rhs;
        res
    }
}
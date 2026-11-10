use std::ops::{BitAnd, BitOr, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr };

#[derive(Debug, Clone)]
pub struct StoreItem<ValueType: Clone> {
    id: usize,
    value: ValueType,
}

impl<ValueType: Clone> StoreItem<ValueType> {
    pub fn create(id: usize, value: &ValueType) -> StoreItem<ValueType> {
        StoreItem {
            id,
            value: value.clone(),
        }
    }
    pub fn get_id(self: &Self) -> usize {
        self.id
    }
    pub fn get_value(self: &Self) -> ValueType {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Store<ValueType: Clone> {
    values: Vec<StoreItem<ValueType>>,
    max_id: usize,
}

#[derive(Debug, Clone)]
pub struct IDList {
    ids: Vec<usize>,
}

pub fn op_where<LVT: Clone,F:Fn(LVT) -> bool>(f : F) -> Box<F> {
    Box::new(f)
}

pub fn op_set<LVT: Clone,F:Fn(LVT) -> LVT>(f : F) -> Box<F> {
    Box::new(f)
}

pub enum StoreError {
    Unknown,
    DuplicateInsert,
}

impl<ValueType: Clone> Store<ValueType> {
    pub fn new() -> Store<ValueType> {
        Store {
            max_id: 0,
            values: vec![],
        }
    }
    pub fn insert(self: &mut Self, id: usize, value: &ValueType) -> Result<usize, StoreError> {
        if self.values.iter().all(|v| v.get_id() != id.clone()) {
            self.values.push(StoreItem::create(id, value));
            if self.max_id < id.clone() {
                self.max_id = id.clone();
            }
            Ok(id)
        } else {
            Err(StoreError::DuplicateInsert)
        }
    }
    pub fn insert_next(self: &mut Self, value: &ValueType) -> usize {
        self.max_id = self.max_id + 1;
        self.values.push(StoreItem::create(self.max_id, value));
        self.max_id
    }
    pub fn update(self: &mut Self, value: &ValueType, id: usize) {
        let pos = self.values.iter().position(|v| v.get_id() == id);
        if let Some(pos) = pos {
            self.values[pos] = StoreItem::create(id, value);
        }
    }
    pub fn delete(self: &mut Self, id: usize) {
        let pos = self.values.iter().position(|v| v.get_id() == id);
        if let Some(pos) = pos {
            self.values.remove(pos);
        }
    }
    pub fn select(self: &Self, id: usize) -> Option<ValueType> {
        let pos = self.values.iter().position(|v| v.get_id() == id);
        if let Some(pos) = pos {
            Some(self.values[pos].get_value())
        } else {
            None
        }
    }
    pub fn get_ids(self: &Self) -> Vec<usize> {
        self.values.iter().map(|v| v.get_id()).collect()
    }
    pub fn get_values(self: &Self) -> Vec<ValueType> {
        self.values.iter().map(|v| v.get_value()).collect()
    }
    pub fn get_max_id(self: &Self) -> usize {
        self.max_id
    }
}

impl Store<usize> {
    pub fn revase(self: &Self) -> Store<usize> {
        let max_id = self
            .values
            .iter()
            .map(|v| v.get_value())
            .reduce(|p, v| if p < v { v } else { p })
            .unwrap_or_default();
        let values: Vec<StoreItem<usize>> = self
            .values
            .iter()
            .map(|v| StoreItem::create(v.get_value(), &v.get_id()))
            .collect();
        Store { max_id, values }
    }
}

impl Not for Store<usize> {
    type Output = Store<usize>;
    fn not(self) -> Self::Output {
        self.revase()
    }
}

impl<LVT: Clone, RVT: Clone> BitAnd<Store<RVT>> for Store<LVT> {
    type Output = IDList;
    fn bitand(self, rhs: Store<RVT>) -> Self::Output {
        let ids: Vec<_> = self
            .values
            .iter()
            .filter(|lv| rhs.values.iter().any(|i| i.get_id() == lv.get_id()))
            .map(|lv| lv.get_id())
            .collect();
        IDList { ids }
    }
}

impl<LVT: Clone,F:Fn(LVT) -> bool> BitAnd<Box<F>> for Store<LVT> {
    type Output = IDList;
    fn bitand(self, rhs: Box<F>) -> Self::Output {
        let ids = self
            .values
            .iter()
            .filter(|lv| rhs(lv.get_value()))
            .map(|lv| lv.get_id())
            .collect();
        IDList { ids }
    }
}

impl<LVT: Clone> BitAnd<IDList> for Store<LVT> {
    type Output = Store<LVT>;
    fn bitand(self, rhs: IDList) -> Self::Output {
        let max_id = self.max_id;
        let values = self
            .values
            .iter()
            .filter(|lv| rhs.ids.iter().any(|i| *i == lv.get_id()))
            .cloned()
            .collect();
        Store { max_id, values }
    }
}

impl<RVT: Clone> BitAnd<Store<RVT>> for IDList {
    type Output = Store<RVT>;
    fn bitand(self, rhs: Store<RVT>) -> Self::Output {
        rhs & self
    }
}

impl<LVT: Clone, RVT: Clone> BitOr<Store<RVT>> for Store<LVT> {
    type Output = IDList;
    fn bitor(self, rhs: Store<RVT>) -> Self::Output {
        let mut ids: Vec<_> = self.values.iter().map(|lv| lv.get_id()).collect();
        let mut ids2: Vec<_> = rhs.values.iter().map(|lv| lv.get_id()).collect();
        ids.append(&mut ids2);
        IDList { ids }
    }
}

impl BitOr<IDList> for IDList {
    type Output = IDList;
    fn bitor(self, rhs: IDList) -> Self::Output {
        let mut ids = self.ids;
        let mut ids2 = rhs.ids;
        ids.append(&mut ids2);
        IDList { ids }
    }
}

impl<LVT: Clone> BitOr<(usize,LVT)> for Store<LVT> {
    type Output = Store<LVT>;
    fn bitor(self, rhs: (usize,LVT)) -> Self::Output {
        let mut result = self.clone();
        let (id,value) = rhs;
        let _ = result.insert(id, &value);
        result
    }
}

impl<LVT: Clone> BitOr<Option<LVT>> for Store<LVT> {
    type Output = Store<LVT>;
    fn bitor(self, rhs: Option<LVT>) -> Self::Output {
        match rhs {
            Some(rhs) => self.clone() | (self.max_id + 1, rhs),
            None => self
        }
    }
}

impl<LVT: Clone,F:Fn(LVT) -> LVT> Shl<Box<F>> for Store<LVT> {
    type Output = Store<LVT>;
    fn shl(self, rhs: Box<F>) -> Self::Output {
        let max_id = self.max_id;
        let values = self
            .values
            .iter()
            .map(|lv| StoreItem::create(lv.get_id(), &(rhs(lv.get_value()))))
            .collect();
        Store { max_id, values }
    }
}

impl<LVT: Clone> Shr<Store<LVT>> for Store<LVT> {
    type Output = Store<LVT>;
    fn shr(self, rhs: Store<LVT>) -> Self::Output {
        let max_id = rhs.max_id;
        let values = rhs
            .values
            .iter()
            .map(|rv| {
                let v = self.select(rv.get_id()).unwrap_or(rv.get_value());
                StoreItem::create(rv.get_id(), &v)
            })
            .collect();
        Store { max_id, values }
    }
}
impl<LVT: Clone> Shl<Store<LVT>> for Store<LVT> {
    type Output = Store<LVT>;
    fn shl(self, rhs: Store<LVT>) -> Self::Output {
        rhs >> self
    }
}

impl<LVT: Clone,F:Fn(LVT) -> LVT> ShlAssign<Box<F>> for Store<LVT> {
    fn shl_assign(&mut self, rhs: Box<F>) {
        let res = self.clone() << rhs;
        self.max_id = res.max_id;
        self.values = res.values;
    }
}

impl<LVT: Clone> ShlAssign<Store<LVT>> for Store<LVT> {
    fn shl_assign(&mut self, rhs: Store<LVT>) {
        let res :Store<LVT> = rhs >> self.clone();
        self.max_id = res.max_id;
        self.values = res.values;
    }
}

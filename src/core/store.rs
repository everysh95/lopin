use std::{ops::{BitAnd, BitOr, BitOrAssign, Not, Shl, ShlAssign, Shr }, sync::Arc};

#[derive(Debug, Clone)]
pub struct Store<ValueType: Clone> {
    values: Vec<(usize,ValueType)>,
    max_id: usize,
}

#[derive(Debug, Clone)]
pub struct IDList {
    ids: Vec<usize>,
}

impl IDList {
    pub fn get_ids(self: &Self) -> Vec<usize> {
        self.ids.clone()
    }
}

pub fn op_where<LVT: Clone,F:Fn(LVT) -> bool>(f : F) -> Arc<F> {
    Arc::new(f)
}

pub fn op_set<LVT: Clone,F:Fn(LVT) -> LVT>(f : F) -> Arc<F> {
    Arc::new(f)
}

pub fn op_fn<LVT: Clone,RVT: Clone,F:Fn(LVT) -> RVT + 'static>(f : F) -> Arc<dyn Fn(LVT) -> RVT> {
    Arc::new(f)
}

pub fn op_map<LVT: Clone + 'static,RVT: Clone + 'static>(rf : Arc<dyn Fn(LVT) -> RVT>) -> Arc<dyn Fn(Store<LVT>) -> Store<RVT>> {
    op_fn(move |s| s << rf.clone())
}


pub fn store<LVT: Clone>() -> Store<LVT> {
    Store::new()
}

pub struct OpValues;

pub fn op_values() -> OpValues {
    OpValues
}

pub fn map_values<LVT: Clone>() -> Arc<dyn Fn(Store<LVT>) -> Vec<LVT>> {
    op_fn(|s| s & op_values())
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
    pub fn select(self: &Self, id: usize) -> Option<ValueType> {
        let pos = self.values.iter().position(|(rid,_)| *rid == id);
        if let Some(pos) = pos {
            Some(self.values[pos].1.clone())
        } else {
            None
        }
    }
    pub fn insert(self: &mut Self, id: usize, value: ValueType) {
        if self.get_ids().iter().all(|i| *i != id) {
            self.values.push((id, value));
            if self.max_id < id.clone() {
                self.max_id = id.clone();
            }
        }
    }
    pub fn get_ids(self: &Self) -> Vec<usize> {
        self.values.iter().map(|(rid,_)| *rid).collect()
    }
    pub fn get_values(self: &Self) -> Vec<ValueType> {
        self.values.iter().map(|(_,v)| v.clone()).collect()
    }
    pub fn get_max_id(self: &Self) -> usize {
        self.max_id
    }
}

impl Store<usize> {
    pub fn revase(self: &Self) -> Store<usize> {
        let max_id = self
            .get_values()
            .iter()
            .cloned()
            .reduce(|p, v| if p < v { v } else { p })
            .unwrap_or_default();
        let values: Vec<(usize, usize)> = self
            .get_values()
            .iter()
            .zip(self.get_ids())
            .map(|(v,i)| (*v,i))
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
            .get_ids()
            .iter()
            .cloned()
            .filter(|lv| rhs.get_ids().iter().any(|i| i == lv ))
            .collect();
        IDList { ids }
    }
}

impl<LVT: Clone,F:Fn(LVT) -> bool> BitAnd<Arc<F>> for Store<LVT> {
    type Output = IDList;
    fn bitand(self, rhs: Arc<F>) -> Self::Output {
        let ids = self
            .get_ids()
            .iter()
            .cloned()
            .zip(self.get_values())
            .filter(|(_,lv)| rhs(lv.clone()))
            .map(|(li,_)| li)
            .collect();
        IDList { ids }
    }
}

impl<LVT: Clone> BitAnd<IDList> for Store<LVT> {
    type Output = Store<LVT>;
    fn bitand(self, rhs: IDList) -> Self::Output {
        let max_id = self.max_id;
        let values = self
            .get_ids()
            .iter()
            .cloned()
            .zip(self.get_values())
            .filter(|(rid,_)| rhs.ids.iter().any(|i| *i == *rid))
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

impl<LVT: Clone> BitAnd<OpValues> for Store<LVT> {
    type Output = Vec<LVT>;
    fn bitand(self, _: OpValues) -> Self::Output {
        self.get_values()
    }
}

impl<RVT: Clone> BitAnd<Store<RVT>> for OpValues {
    type Output = Vec<RVT>;
    fn bitand(self, rhs: Store<RVT>) -> Self::Output {
        rhs & self
    }
}


impl<LVT: Clone, RVT: Clone> BitOr<Store<RVT>> for Store<LVT> {
    type Output = IDList;
    fn bitor(self, rhs: Store<RVT>) -> Self::Output {
        let mut ids: Vec<_> = self.get_ids();
        let mut ids2: Vec<_> = rhs.get_ids();
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

impl<LVT: Clone> BitOrAssign<(usize,LVT)> for Store<LVT> {
    fn bitor_assign(&mut self, rhs: (usize,LVT)) {
        let (id,value) = rhs;
        self.insert(id, value);
    }
}

impl<LVT: Clone> BitOr<(usize,LVT)> for Store<LVT> {
    type Output = Store<LVT>;
    fn bitor(self, rhs: (usize,LVT)) -> Self::Output {
        let mut result = self.clone();
        result |= rhs;
        result
    }
}


impl<LVT: Clone> BitOrAssign<Option<LVT>> for Store<LVT> {
    fn bitor_assign(&mut self, rhs: Option<LVT>) {
        if let Some(rhs) = rhs {
            *self |= (self.get_max_id() + 1, rhs);
        }
    }
}

impl<LVT: Clone> BitOr<Option<LVT>> for Store<LVT> {
    type Output = Store<LVT>;
    fn bitor(self, rhs: Option<LVT>) -> Self::Output {
        let mut result = self.clone();
        result |= rhs;
        result
    }
}


impl<LVT: Clone> ShlAssign<Arc<dyn Fn(LVT) -> LVT>> for Store<LVT> {
    fn shl_assign(&mut self, rhs: Arc<dyn Fn(LVT) -> LVT>) {
        self.values = self
            .get_ids()
            .iter()
            .cloned()
            .zip(self.get_values())
            .map(|(id,value)| (id, rhs(value)))
            .collect();
    }
}

impl<LVT: Clone,F: Fn(LVT) -> LVT> ShlAssign<Arc<F>> for Store<LVT> {
    fn shl_assign(&mut self, rhs: Arc<F>) {
        self.values = self
            .get_ids()
            .iter()
            .cloned()
            .zip(self.get_values())
            .map(|(id,value)| (id, rhs(value)))
            .collect();
    }
}

impl<LVT: Clone> ShlAssign<Store<LVT>> for Store<LVT> {
    fn shl_assign(&mut self, rhs: Store<LVT>) {
        self.values = self.get_ids()
            .iter()
            .cloned()
            .zip(self.get_values())
            .map(|(ri,rv)| {
                if let Some(v) = rhs.select(ri) {
                    (ri,v)
                } else {
                    (ri,rv)
                }
            })
            .collect();
    }
}

impl<LVT: Clone,RVT: Clone> Shl<Arc<dyn Fn(LVT) -> RVT>> for Store<LVT> {
    type Output = Store<RVT>;
    fn shl(self, rhs: Arc<dyn Fn(LVT) -> RVT>) -> Self::Output {
        let max_id = self.max_id;
        let values = self
            .get_ids()
            .iter()
            .cloned()
            .zip(self.get_values())
            .map(|(id,value)| (id, rhs(value)))
            .collect();
        Store { max_id, values }
    }
}

impl<LVT: Clone,RVT: Clone, F : Fn(LVT) -> RVT> Shl<Arc<F>> for Store<LVT> {
    type Output = Store<RVT>;
    fn shl(self, rhs: Arc<F>) -> Self::Output {
        let max_id = self.max_id;
        let values = self
            .get_ids()
            .iter()
            .cloned()
            .zip(self.get_values())
            .map(|(id,value)| (id, rhs(value)))
            .collect();
        Store { max_id, values }
    }
}

impl<LVT: Clone> Shl<Store<LVT>> for Store<LVT> {
    type Output = Store<LVT>;
    fn shl(self, rhs: Store<LVT>) -> Self::Output {
        let mut res = self;
        res <<= rhs;
        res
    }
}

impl<LVT: Clone> Shr<Store<LVT>> for Store<LVT> {
    type Output = Store<LVT>;
    fn shr(self, rhs: Store<LVT>) -> Self::Output {
        rhs << self        
    }
}
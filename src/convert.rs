use crate::{RawStore, Store};
use async_trait::async_trait;
use std::marker::Send;
use std::ops::BitOr;
use std::ops::BitAnd;
use std::ops::BitXor;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Converter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    raw: Arc<dyn RawConverter<ST, DT> + Send + Sync>,
}

impl<ST: Clone + Send + Sync, DT: Clone + Send + Sync> Converter<ST, DT> {
    pub fn new(raw: Arc<dyn RawConverter<ST, DT> + Send + Sync>) -> Converter<ST, DT> {
        Converter { raw }
    }
    pub async fn to(&self, src: ST) -> Option<DT> {
        self.raw.to(src).await
    }
    pub async fn from(&self, dist: DT) -> Option<ST> {
        self.raw.from(dist).await
    }
}
impl<ST: Clone + Send + Sync, DT: Clone + Send + Sync> Converter<Vec<ST>, Vec<DT>> {
    pub fn to_broadcast(&self) -> BroadcastConverter<ST,DT> {
        BroadcastConverter::new(self.raw.clone())
    }
}

pub struct BroadcastConverter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    raw: Arc<dyn RawConverter<Vec<ST>, Vec<DT>> + Send + Sync>,
}

impl<ST: Clone + Send + Sync, DT: Clone + Send + Sync> BroadcastConverter<ST, DT> {
    pub fn new(raw: Arc<dyn RawConverter<Vec<ST>, Vec<DT>> + Send + Sync>) -> BroadcastConverter<ST, DT> {
        BroadcastConverter { raw }
    }
    pub async fn to(&self, src: Vec<ST>) -> Option<Vec<DT>> {
        self.raw.to(src).await
    }
    pub async fn from(&self, dist: Vec<DT>) -> Option<Vec<ST>> {
        self.raw.from(dist).await
    }
    pub fn to_narrowcast(&self) -> Converter<Vec<ST>,Vec<DT>> {
        Converter::new(self.raw.clone())
    }
}

#[async_trait]
pub trait RawConverter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    async fn to(&self, src: ST) -> Option<DT>;
    async fn from(&self, dist: DT) -> Option<ST>;
}

struct Convert<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    store: Store<ST>,
    convert: Converter<ST, DT>,
}

#[async_trait]
impl<ST: Clone + Send + Sync, DT: Clone + Send + Sync> RawStore<DT> for Convert<ST, DT> {
    async fn get(&mut self) -> Option<DT> {
        let value = self.store.get().await;
        match value {
            Some(v) => self.convert.to(v).await,
            None => None,
        }
    }
    async fn put(&mut self, value: DT) {
        let put_value = self.convert.from(value.clone()).await;
        match put_value {
            Some(v) => self.store.put(v).await,
            None => {}
        }
    }
}

struct VecConvert<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    store: Store<Vec<ST>>,
    convert: Converter<ST, DT>,
}

#[async_trait]
impl<ST: Clone + Send + Sync, DT: Clone + Send + Sync> RawStore<Vec<DT>> for VecConvert<ST, DT> {
    async fn get(&mut self) -> Option<Vec<DT>> {
        let value = self.store.get().await;
        match value {
            Some(v) => {
                let mut result: Vec<DT> = vec![];
                for sv in v.iter() {
                    if let Some(dv) = self.convert.to(sv.clone()).await {
                        result.push(dv);
                    }
                }
                Some(result)
            },
            None => None,
        }
    }
    async fn put(&mut self, value: Vec<DT>) {
        let mut result: Vec<ST> = vec![];
        for dv in value.iter() {
            if let Some(sv) = self.convert.from(dv.clone()).await {
                result.push(sv);
            }
        }
        self.store.put(result).await;
    }
}
struct BroadcastConvert<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    store: Store<ST>,
    convert: BroadcastConverter<ST,DT>,
}

#[async_trait]
impl<ST: Clone + Send + Sync, DT: Clone + Send + Sync> RawStore<Vec<DT>> for BroadcastConvert<ST, DT> {
    async fn get(&mut self) -> Option<Vec<DT>> {
        let value = self.store.get().await;
        match value {
            Some(v) => self.convert.to(vec![v.clone()]).await,
            None => None,
        }
    }
    async fn put(&mut self, value: Vec<DT>) {
        if let Some(svec) = self.convert.from(value.clone()).await {
            for sv in svec.iter() {
                self.store.put(sv.clone()).await;
            }
        }
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> BitXor<BroadcastConverter<ST, DT>>
    for Store<ST>
{
    type Output = Store<Vec<DT>>;
    fn bitxor(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(BroadcastConvert {
            store: self,
            convert: rhs,
        })));
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> BitXor<BroadcastConverter<ST, DT>>
    for Store<Vec<ST>>
{
    type Output = Store<Vec<DT>>;
    fn bitxor(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(Convert {
            store: self,
            convert: rhs.to_narrowcast(),
        })));
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> BitAnd<BroadcastConverter<ST, DT>>
    for Store<ST>
{
    type Output = Store<Vec<DT>>;
    fn bitand(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(BroadcastConvert {
            store: self,
            convert: rhs,
        })));
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> BitAnd<BroadcastConverter<ST, DT>>
    for Store<Vec<ST>>
{
    type Output = Store<Vec<DT>>;
    fn bitand(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(Convert {
            store: self,
            convert: rhs.to_narrowcast(),
        })));
    }
}


impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> BitXor<Converter<ST, DT>>
    for Store<ST>
{
    type Output = Store<DT>;
    fn bitxor(self, rhs: Converter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(Convert {
            store: self,
            convert: rhs,
        })));
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> BitXor<Converter<ST, DT>>
    for Store<Vec<ST>>
{
    type Output = Store<Vec<DT>>;
    fn bitxor(self, rhs: Converter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(VecConvert {
            store: self,
            convert: rhs,
        })));
    }
}

struct MultiConverter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    lhs: Converter<Vec<ST>, Vec<DT>>,
    rhs: Converter<Vec<ST>, Vec<DT>>,
}

#[async_trait]
impl<ST: Clone + Send + Sync, DT: Clone + Send + Sync> RawConverter<Vec<ST>, Vec<DT>>
    for MultiConverter<ST, DT>
{
    async fn to(&self, src: Vec<ST>) -> Option<Vec<DT>> {
        let opt_lv = self.lhs.to(src.clone()).await;
        let opt_rv = self.rhs.to(src.clone()).await;
        match opt_lv {
            Some(lv) => match opt_rv {
                Some(rv) => Some(vec![lv, rv].concat()),
                None => Some(lv),
            },
            None => match opt_rv {
                Some(rv) => Some(rv),
                None => None,
            },
        }
    }
    async fn from(&self, dist: Vec<DT>) -> Option<Vec<ST>> {
        let opt_lv = self.lhs.from(dist.clone()).await;
        let opt_rv = self.rhs.from(dist.clone()).await;
        match opt_lv {
            Some(lv) => match opt_rv {
                Some(rv) => Some(vec![lv, rv].concat()),
                None => Some(lv),
            },
            None => match opt_rv {
                Some(rv) => Some(rv),
                None => None,
            },
        }
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitOr<Converter<Vec<ST>, Vec<DT>>> for Converter<Vec<ST>, Vec<DT>>
{
    type Output = BroadcastConverter<ST,DT>;
    fn bitor(self, rhs: Converter<Vec<ST>, Vec<DT>>) -> Self::Output {
        BroadcastConverter::new(Arc::new(MultiConverter { lhs: self, rhs }))
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitOr<Converter<Vec<ST>, Vec<DT>>> for BroadcastConverter<ST, DT>
{
    type Output = BroadcastConverter<ST,DT>;
    fn bitor(self, rhs: Converter<Vec<ST>, Vec<DT>>) -> Self::Output {
        BroadcastConverter::new(Arc::new(MultiConverter { lhs: self.to_narrowcast(), rhs }))
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitOr<BroadcastConverter<ST, DT>> for Converter<Vec<ST>, Vec<DT>>
{
    type Output = BroadcastConverter<ST,DT>;
    fn bitor(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        BroadcastConverter::new(Arc::new(MultiConverter { lhs: self, rhs: rhs.to_narrowcast() }))
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitOr<BroadcastConverter<ST, DT>> for BroadcastConverter<ST, DT>
{
    type Output = BroadcastConverter<ST,DT>;
    fn bitor(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        BroadcastConverter::new(Arc::new(MultiConverter { lhs: self.to_narrowcast(), rhs: rhs.to_narrowcast() }))
    }
}

struct PutOnly;

#[async_trait]
impl<ST: 'static +  Clone + Send + Sync> RawConverter<ST, ST> for PutOnly {
    async fn to(&self, _src: ST) -> Option<ST> {
        None
    }
    async fn from(&self, dist: ST) -> Option<ST> {
        Some(dist)
    }
}

pub fn put_only<ST: 'static +  Clone + Send + Sync>() -> Converter<ST,ST> {
    Converter::new(Arc::new(PutOnly))
}

struct GetOnly;

#[async_trait]
impl<ST: 'static +  Clone + Send + Sync> RawConverter<ST, ST> for GetOnly {
    async fn to(&self, src: ST) -> Option<ST> {
        Some(src)
    }
    async fn from(&self, _dist: ST) -> Option<ST> {
        None
    }
}

pub fn get_only<ST: 'static +  Clone + Send + Sync>() -> Converter<ST,ST> {
    Converter::new(Arc::new(GetOnly))
}

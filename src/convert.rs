use crate::{RawStore, Store};
use async_trait::async_trait;
use std::fmt::Debug;
use std::marker::Send;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Converter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    raw: Arc<dyn RawConverter<ST, DT> + Send + Sync>,
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> Converter<ST, DT> {
    pub fn new(raw: Arc<dyn RawConverter<ST, DT> + Send + Sync>) -> Converter<ST, DT> {
        Converter { raw }
    }
    pub async fn to(&self, src: ST) -> Option<DT> {
        self.raw.to(src).await
    }
    pub async fn from(&self, old: Option<ST>, dist: DT) -> Option<ST> {
        self.raw.from(old, dist).await
    }
    pub fn to_vec_converter(&self) -> Converter<Vec<ST>, Vec<DT>> {
        Converter::new(Arc::new(VecConverter { base: self.clone() }))
    }
}
impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    Converter<Vec<ST>, Vec<DT>>
{
    pub fn to_broadcast(&self) -> BroadcastConverter<ST, DT> {
        BroadcastConverter::new(self.raw.clone())
    }
}

struct VecConverter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    base: Converter<ST, DT>,
}

#[async_trait]
impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    RawConverter<Vec<ST>, Vec<DT>> for VecConverter<ST, DT>
{
    async fn to(&self, src: Vec<ST>) -> Option<Vec<DT>> {
        let mut result: Vec<DT> = vec![];
        for sv in src.iter() {
            if let Some(dv) = self.base.to(sv.clone()).await {
                result.push(dv);
            }
        }
        Some(result)
    }
    async fn from(&self, old: Option<Vec<ST>>, dist: Vec<DT>) -> Option<Vec<ST>> {
        let mut result: Vec<ST> = vec![];
        match old {
            Some(old) => {
                for (dv, ov) in dist.iter().zip(old.iter()) {
                    if let Some(sv) = self.base.from(Some(ov.clone()), dv.clone()).await {
                        result.push(sv);
                    }
                }
            }
            None => {
                for dv in dist.iter() {
                    if let Some(sv) = self.base.from(None, dv.clone()).await {
                        result.push(sv);
                    }
                }
            }
        }
        Some(result)
    }
}

pub struct BroadcastConverter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    raw: Arc<dyn RawConverter<Vec<ST>, Vec<DT>> + Send + Sync>,
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BroadcastConverter<ST, DT>
{
    pub fn new(
        raw: Arc<dyn RawConverter<Vec<ST>, Vec<DT>> + Send + Sync>,
    ) -> BroadcastConverter<ST, DT> {
        BroadcastConverter { raw }
    }
    pub async fn to(&self, src: Vec<ST>) -> Option<Vec<DT>> {
        self.raw.to(src).await
    }
    pub async fn from(&self, old: Option<Vec<ST>>, dist: Vec<DT>) -> Option<Vec<ST>> {
        self.raw.from(old, dist).await
    }
    pub fn to_narrowcast(&self) -> Converter<Vec<ST>, Vec<DT>> {
        Converter::new(self.raw.clone())
    }
}

#[async_trait]
pub trait RawConverter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    async fn to(&self, src: ST) -> Option<DT>;
    async fn from(&self, old: Option<ST>, dist: DT) -> Option<ST>;
}

struct Convert<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    store: Store<ST>,
    convert: Converter<ST, DT>,
}

#[async_trait]
impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> RawStore<DT>
    for Convert<ST, DT>
{
    async fn get(&mut self) -> Option<DT> {
        let value = self.store.get().await;
        match value {
            Some(v) => self.convert.to(v).await,
            None => None,
        }
    }
    async fn put(&mut self, value: DT) {
        let put_value = self
            .convert
            .from(self.store.get().await, value.clone())
            .await;
        match put_value {
            Some(v) => self.store.put(v).await,
            None => {}
        }
    }
}

struct BroadcastConvert<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    store: Store<ST>,
    convert: BroadcastConverter<ST, DT>,
}

#[async_trait]
impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> RawStore<Vec<DT>>
    for BroadcastConvert<ST, DT>
{
    async fn get(&mut self) -> Option<Vec<DT>> {
        let value = self.store.get().await;
        match value {
            Some(v) => self.convert.to(vec![v.clone()]).await,
            None => None,
        }
    }
    async fn put(&mut self, value: Vec<DT>) {
        if let Some(svec) = self
            .convert
            .from(
                match self.store.get().await {
                    Some(old) => Some(vec![old]),
                    None => None,
                },
                value.clone(),
            )
            .await
        {
            for sv in svec.iter() {
                self.store.put(sv.clone()).await;
            }
        }
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitXor<BroadcastConverter<ST, DT>> for Store<ST>
{
    type Output = Store<Vec<DT>>;
    fn bitxor(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(BroadcastConvert {
            store: self,
            convert: rhs,
        })));
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitXor<BroadcastConverter<ST, DT>> for Store<Vec<ST>>
{
    type Output = Store<Vec<DT>>;
    fn bitxor(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(Convert {
            store: self,
            convert: rhs.to_narrowcast(),
        })));
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitAnd<BroadcastConverter<ST, DT>> for Store<ST>
{
    type Output = Store<Vec<DT>>;
    fn bitand(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        return Store::new(Arc::new(Mutex::new(BroadcastConvert {
            store: self,
            convert: rhs,
        })));
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitAnd<BroadcastConverter<ST, DT>> for Store<Vec<ST>>
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
        return Store::new(Arc::new(Mutex::new(Convert {
            store: self,
            convert: rhs.to_vec_converter(),
        })));
    }
}

impl<
        ST: Clone + Send + Sync + 'static,
        SDT: Clone + Send + Sync + 'static,
        DT: Clone + Send + Sync + 'static,
    > BitXor<Converter<SDT, DT>> for Converter<ST, SDT>
{
    type Output = BroadcastConverter<ST, DT>;
    fn bitxor(self, rhs: Converter<SDT, DT>) -> Self::Output {
        return Converter::new(Arc::new(ChainConverter { lhs: self, rhs }))
            .to_vec_converter()
            .to_broadcast();
    }
}

impl<
        ST: Clone + Send + Sync + 'static,
        SDT: Clone + Send + Sync + 'static,
        DT: Clone + Send + Sync + 'static,
    > BitXor<Converter<SDT, DT>> for BroadcastConverter<ST, SDT>
{
    type Output = BroadcastConverter<ST, DT>;
    fn bitxor(self, rhs: Converter<SDT, DT>) -> Self::Output {
        return Converter::new(Arc::new(ChainConverter {
            lhs: self.to_narrowcast(),
            rhs: rhs.to_vec_converter(),
        }))
        .to_broadcast();
    }
}

impl<
        ST: Clone + Send + Sync + 'static,
        SDT: Clone + Send + Sync + 'static,
        DT: Clone + Send + Sync + 'static,
    > BitXor<BroadcastConverter<SDT, DT>> for Converter<ST, SDT>
{
    type Output = BroadcastConverter<ST, DT>;
    fn bitxor(self, rhs: BroadcastConverter<SDT, DT>) -> Self::Output {
        return Converter::new(Arc::new(ChainConverter {
            lhs: self.to_vec_converter(),
            rhs: rhs.to_narrowcast(),
        }))
        .to_broadcast();
    }
}

struct ChainConverter<ST: Clone + Send + Sync, SDT: Clone + Send + Sync, DT: Clone + Send + Sync> {
    lhs: Converter<ST, SDT>,
    rhs: Converter<SDT, DT>,
}

#[async_trait]
impl<
        ST: Clone + Send + Sync + 'static,
        SDT: Clone + Send + Sync + 'static,
        DT: Clone + Send + Sync + 'static,
    > RawConverter<ST, DT> for ChainConverter<ST, SDT, DT>
{
    async fn to(&self, src: ST) -> Option<DT> {
        if let Some(lv) = self.lhs.to(src).await {
            if let Some(rv) = self.rhs.to(lv).await {
                Some(rv)
            } else {
                None
            }
        } else {
            None
        }
    }
    async fn from(&self, old: Option<ST>, dist: DT) -> Option<ST> {
        if let Some(rv) = self
            .rhs
            .from(
                match old.clone() {
                    Some(old) => self.lhs.to(old).await,
                    None => None,
                },
                dist,
            )
            .await
        {
            if let Some(lv) = self.lhs.from(old, rv).await {
                Some(lv)
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct MultiConverter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    lhs: Converter<Vec<ST>, Vec<DT>>,
    rhs: Converter<Vec<ST>, Vec<DT>>,
}

#[async_trait]
impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    RawConverter<Vec<ST>, Vec<DT>> for MultiConverter<ST, DT>
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
    async fn from(&self, old: Option<Vec<ST>>, dist: Vec<DT>) -> Option<Vec<ST>> {
        let opt_lv = self.lhs.from(old.clone(), dist.clone()).await;
        let opt_rv = self.rhs.from(old.clone(), dist.clone()).await;
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
    type Output = BroadcastConverter<ST, DT>;
    fn bitor(self, rhs: Converter<Vec<ST>, Vec<DT>>) -> Self::Output {
        BroadcastConverter::new(Arc::new(MultiConverter { lhs: self, rhs }))
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitOr<Converter<Vec<ST>, Vec<DT>>> for BroadcastConverter<ST, DT>
{
    type Output = BroadcastConverter<ST, DT>;
    fn bitor(self, rhs: Converter<Vec<ST>, Vec<DT>>) -> Self::Output {
        BroadcastConverter::new(Arc::new(MultiConverter {
            lhs: self.to_narrowcast(),
            rhs,
        }))
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitOr<BroadcastConverter<ST, DT>> for Converter<Vec<ST>, Vec<DT>>
{
    type Output = BroadcastConverter<ST, DT>;
    fn bitor(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        BroadcastConverter::new(Arc::new(MultiConverter {
            lhs: self,
            rhs: rhs.to_narrowcast(),
        }))
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static>
    BitOr<BroadcastConverter<ST, DT>> for BroadcastConverter<ST, DT>
{
    type Output = BroadcastConverter<ST, DT>;
    fn bitor(self, rhs: BroadcastConverter<ST, DT>) -> Self::Output {
        BroadcastConverter::new(Arc::new(MultiConverter {
            lhs: self.to_narrowcast(),
            rhs: rhs.to_narrowcast(),
        }))
    }
}

struct Dummy;

#[async_trait]
impl<ST: 'static + Clone + Send + Sync> RawConverter<ST, ST> for Dummy {
    async fn to(&self, src: ST) -> Option<ST> {
        Some(src)
    }
    async fn from(&self, _old: Option<ST>, dist: ST) -> Option<ST> {
        Some(dist)
    }
}

pub fn dummy<ST: 'static + Clone + Send + Sync>() -> Converter<ST, ST> {
    Converter::new(Arc::new(Dummy))
}

struct Unwrap;

#[async_trait]
impl<ST: 'static + Clone + Send + Sync,ET: 'static + Clone + Send + Sync + Debug> RawConverter<Result<ST,ET>, ST> for Unwrap {
    async fn to(&self, src: Result<ST,ET>) -> Option<ST> {
        if src.is_ok() {
            Some(src.unwrap())
        } else {
            None
        }
    }
    async fn from(&self, _old: Option<Result<ST,ET>>, dist: ST) -> Option<Result<ST,ET>> {
        Some(Ok(dist))
    }
}

pub fn unwarp<ST: 'static + Clone + Send + Sync,ET: 'static + Clone + Send + Sync + Debug>() -> Converter<Result<ST,ET>, ST> {
    Converter::new(Arc::new(Unwrap))
}

struct UnwrapErr;

#[async_trait]
impl<ST: 'static + Clone + Send + Sync + Debug,ET: 'static + Clone + Send + Sync + Debug> RawConverter<Result<ST,ET>, ET> for UnwrapErr {
    async fn to(&self, src: Result<ST,ET>) -> Option<ET> {
        if src.is_err() {
            Some(src.unwrap_err())
        } else {
            None
        }
    }
    async fn from(&self, _old: Option<Result<ST,ET>>, _dist: ET) -> Option<Result<ST,ET>> {
        None
    }
}

pub fn unwarp_err<ST: 'static + Clone + Send + Sync + Debug,ET: 'static + Clone + Send + Sync + Debug>() -> Converter<Result<ST,ET>, ET> {
    Converter::new(Arc::new(UnwrapErr))
}

struct UnwrapOr<ST: 'static + Clone + Send + Sync> {
    or: ST
}

#[async_trait]
impl<ST: 'static + Clone + Send + Sync,ET: 'static + Clone + Send + Sync + Debug> RawConverter<Result<ST,ET>, ST> for UnwrapOr<ST> {
    async fn to(&self, src: Result<ST,ET>) -> Option<ST> {
        Some(src.unwrap_or(self.or.clone()))
    }
    async fn from(&self, _old: Option<Result<ST,ET>>, dist: ST) -> Option<Result<ST,ET>> {
        Some(Ok(dist))
    }
}

pub fn unwarp_or<ST: 'static + Clone + Send + Sync,ET: 'static + Clone + Send + Sync + Debug>(or : ST) -> Converter<Result<ST,ET>, ST> {
    Converter::new(Arc::new(UnwrapOr {
        or
    }))
}
struct ToVec;

#[async_trait]
impl<ST: 'static + Clone + Send + Sync> RawConverter<ST, Vec<ST>> for ToVec {
    async fn to(&self, src: ST) -> Option<Vec<ST>> {
        Some(vec![src])
    }
    async fn from(&self, _old: Option<ST>, dist: Vec<ST>) -> Option<ST> {
        match dist.last() {
            Some(dist) => Some(dist.clone()),
            None => None
        }
    }
}

pub fn to_vec<ST: 'static + Clone + Send + Sync>() -> Converter<ST, Vec<ST>> {
    Converter::new(Arc::new(ToVec))
}
struct FromVec;

#[async_trait]
impl<ST: 'static + Clone + Send + Sync> RawConverter<Vec<ST>, ST> for FromVec {
    async fn to(&self, src: Vec<ST>) -> Option<ST> {
        match src.last() {
            Some(dist) => Some(dist.clone()),
            None => None
        }
    }
    async fn from(&self, _old: Option<Vec<ST>>, dist: ST) -> Option<Vec<ST>> {
        Some(vec![dist])
    }
}

pub fn from_vec<ST: 'static + Clone + Send + Sync>() -> Converter<Vec<ST>, ST> {
    Converter::new(Arc::new(FromVec))
}
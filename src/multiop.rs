use crate::{BroadcastConverter, Condition, Converter, RawConverter};
use async_trait::async_trait;
use std::ops::BitAnd;
use std::ops::BitXor;
use std::sync::Arc;

struct ConditionedConverter<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    condition: Condition<ST>,
    converter: Converter<ST, DT>,
}

#[async_trait]
impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> RawConverter<ST, DT>
    for ConditionedConverter<ST, DT>
{
    async fn to(&self, src: ST) -> Option<DT> {
        if self.condition.validation(src.clone()).await {
            self.converter.to(src).await
        } else {
            None
        }
    }
    async fn from(&self, old: Option<ST>, dist: DT) -> Option<ST> {
        match self.converter.from(old, dist).await {
            Some(src) => {
                if self.condition.validation(src.clone()).await {
                    Some(src)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

struct ConditionedConverterR<ST: Clone + Send + Sync, DT: Clone + Send + Sync> {
    condition: Condition<DT>,
    converter: Converter<ST, DT>,
}

#[async_trait]
impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> RawConverter<ST, DT>
    for ConditionedConverterR<ST, DT>
{
    async fn to(&self, src: ST) -> Option<DT> {
        match self.converter.to(src).await {
            Some(dist) => {
                if self.condition.validation(dist.clone()).await {
                    Some(dist)
                } else {
                    None
                }
            }
            None => None,
        }
    }
    async fn from(&self, old: Option<ST>, dist: DT) -> Option<ST> {
        if self.condition.validation(dist.clone()).await {
            self.converter.from(old, dist).await
        } else {
            None
        }
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> BitXor<Converter<ST, DT>>
    for Condition<ST>
{
    type Output = BroadcastConverter<ST, DT>;
    fn bitxor(self, rhs: Converter<ST, DT>) -> Self::Output {
        return Converter::new(Arc::new(ConditionedConverter {
            condition: self,
            converter: rhs,
        }))
        .to_vec_converter()
        .to_broadcast();
    }
}

impl<ST: Clone + Send + Sync + 'static, DT: Clone + Send + Sync + 'static> BitAnd<Condition<DT>>
    for Converter<ST, DT>
{
    type Output = BroadcastConverter<ST, DT>;
    fn bitand(self, rhs: Condition<DT>) -> Self::Output {
        return Converter::new(Arc::new(ConditionedConverterR {
            condition: rhs,
            converter: self,
        }))
        .to_vec_converter()
        .to_broadcast();
    }
}

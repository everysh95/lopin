use crate::{Puller, Pusher, RawPusher, RawStore, Store};
use async_trait::async_trait;

struct InMemoryStore<Type>
where
    Type: Send + Sync + Clone + PartialEq,
{
    value: Option<Type>,
}

#[async_trait]
impl<Type> RawStore<Type> for InMemoryStore<Type>
where
    Type: Send + Sync + Clone + PartialEq,
{
    async fn push(&mut self, value: Type, puller_list: &mut Vec<Puller<Type>>) {
        if self.value != Some(value.clone()) {
            self.value = Some(value.clone());
            for p in puller_list.iter_mut() {
                p.pull(value.clone()).await;
            }
        }
    }
}

pub fn in_memory<Type>(init_value: Option<Type>) -> Store<Type>
where
    Type: Send + Sync + Clone + PartialEq + 'static,
{
    Store::new(InMemoryStore { value: init_value })
}

struct DirectPusher<Type>
where
    Type: Send + Sync + Clone,
{
    value: Type,
}

#[async_trait]
impl<Type> RawPusher<Type> for DirectPusher<Type>
where
    Type: Send + Sync + Clone,
{
    async fn awake(&mut self, store: &mut Store<Type>) {
        store.push(self.value.clone()).await;
    }
}

pub fn direct<Type>(value: Type) -> Pusher<Type>
where
    Type: Send + Sync + Clone + 'static,
{
    Pusher::new(DirectPusher { value })
}

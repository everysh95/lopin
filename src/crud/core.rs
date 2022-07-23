use crate::{Condition, Converter, Propaty, PropatyMap, RawConverter, RawStore, Store};
use async_trait::async_trait;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ValueNotFound,
    AleadyExists,
    KeyNotFound,
    DestinationIsError,
}

struct Create<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
> {
    selecter: IDType,
    key: KeyType,
}

struct Read<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
> {
    selecter: IDType,
    phantom: PhantomData<KeyType>,
}

struct Update<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
> {
    selecter: IDType,
    key: KeyType,
}

struct Delete<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
> {
    selecter: IDType,
    key: KeyType,
}

#[async_trait]
impl<
        KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
        IDType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    > RawConverter<Vec<Propaty<IDType>>, Result<Vec<Propaty<KeyType>>, Error>>
    for Create<KeyType, IDType>
{
    async fn to(&self, src: Vec<Propaty<IDType>>) -> Option<Result<Vec<Propaty<KeyType>>, Error>> {
        src.get_value(&self.selecter)
    }
    async fn from(
        &self,
        old: Option<Vec<Propaty<IDType>>>,
        dist: Result<Vec<Propaty<KeyType>>, Error>,
    ) -> Option<Vec<Propaty<IDType>>> {
        match dist {
            Ok(dist) => match dist.get_value(&self.key) {
                Some(dist_key) => match old {
                    Some(old) => match old.get_value::<Vec<Propaty<KeyType>>>(&dist_key) {
                        None => Some(
                            vec![
                                vec![Propaty::new(
                                    self.selecter.clone(),
                                    Result::<Vec<Propaty<KeyType>>, Error>::Ok(dist.clone()),
                                )],
                                old[1..].to_vec(),
                                vec![Propaty::new(dist_key, dist)],
                            ]
                            .concat(),
                        ),
                        Some(_) => Some(
                            vec![
                                vec![Propaty::new(
                                    self.selecter.clone(),
                                    Result::<Vec<Propaty<KeyType>>, Error>::Err(
                                        Error::AleadyExists,
                                    ),
                                )],
                                old[1..].to_vec(),
                            ]
                            .concat(),
                        ),
                    },
                    None => Some(vec![
                        Propaty::new(self.selecter.clone(), dist.clone()),
                        Propaty::new(dist_key, dist.clone()),
                    ]),
                },
                None => match old {
                    Some(old) => Some(
                        vec![
                            vec![Propaty::new(
                                self.selecter.clone(),
                                Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::KeyNotFound),
                            )],
                            old[1..].to_vec(),
                        ]
                        .concat(),
                    ),
                    None => Some(vec![Propaty::new(
                        self.selecter.clone(),
                        Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::KeyNotFound),
                    )]),
                },
            },
            Err(_) => match old {
                Some(old) => Some(
                    vec![
                        vec![Propaty::new(
                            self.selecter.clone(),
                            Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::DestinationIsError),
                        )],
                        old[1..].to_vec(),
                    ]
                    .concat(),
                ),
                None => Some(vec![Propaty::new(
                    self.selecter.clone(),
                    Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::DestinationIsError),
                )]),
            },
        }
    }
}

#[async_trait]
impl<
        KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
        IDType: 'static + PartialEq + Clone + Send + Sync,
    > RawConverter<Vec<Propaty<IDType>>, Result<Vec<Vec<Propaty<KeyType>>>, Error>>
    for Read<KeyType, IDType>
{
    async fn to(
        &self,
        src: Vec<Propaty<IDType>>,
    ) -> Option<Result<Vec<Vec<Propaty<KeyType>>>, Error>> {
        if let Some(condition) =
            src.get_value::<Result<Vec<Vec<Propaty<KeyType>>>, Error>>(&self.selecter)
        {
            match condition {
                Ok(condition) => {
                    let finded: Vec<Vec<Propaty<KeyType>>> = src
                        .iter()
                        .map(|s| match s.get().downcast_ref::<Vec<Propaty<KeyType>>>() {
                            Some(sp) => Some(sp.clone()),
                            None => None,
                        })
                        .filter(|p| p.is_some())
                        .map(|s| s.unwrap())
                        .filter(|vp| {
                            condition
                                .iter()
                                .any(|c| c.iter().all(|cp| vp.iter().any(|p| cp == p)))
                        })
                        .collect();
                    if finded.is_empty() {
                        Some(Result::<Vec<Vec<Propaty<KeyType>>>, Error>::Err(
                            Error::ValueNotFound,
                        ))
                    } else {
                        Some(Result::<Vec<Vec<Propaty<KeyType>>>, Error>::Ok(finded))
                    }
                }
                Err(_) => Some(condition),
            }
        } else {
            None
        }
    }
    async fn from(
        &self,
        old: Option<Vec<Propaty<IDType>>>,
        dist: Result<Vec<Vec<Propaty<KeyType>>>, Error>,
    ) -> Option<Vec<Propaty<IDType>>> {
        match dist {
            Ok(dist) => match old {
                Some(old) => Some(
                    vec![
                        vec![Propaty::new(
                            self.selecter.clone(),
                            Result::<Vec<Vec<Propaty<KeyType>>>, Error>::Ok(dist.clone()),
                        )],
                        old[1..].to_vec(),
                    ]
                    .concat(),
                ),
                None => Some(vec![Propaty::new(
                    self.selecter.clone(),
                    Result::<Vec<Vec<Propaty<KeyType>>>, Error>::Ok(dist.clone()),
                )]),
            },
            Err(_) => match old {
                Some(old) => Some(
                    vec![
                        vec![Propaty::new(
                            self.selecter.clone(),
                            Result::<Vec<Vec<Propaty<KeyType>>>, Error>::Err(
                                Error::DestinationIsError,
                            ),
                        )],
                        old[1..].to_vec(),
                    ]
                    .concat(),
                ),
                None => Some(vec![Propaty::new(
                    self.selecter.clone(),
                    Result::<Vec<Vec<Propaty<KeyType>>>, Error>::Err(Error::DestinationIsError),
                )]),
            },
        }
    }
}

#[async_trait]
impl<
        KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
        IDType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    > RawConverter<Vec<Propaty<IDType>>, Result<Vec<Propaty<KeyType>>, Error>>
    for Update<KeyType, IDType>
{
    async fn to(&self, src: Vec<Propaty<IDType>>) -> Option<Result<Vec<Propaty<KeyType>>, Error>> {
        src.get_value::<Result<Vec<Propaty<KeyType>>, Error>>(&self.selecter)
    }
    async fn from(
        &self,
        old: Option<Vec<Propaty<IDType>>>,
        dist: Result<Vec<Propaty<KeyType>>, Error>,
    ) -> Option<Vec<Propaty<IDType>>> {
        match dist {
            Ok(dist) => match dist.get_value(&self.key) {
                Some(dist_key) => match old {
                    Some(old) => match old.get_value::<Vec<Propaty<KeyType>>>(&dist_key) {
                        None => Some(
                            vec![
                                vec![Propaty::new(
                                    self.selecter.clone(),
                                    Result::<Vec<Propaty<KeyType>>, Error>::Err(
                                        Error::ValueNotFound,
                                    ),
                                )],
                                old[1..].to_vec(),
                            ]
                            .concat(),
                        ),
                        Some(_) => Some(
                            vec![
                                vec![Propaty::new(self.selecter.clone(), dist.clone())],
                                old[1..]
                                    .to_vec()
                                    .iter()
                                    .map(|p| {
                                        if p.key.clone() == dist_key.clone() {
                                            Propaty::new(dist_key.clone(), dist.clone())
                                        } else {
                                            p.clone()
                                        }
                                    })
                                    .collect(),
                            ]
                            .concat(),
                        ),
                    },
                    None => Some(vec![
                        Propaty::new(self.selecter.clone(), dist.clone()),
                        Propaty::new(dist_key, dist.clone()),
                    ]),
                },
                None => match old {
                    Some(old) => Some(
                        vec![
                            vec![Propaty::new(
                                self.selecter.clone(),
                                Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::KeyNotFound),
                            )],
                            old[1..].to_vec(),
                        ]
                        .concat(),
                    ),
                    None => Some(vec![Propaty::new(
                        self.selecter.clone(),
                        Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::KeyNotFound),
                    )]),
                },
            },
            Err(_) => match old {
                Some(old) => Some(
                    vec![
                        vec![Propaty::new(
                            self.selecter.clone(),
                            Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::DestinationIsError),
                        )],
                        old[1..].to_vec(),
                    ]
                    .concat(),
                ),
                None => Some(vec![Propaty::new(
                    self.selecter.clone(),
                    Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::DestinationIsError),
                )]),
            },
        }
    }
}

#[async_trait]
impl<
        KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
        IDType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    > RawConverter<Vec<Propaty<IDType>>, Result<Vec<Propaty<KeyType>>, Error>>
    for Delete<KeyType, IDType>
{
    async fn to(&self, src: Vec<Propaty<IDType>>) -> Option<Result<Vec<Propaty<KeyType>>, Error>> {
        src.get_value::<Result<Vec<Propaty<KeyType>>, Error>>(&self.selecter)
    }
    async fn from(
        &self,
        old: Option<Vec<Propaty<IDType>>>,
        dist: Result<Vec<Propaty<KeyType>>, Error>,
    ) -> Option<Vec<Propaty<IDType>>> {
        match dist {
            Ok(dist) => match dist.get_value(&self.key) {
                Some(dist_key) => match old {
                    Some(old) => match old.get_value::<Vec<Propaty<KeyType>>>(&dist_key) {
                        None => Some(
                            vec![
                                vec![Propaty::new(
                                    self.selecter.clone(),
                                    Result::<Vec<Propaty<KeyType>>, Error>::Err(
                                        Error::ValueNotFound,
                                    ),
                                )],
                                old[1..].to_vec(),
                            ]
                            .concat(),
                        ),
                        Some(_) => Some(
                            vec![
                                vec![Propaty::new(self.selecter.clone(), dist.clone())],
                                old[1..]
                                    .to_vec()
                                    .iter()
                                    .filter(|p| p.key.clone() != dist_key.clone())
                                    .cloned()
                                    .collect(),
                            ]
                            .concat(),
                        ),
                    },
                    None => Some(vec![
                        Propaty::new(self.selecter.clone(), dist.clone()),
                        Propaty::new(dist_key, dist.clone()),
                    ]),
                },
                None => match old {
                    Some(old) => Some(
                        vec![
                            vec![Propaty::new(
                                self.selecter.clone(),
                                Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::KeyNotFound),
                            )],
                            old[1..].to_vec(),
                        ]
                        .concat(),
                    ),
                    None => Some(vec![Propaty::new(
                        self.selecter.clone(),
                        Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::KeyNotFound),
                    )]),
                },
            },
            Err(_) => match old {
                Some(old) => Some(
                    vec![
                        vec![Propaty::new(
                            self.selecter.clone(),
                            Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::DestinationIsError),
                        )],
                        old[1..].to_vec(),
                    ]
                    .concat(),
                ),
                None => Some(vec![Propaty::new(
                    self.selecter.clone(),
                    Result::<Vec<Propaty<KeyType>>, Error>::Err(Error::DestinationIsError),
                )]),
            },
        }
    }
}

pub fn create<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
>(
    key: KeyType,
    selecter: IDType,
) -> Converter<Vec<Propaty<IDType>>, Result<Vec<Propaty<KeyType>>, Error>> {
    Converter::new(Arc::new(Create { key, selecter }))
}

pub fn read<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync,
>(
    selecter: IDType,
) -> Converter<Vec<Propaty<IDType>>, Result<Vec<Vec<Propaty<KeyType>>>, Error>> {
    Converter::new(Arc::new(Read {
        selecter,
        phantom: PhantomData,
    }))
}

pub fn update<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
>(
    key: KeyType,
    selecter: IDType,
) -> Converter<Vec<Propaty<IDType>>, Result<Vec<Propaty<KeyType>>, Error>> {
    Converter::new(Arc::new(Update { key, selecter }))
}

pub fn delete<
    KeyType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
    IDType: 'static + PartialEq + Clone + Send + Sync + std::fmt::Debug,
>(
    key: KeyType,
    selecter: IDType,
) -> Converter<Vec<Propaty<IDType>>, Result<Vec<Propaty<KeyType>>, Error>> {
    Converter::new(Arc::new(Delete { key, selecter }))
}

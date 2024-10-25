use crate::error::Error;
use async_trait::async_trait;
use client::{Client, PaginatedResponse, Params};
use models::resource::Resource;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait ResourceService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync,
    C: Serialize + Send + Sync,
    U: Serialize + Send + Sync,
{
    type Id: ToString + Send + Sync;

    fn new(client: &'a Client) -> Self;
}

#[async_trait]
pub trait GetService<'a, T>
where
    T: Resource + DeserializeOwned + Send + Sync,
{
    type Id: ToString + Send + Sync;
    async fn get(&self, id: Self::Id) -> Result<T, Error>;
}

#[async_trait]
pub trait PostService<'a, T, C>
where
    T: Resource + DeserializeOwned + Send + Sync,
    C: Serialize + Send + Sync,
{
    async fn post(&self, resource_for_create: &C) -> Result<T, Error>;
}

#[async_trait]
pub trait PatchService<'a, T, U>
where
    T: Resource + DeserializeOwned + Send + Sync,
    U: Serialize + Send + Sync,
{
    type Id: ToString + Send + Sync;
    async fn patch(&self, id: Self::Id, params: &U) -> Result<T, Error>;
}

#[async_trait]
pub trait PutService<'a, T, C>
where
    T: Resource + DeserializeOwned + Send + Sync,
    C: Serialize + Send + Sync,
{
    type Id: ToString + Send + Sync;
    async fn put(&self, resource_for_create: &C) -> Result<T, Error>;
}

#[async_trait]
pub trait DeleteService<'a> {
    type Id: ToString + Send + Sync;
    async fn delete(&self, id: Self::Id) -> Result<(), Error>;
}

#[async_trait]
pub trait FindService<'a, T, P>
where
    T: Resource + DeserializeOwned + Send + Sync,
    P: Params + Serialize + Send + Sync,
{
    async fn find(&self, params: P) -> Result<PaginatedResponse<T>, Error>;
}

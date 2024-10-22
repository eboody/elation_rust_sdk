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
    //
    //async fn get(&self, id: Self::Id) -> Result<T, Error>;
    //
    //async fn create(&self, resource: &C) -> Result<T, Error>;
    //
    //async fn update(&self, id: Self::Id, resource: &U) -> Result<T, Error>;
    //
    //async fn put(&self, id: Self::Id, resource: &U) -> Result<T, Error>;
    //
    //async fn delete(&self, id: Self::Id) -> Result<(), Error>;
    //
    //async fn list<P>(&self, params: P) -> Result<PaginatedResponse<T>, Error>
    //where
    //    P: Params + Send + Sync;
}

#[async_trait]
pub trait GetService<'a, T>
where
    T: Resource + DeserializeOwned + Send + Sync,
{
    type Id: ToString + Send + Sync;
    async fn get(&self, id: Self::Id) -> Result<T, Error>;
}

// Similarly implement other traits for BaseService

#[async_trait]
pub trait CreateService<'a, T, C>
where
    T: Resource + DeserializeOwned + Send + Sync,
    C: Serialize + Send + Sync,
{
    async fn create(&self, resource_for_create: &C) -> Result<T, Error>;
}

#[async_trait]
pub trait UpdateService<'a, T, U>
where
    T: Resource + DeserializeOwned + Send + Sync,
    U: Serialize + Send + Sync,
{
    type Id: ToString + Send + Sync;
    async fn update(&self, id: Self::Id, params: &U) -> Result<T, Error>;
}

#[async_trait]
pub trait PutService<'a, T, C>
where
    T: Resource + DeserializeOwned + Send + Sync,
    C: Serialize + Send + Sync,
{
    type Id: ToString + Send + Sync;
    async fn put(&self, id: Self::Id, resource_for_create: &C) -> Result<T, Error>;
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

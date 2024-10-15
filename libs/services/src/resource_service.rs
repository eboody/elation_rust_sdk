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

    async fn get(&self, id: Self::Id) -> Result<T, Error>;

    async fn create(&self, resource: &C) -> Result<T, Error>;

    async fn update(&self, id: Self::Id, resource: &U) -> Result<T, Error>;

    async fn put(&self, id: Self::Id, resource: &U) -> Result<T, Error>;

    async fn delete(&self, id: Self::Id) -> Result<(), Error>;

    async fn list<P>(&self, params: P) -> Result<PaginatedResponse<T>, Error>
    where
        P: Params + Send + Sync;
}

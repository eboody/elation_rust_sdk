use crate::error::Error;
use crate::resource_service::{
    DeleteService, FindService, GetService, PatchService, PostService, PutService,
};
use async_trait::async_trait;
use client::{Client, PaginatedResponse, Params};
use models::resource::Resource;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub struct BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync,
    C: Serialize + Send + Sync,
    U: Serialize + Send + Sync,
{
    client: &'a Client,
    _marker: std::marker::PhantomData<(T, C, U)>,
}

impl<'a, T, C, U> BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync,
    C: Serialize + Send + Sync,
    U: Serialize + Send + Sync,
{
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<'a, T, C, U> GetService<'a, T> for BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync,
    T::Id: ToString + Send + Sync,
    C: Serialize + Send + Sync,
    U: Serialize + Send + Sync,
{
    type Id = T::Id;

    async fn get(&self, id: Self::Id) -> Result<T, Error> {
        let endpoint = format!("{}/{}/", T::endpoint(), id.to_string());
        let response = self.client.get(&endpoint, ()).await?;
        let resource = response.json::<T>().await?;
        Ok(resource)
    }
}

#[async_trait]
impl<'a, T, C, U> PostService<'a, T, C> for BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync,
    T::Id: ToString + Send + Sync,
    C: Serialize + Send + Sync + Debug,
    U: Serialize + Send + Sync,
{
    async fn post(&self, resource: &C) -> Result<T, Error> {
        let endpoint = T::endpoint();
        let response = self.client.post(endpoint, resource).await?;
        let created_resource = response.json::<T>().await?;
        Ok(created_resource)
    }
}

#[async_trait]
impl<'a, T, C, U> PatchService<'a, T, U> for BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync,
    T::Id: ToString + Send + Sync,
    C: Serialize + Send + Sync + Debug,
    U: Serialize + Send + Sync + Debug,
{
    type Id = T::Id;

    async fn patch(&self, id: Self::Id, resource: &U) -> Result<T, Error> {
        let endpoint = format!("{}/{}/", T::endpoint(), id.to_string());
        let response = self.client.patch(&endpoint, resource).await?;
        let updated_resource = response.json::<T>().await?;
        Ok(updated_resource)
    }
}

#[async_trait]
impl<'a, T, C, U> DeleteService<'a> for BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync,
    T::Id: ToString + Send + Sync,
    C: Serialize + Send + Sync + Debug,
    U: Serialize + Send + Sync + Debug,
{
    type Id = T::Id;

    async fn delete(&self, id: Self::Id) -> Result<(), Error> {
        let endpoint = format!("{}/{}/", T::endpoint(), id.to_string());
        self.client.delete(&endpoint).await?;
        Ok(())
    }
}

#[async_trait]
impl<'a, T, C, U> PutService<'a, T, C> for BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync + Debug,
    T::Id: ToString + Send + Sync,
    C: Serialize + Send + Sync + Debug,
    U: Serialize + Send + Sync + Debug,
{
    type Id = T::Id;

    async fn put(&self, resource_for_create: &C) -> Result<T, Error> {
        let endpoint = format!("{}/", T::endpoint());
        let response = self.client.put(&endpoint, resource_for_create).await?;
        let updated_resource = response.json::<T>().await?;
        Ok(updated_resource)
    }
}

#[async_trait]
impl<'a, T, C, U, P> FindService<'a, T, P> for BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync + Debug,
    T::Id: ToString + Send + Sync,
    C: Serialize + Send + Sync + Debug,
    U: Serialize + Send + Sync + Debug,
    P: Params + Send + Sync + 'a,
{
    async fn find(&self, params: P) -> Result<PaginatedResponse<T>, Error> {
        let endpoint = format!("{}/", T::endpoint());
        let response = self.client.get(&endpoint, params).await?;
        let paginated_response = response.json::<PaginatedResponse<T>>().await?;
        Ok(paginated_response)
    }
}

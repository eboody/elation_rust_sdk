use crate::error::Error;
use crate::resource_service::ResourceService;
use async_trait::async_trait;
use client::{Client, PaginatedResponse, Params};
use models::resource::Resource;
use serde::de::DeserializeOwned;
use serde::Serialize;

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
impl<'a, T, C, U> ResourceService<'a, T, C, U> for BaseService<'a, T, C, U>
where
    T: Resource + Serialize + DeserializeOwned + Send + Sync,
    C: Serialize + Send + Sync + std::fmt::Debug,
    U: Serialize + Send + Sync + std::fmt::Debug,
    T::Id: ToString + Send + Sync,
{
    type Id = T::Id;

    fn new(client: &'a Client) -> Self {
        BaseService::new(client)
    }

    async fn get(&self, id: Self::Id) -> Result<T, Error> {
        let endpoint = format!("{}/{}/", T::endpoint(), id.to_string());
        let response = self.client.get(&endpoint, ()).await?;
        let resource = response.json::<T>().await?;
        Ok(resource)
    }

    async fn create(&self, resource: &C) -> Result<T, Error> {
        let endpoint = T::endpoint();
        let response = self.client.post(&endpoint, resource).await?;
        let created_resource = response.json::<T>().await?;
        Ok(created_resource)
    }

    async fn update(&self, id: Self::Id, resource: &U) -> Result<T, Error> {
        let endpoint = format!("{}/{}/", T::endpoint(), id.to_string());
        let response = self.client.patch(&endpoint, resource).await?;
        let updated_resource = response.json::<T>().await?;
        Ok(updated_resource)
    }

    async fn put(&self, id: Self::Id, resource: &U) -> Result<T, Error> {
        let endpoint = format!("{}/{}/", T::endpoint(), id.to_string());
        let response = self.client.put(&endpoint, resource).await?;
        let updated_resource = response.json::<T>().await?;
        Ok(updated_resource)
    }

    async fn delete(&self, id: Self::Id) -> Result<(), Error> {
        let endpoint = format!("{}/{}/", T::endpoint(), id.to_string());
        self.client.delete(&endpoint).await?;
        Ok(())
    }

    async fn list<P>(&self, params: P) -> Result<PaginatedResponse<T>, Error>
    where
        P: Params + Send + Sync,
    {
        let endpoint = format!("{}/", T::endpoint());
        let response = self.client.get(&endpoint, params).await?;
        let paginated_response = response.json::<PaginatedResponse<T>>().await?;
        Ok(paginated_response)
    }
}

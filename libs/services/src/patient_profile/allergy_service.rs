use crate::{
    base_service::BaseService,
    resource_service::{
        CreateService, DeleteService, FindService, GetService, PutService, UpdateService,
    },
    Result,
};

use async_trait::async_trait;
use client::{Client, PaginatedResponse};
use models::patient_profile::{Allergy, AllergyForCreate, AllergyForUpdate, AllergyQueryParams};

//pub type AllergyService<'a> = BaseService<'a, Allergy, AllergyForCreate, AllergyForUpdate>;
pub struct AllergyService<'a> {
    base: BaseService<'a, Allergy, AllergyForCreate, AllergyForUpdate>,
}

impl<'a> AllergyService<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            base: BaseService::new(client),
        }
    }
}

#[async_trait]
impl<'a> GetService<'a, Allergy> for AllergyService<'a> {
    type Id = i64;

    async fn get(&self, id: Self::Id) -> Result<Allergy> {
        self.base.get(id).await
    }
}

#[async_trait]
impl<'a> CreateService<'a, Allergy, AllergyForCreate> for AllergyService<'a> {
    async fn create(&self, resource: &AllergyForCreate) -> Result<Allergy> {
        self.base.create(resource).await
    }
}

#[async_trait]
impl<'a> DeleteService<'a> for AllergyService<'a> {
    type Id = i64;
    async fn delete(&self, id: i64) -> Result<()> {
        self.base.delete(id).await
    }
}

#[async_trait]
impl<'a> PutService<'a, Allergy, AllergyForCreate> for AllergyService<'a> {
    type Id = i64;
    async fn put(&self, allergy: &AllergyForCreate) -> Result<Allergy> {
        self.base.put(allergy).await
    }
}

//#[async_trait]
//impl<'a> UpdateService<'a, Allergy, AllergyForUpdate> for AllergyService<'a> {
//    type Id = i64;
//    async fn update(&self, id: i64, allergy: &AllergyForUpdate) -> Result<Allergy> {
//        self.base.update(id, allergy).await
//    }
//}

#[async_trait]
impl<'a> FindService<'a, Allergy, AllergyQueryParams> for AllergyService<'a> {
    async fn find(&self, params: AllergyQueryParams) -> Result<PaginatedResponse<Allergy>> {
        self.base.find(params).await
    }
}

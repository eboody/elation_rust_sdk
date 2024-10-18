// Helper macro to implement the specified trait for the service
#[macro_export]
macro_rules! impl_service_trait {
    // Implementation for GetService
    ($service:ident, $resource:ty, $id:ty, $_create:ty, $_update:ty, GetService) => {
        #[async_trait]
        impl<'a> GetService<'a, $resource> for $service<'a> {
            type Id = $id;

            async fn get(&self, id: Self::Id) -> Result<$resource, Error> {
                self.base.get(id).await
            }
        }
    };

    // Implementation for CreateService
    ($service:ident, $resource:ty, $_id:ty, $create:ty, $_update:ty, CreateService) => {
        #[async_trait]
        impl<'a> CreateService<'a, $resource, $create> for $service<'a> {
            async fn create(&self, resource: &$create) -> Result<$resource, Error> {
                self.base.create(resource).await
            }
        }
    };

    // Implementation for UpdateService
    ($service:ident, $resource:ty, $id:ty, $_create:ty, $update:ty, UpdateService) => {
        #[async_trait]
        impl<'a> UpdateService<'a, $resource, $update> for $service<'a> {
            type Id = $id;

            async fn update(&self, id: Self::Id, resource: &$update) -> Result<$resource, Error> {
                self.base.update(id, resource).await
            }
        }
    };

    // Implementation for DeleteService
    ($service:ident, $_resource:ty, $id:ty, $_create:ty, $_update:ty, DeleteService) => {
        #[async_trait]
        impl<'a> DeleteService<'a> for $service<'a> {
            type Id = $id;

            async fn delete(&self, id: Self::Id) -> Result<(), Error> {
                self.base.delete(id).await
            }
        }
    };

    // Implementation for ListService
    ($service:ident, $resource:ty, $_id:ty, $_create:ty, $_update:ty, ListService) => {
        #[async_trait]
        impl<'a> ListService<'a, $resource> for $service<'a> {
            async fn list<P>(&self, params: P) -> Result<PaginatedResponse<$resource>, Error>
            where
                P: Params + Send + Sync,
            {
                self.base.list(params).await
            }
        }
    };
}

#[macro_export]
macro_rules! impl_service {
    (
        $service:ident,
        $resource:ty,
        $id:ty,
        $create:ty,
        $update:ty,
        [$($trait:ident),*]
    ) => {
        // Define the service struct
        pub struct $service<'a> {
            base: BaseService<'a, $resource, $create, $update>,
        }

        // Implement the constructor
        impl<'a> $service<'a> {
            pub fn new(client: &'a Client) -> Self {
                Self {
                    base: BaseService::new(client),
                }
            }
        }

        // Implement the specified traits by delegating to BaseService
        $(
            impl_service_trait!($service, $resource, $id, $create, $update, $trait);
        )*
    };
}

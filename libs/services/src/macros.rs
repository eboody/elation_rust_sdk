#[macro_export]
macro_rules! impl_service_trait {
    // Implement GetService
    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        GetService
    ) => {
        #[async_trait::async_trait]
        impl<'a> GetService<'a, $resource> for $service_name<'a> {
            type Id = $id_type;

            async fn get(&self, id: Self::Id) -> Result<$resource> {
                self.base.get(id).await
            }
        }
    };

    // Implement CreateService
    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PostService
    ) => {
        #[async_trait::async_trait]
        impl<'a> PostService<'a, $resource, $resource_for_create> for $service_name<'a> {
            async fn post(&self, resource: &$resource_for_create) -> Result<$resource> {
                self.base.post(resource).await
            }
        }
    };

    // Implement DeleteService
    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        DeleteService
    ) => {
        #[async_trait::async_trait]
        impl<'a> DeleteService<'a> for $service_name<'a> {
            type Id = $id_type;

            async fn delete(&self, id: Self::Id) -> Result<()> {
                self.base.delete(id).await
            }
        }
    };

    // Implement UpdateService
    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PatchService
    ) => {
        #[async_trait::async_trait]
        impl<'a> PatchService<'a, $resource, $resource_for_update> for $service_name<'a> {
            type Id = $id_type;

            async fn patch(
                &self,
                id: Self::Id,
                resource: &$resource_for_update,
            ) -> Result<$resource> {
                self.base.patch(id, resource).await
            }
        }
    };

    // Implement PutService
    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PutService
    ) => {
        #[async_trait::async_trait]
        impl<'a> PutService<'a, $resource, $resource_for_create> for $service_name<'a> {
            type Id = $id_type;

            async fn put(&self, resource: &$resource_for_create) -> Result<$resource> {
                self.base.put(resource).await
            }
        }
    };

    // Implement FindService
    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        FindService
    ) => {
        #[async_trait::async_trait]
        impl<'a> FindService<'a, $resource, $resource_query_params> for $service_name<'a> {
            async fn find(
                &self,
                params: $resource_query_params,
            ) -> Result<PaginatedResponse<$resource>> {
                self.base.find(params).await
            }
        }
    };
}
#[macro_export]
macro_rules! impl_service {
    (
        ServiceName: $service_name:ident,
        Resource: $resource:ty,
        ForCreate: $resource_for_create:ty,
        ForUpdate: $resource_for_update:ty,
        QueryParams: $resource_query_params:ty,
        IdType: $id_type:ty,
        Traits: [$($trait_name:ident),*]
    ) => {
        pub struct $service_name<'a> {
            base: BaseService<'a, $resource, $resource_for_create, $resource_for_update>,
        }

        impl<'a> $service_name<'a> {
            pub fn new(client: &'a Client) -> Self {
                Self {
                    base: BaseService::new(client),
                }
            }
        }

        $(
            crate::impl_service_trait!(
                $service_name,
                $resource,
                $resource_for_create,
                $resource_for_update,
                $resource_query_params,
                $id_type,
                $trait_name
            );
        )*
    };
}

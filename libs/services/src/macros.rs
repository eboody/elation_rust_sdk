#[macro_export]
macro_rules! impl_service_trait {
    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        GetService
    ) => {
        doc_comment! {
concat!(
    "Fetches a single instance of the resource by ID.\n\n",
    "### Parameters:\n",
    "- `id`: The unique identifier of the resource, of type `", stringify!($id_type), "`.\n\n",
    "### Returns:\n",
    "- [", stringify!(Result), "]<[", stringify!($resource), "]>: Contains the requested resource if found, or an error if not found.\n\n",
    "### Example:\n",
    "```rust\n",
    "let service = ", stringify!($service_name), "::new(&client);\n",
    "let id: ", stringify!($id_type), " = ...;\n",
    "let resource = service.get(id).await?;\n",
    "println!(\"Resource: {:?}\", resource);\n",
    "```\n"
),
            #[async_trait::async_trait]
            impl<'a> GetService<'a, $resource> for $service_name<'a> {
                type Id = $id_type;

                async fn get(&self, id: Self::Id) -> Result<$resource> {
                    self.base.get(id).await
                }
            }
        }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PostService
    ) => {
        doc_comment! {


concat!(
    "Creates a new instance of the resource.\n\n",
    "### Parameters:\n",
    "- `resource`: A reference to the creation struct of type `", stringify!($resource_for_create), "`.\n\n",
    "### Returns:\n",
    "- [", stringify!(Result), "]<[", stringify!($resource), "]>: Contains the created resource if successful, or an error if creation fails.\n\n",
    "### Example:\n",
    "```rust\n",
    "let service = ", stringify!($service_name), "::new(&client);\n",
    "let new_resource = ", stringify!($resource_for_create), " { /* fields */ };\n",
    "let created_resource = service.post(&new_resource).await?;\n",
    "println!(\"Created Resource: {:?}\", created_resource);\n",
    "```\n"
),


            #[async_trait::async_trait]
            impl<'a> PostService<'a, $resource, $resource_for_create> for $service_name<'a> {
                async fn post(&self, resource: &$resource_for_create) -> Result<$resource> {
                    self.base.post(resource).await
                }
            }
        }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        DeleteService
    ) => {
        doc_comment! {

concat!(
    "Deletes an existing resource by ID.\n\n",
    "### Parameters:\n",
    "- `id`: The unique identifier of the resource to delete, of type `", stringify!($id_type), "`.\n\n",
    "### Returns:\n",
    "- [", stringify!(Result), "]<()>: Indicates success with an empty result, or an error if deletion fails.\n\n",
    "### Example:\n",
    "```rust\n",
    "let service = ", stringify!($service_name), "::new(&client);\n",
    "let id: ", stringify!($id_type), " = ...;\n",
    "service.delete(id).await?;\n",
    "println!(\"Resource deleted successfully.\");\n",
    "```\n"
),

            #[async_trait::async_trait]
            impl<'a> DeleteService<'a> for $service_name<'a> {
                type Id = $id_type;

                async fn delete(&self, id: Self::Id) -> Result<()> {
                    self.base.delete(id).await
                }
            }
        }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PatchService
    ) => {
        doc_comment!{

concat!(
    "Applies a partial update to an existing resource by ID.\n\n",
    "### Parameters:\n",
    "- `id`: The unique identifier of the resource to update, of type `", stringify!($id_type), "`.\n",
    "- `resource`: A reference to the update struct, of type `", stringify!($resource_for_update), "`.\n\n",
    "### Returns:\n",
    "- [", stringify!(Result), "]<[", stringify!($resource), "]>: Contains the updated resource if successful, or an error if the operation fails.\n\n",
    "### Example:\n",
    "```rust\n",
    "let service = ", stringify!($service_name), "::new(&client);\n",
    "let id: ", stringify!($id_type), " = ...;\n",
    "let update_data = ", stringify!($resource_for_update), " { /* fields */ };\n",
    "let updated_resource = service.patch(id, &update_data).await?;\n",
    "println!(\"Updated Resource: {:?}\", updated_resource);\n",
    "```\n"
),

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
        }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PutService
    ) => {
        doc_comment! {
concat!(
    "Replaces an existing resource or creates it if it does not exist.\n\n",
    "### Parameters:\n",
    "- `resource`: A reference to the struct used for replacement or creation, of type `", stringify!($resource_for_create), "`.\n\n",
    "### Returns:\n",
    "- [", stringify!(Result), "]<[", stringify!($resource), "]>: Contains the upserted resource if successful, or an error if the operation fails.\n\n",
    "### Example:\n",
    "```rust\n",
    "let service = ", stringify!($service_name), "::new(&client);\n",
    "let resource_data = ", stringify!($resource_for_create), " { /* fields */ };\n",
    "let upserted_resource = service.put(&resource_data).await?;\n",
    "println!(\"Upserted Resource: {:?}\", upserted_resource);\n",
    "```\n"
),
            #[async_trait::async_trait]
            impl<'a> PutService<'a, $resource, $resource_for_create> for $service_name<'a> {
                type Id = $id_type;

                async fn put(&self, resource: &$resource_for_create) -> Result<$resource> {
                    self.base.put(resource).await
                }
            }
        }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        FindService
    ) => {
        doc_comment! {
concat!(
    "Finds pages of [", stringify!($resource), "] based on the provided query parameters.\n\n",
    "### Parameters:\n",
    "- `params`: The query parameters used for filtering, of type [", stringify!($resource_query_params), "].\n\n",
    "### Returns:\n",
    "- [", stringify!(Result), "](services::Result)<[", stringify!(PaginatedResponse), "]<[", stringify!($resource), "]>>: Contains a paginated list of resources matching the criteria, or an error if the operation fails.\n\n",
    "### Example: Fetching a Single Page\n",
    "```rust\n",
    "let service = ", stringify!($service_name), "::new(&client);\n",
    "let params = ", stringify!($resource_query_params), " { /* fields */ };\n",
    "let page = service.find(params).await?;\n",
    "println!(\"Items on this page: {:?}\", page.results);\n",
    "println!(\"Total items available: {}\", page.count);\n",
    "```\n\n",
    "### Example: Iterating Through All Pages\n",
    "```rust\n",
    "let service = ", stringify!($service_name), "::new(&client);\n",
    "let params = ", stringify!($resource_query_params), " { /* fields */ };\n",
    "let mut current_page = service.find(params).await?;\n",
    "let mut all_results = current_page.results;\n\n",
    "while current_page.has_next() {\n",
    "    if let Some(next_page) = current_page.fetch_next_page(&client).await? {\n",
    "        all_results.extend(next_page.results);\n",
    "        current_page = next_page;\n",
    "    } else {\n",
    "        break;\n",
    "    }\n",
    "}\n",
    "println!(\"Total items fetched: {}\", all_results.len());\n",
    "```\n"
),
            #[async_trait::async_trait]
            impl<'a> FindService<'a, $resource, $resource_query_params> for $service_name<'a> {
                async fn find(&self, params: $resource_query_params) -> Result<PaginatedResponse<$resource>> {
                    self.base.find(params).await
                }
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
        doc_comment! {
            concat!(
                "Provides methods for managing [", stringify!($resource), "](models::", stringify!($resource) ,") resources.\n\n",
                "### Overview\n",
                "This service allows for operations on `", stringify!($resource), "` entities with methods depending on the specified traits (Check this pages Trait Implementations to see what methods this service has):\n\n",
                $(
                    "- **[`", stringify!($trait_name), "`](#trait-implementations)**: Provides ", stringify!($trait_name), " functionality for `", stringify!($resource), "`.\n",
                )*
                "\n### Examples\n\n",
                "#### Example: Initializing `", stringify!($service_name), "`\n",
                "```rust\n",
                "let client = Client::new();\n",
                "let service = ", stringify!($service_name), "::new(&client);\n",
                "```\n"
            ),
            pub struct $service_name<'a> {
                base: BaseService<'a, $resource, $resource_for_create, $resource_for_update>,
            }
        }

        impl<'a> $service_name<'a> {
            doc_comment! {
                concat!(
                    "Creates a new instance of `", stringify!($service_name), "`.\n\n",
                    "### Parameters:\n",
                    "- `client`: A reference to the client instance used for making API requests.\n\n",
                    "### Returns:\n",
                    "- A new instance of `", stringify!($service_name), "` configured to manage `", stringify!($resource), "` resources.\n\n",
                    "### Example\n",
                    "```rust\n",
                    "let client = Client::new();\n",
                    "let service = ", stringify!($service_name), "::new(&client);\n",
                    "```\n"
                ),
                pub fn new(client: &'a Client) -> Self {
                    Self {
                        base: BaseService::new(client),
                    }
                }
            }

        }

        $(
            $crate::impl_service_trait!(
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

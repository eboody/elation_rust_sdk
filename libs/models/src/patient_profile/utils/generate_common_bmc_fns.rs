/// Convenience macro rules to generate default CRUD functions for a Bmc/Entity.
/// Note: If custom functionality is required, use the code below as foundational
///       code for the custom implementations.
#[macro_export]
macro_rules! generate_common_bmc_fns {
	(
		Bmc: $struct_name:ident,
		Entity: $entity:ty,
		$(ForCreate: $for_create:ty,)?
		$(ForUpdate: $for_update:ty,)?
		$(Filter: $filter:ty,)?
	) => {
		impl $struct_name {
			$(
				pub async fn create(
					_entity_c: $for_create,
				) -> Result<i64, ()> {
      todo!()
				}

				//pub async fn create_many(
				//	_entity_c: Vec<$for_create>,
				//) -> Result<Vec<i64>, ()> {
				//  todo!()
				//}
			)?

				pub async fn get(
					//ctx: &Ctx,
					//mm: &ModelManager,
					client: &Client,
					id: i64
				) -> Result<$entity, ()> {
					base::create::<Self, _>(ctx, mm, entity_c).await
				}

				pub async fn list(
					//ctx: &Ctx,
					//mm: &ModelManager,
					//filter: Option<Vec<$filter>>,
					//list_options: Option<ListOptions>,
				) -> Result<Vec<$entity>, ()> {
      todo!()
				}


				pub async fn update(
					//ctx: &Ctx,
					//mm: &ModelManager,
					//id: i64,
					//entity_u: $for_update,
				) -> Result<(), ()> {
      todo!()
				}

				pub async fn delete(
					//ctx: &Ctx,
					//mm: &ModelManager,
					//id: i64,
				) -> Result<(), ()> {
      todo!()
				}

				//pub async fn delete_many(
				//	//ctx: &Ctx,
				//	//mm: &ModelManager,
				//	//ids: Vec<i64>,
				//) -> Result<u64, ()> {
				//  todo!()
				//}
		}
	};
}

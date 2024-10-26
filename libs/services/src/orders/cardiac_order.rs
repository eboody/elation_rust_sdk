use crate::prelude::*;
use models::orders::{
    CardiacOrder, CardiacOrderForCreate, CardiacOrderForUpdate, CardiacOrderQueryParams,
};

use crate::impl_service;

impl_service!(
    ServiceName: CardiacOrderService,
    Resource: CardiacOrder,
    ForCreate: CardiacOrderForCreate,
    ForUpdate: CardiacOrderForUpdate,
    QueryParams: CardiacOrderQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        PutService,
        PatchService,
        DeleteService
    ]
);

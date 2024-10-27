use crate::prelude::*;
use models::orders::{
    PulmonaryOrder, PulmonaryOrderForCreate, PulmonaryOrderForUpdate, PulmonaryOrderQueryParams,
};

use crate::impl_service;

impl_service!(
    ServiceName: PulmonaryOrderService,
    Resource: PulmonaryOrder,
    ForCreate: PulmonaryOrderForCreate,
    ForUpdate: PulmonaryOrderForUpdate,
    QueryParams: PulmonaryOrderQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        PatchService,
        PutService,
        DeleteService
    ]
);

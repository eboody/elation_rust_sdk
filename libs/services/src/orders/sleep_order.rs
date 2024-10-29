use crate::prelude::*;
use models::orders::{SleepOrder, SleepOrderForCreate, SleepOrderForUpdate, SleepOrderQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: SleepOrderService,
    Resource: SleepOrder,
    ForCreate: SleepOrderForCreate,
    ForUpdate: SleepOrderForUpdate,
    QueryParams: SleepOrderQueryParams,
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

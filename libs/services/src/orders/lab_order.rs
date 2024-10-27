use crate::prelude::*;
use models::orders::{LabOrder, LabOrderForCreate, LabOrderForUpdate, LabOrderQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: LabOrderService,
    Resource: LabOrder,
    ForCreate: LabOrderForCreate,
    ForUpdate: LabOrderForUpdate,
    QueryParams: LabOrderQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        PatchService,
        DeleteService
    ]
);

use crate::prelude::*;
use models::orders::{LabOrderTest, LabOrderTestForCreate, LabOrderTestQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: LabOrderTestService,
    Resource: LabOrderTest,
    ForCreate: LabOrderTestForCreate,
    ForUpdate: (),
    QueryParams: LabOrderTestQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        DeleteService
    ]
);

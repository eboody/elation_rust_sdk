use crate::prelude::*;
use models::orders::{
    PulmonaryOrderTest, PulmonaryOrderTestForCreate, PulmonaryOrderTestForUpdate,
    PulmonaryOrderTestQueryParams,
};

use crate::impl_service;

impl_service!(
    ServiceName: PulmonaryOrderTestService,
    Resource: PulmonaryOrderTest,
    ForCreate: PulmonaryOrderTestForCreate,
    ForUpdate: PulmonaryOrderTestForUpdate,
    QueryParams: PulmonaryOrderTestQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        DeleteService
    ]
);

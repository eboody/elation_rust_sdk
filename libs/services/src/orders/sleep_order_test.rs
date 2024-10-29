use crate::prelude::*;
use models::orders::{
    SleepOrderTest, SleepOrderTestForCreate, SleepOrderTestForUpdate, SleepOrderTestQueryParams,
};

use crate::impl_service;

impl_service!(
    ServiceName: SleepOrderTestService,
    Resource: SleepOrderTest,
    ForCreate: SleepOrderTestForCreate,
    ForUpdate: SleepOrderTestForUpdate,
    QueryParams: SleepOrderTestQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        DeleteService
    ]
);

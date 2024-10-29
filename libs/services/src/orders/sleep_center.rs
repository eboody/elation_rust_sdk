use crate::prelude::*;
use models::orders::{SleepCenter, SleepCenterQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: SleepCenterService,
    Resource: SleepCenter,
    ForCreate: (),
    ForUpdate: (),
    QueryParams: SleepCenterQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService
    ]
);

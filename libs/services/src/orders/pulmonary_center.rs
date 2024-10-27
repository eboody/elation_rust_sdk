use crate::prelude::*;
use models::orders::{PulmonaryCenter, PulmonaryCenterQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: PulmonaryCenterService,
    Resource: PulmonaryCenter,
    ForCreate: (),
    ForUpdate: (),
    QueryParams: PulmonaryCenterQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService
    ]
);

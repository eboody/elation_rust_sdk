use crate::prelude::*;
use models::orders::{CardiacCenter, CardiacCenterQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: CardiacCenterService,
    Resource: CardiacCenter,
    ForCreate: (),
    ForUpdate: (),
    QueryParams: CardiacCenterQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService
    ]
);

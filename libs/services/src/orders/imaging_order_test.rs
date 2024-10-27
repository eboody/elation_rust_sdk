use crate::prelude::*;
use models::orders::{ImagingOrderTest, ImagingOrderTestForCreate, ImagingOrderTestQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: ImagingOrderTestService,
    Resource: ImagingOrderTest,
    ForCreate: ImagingOrderTestForCreate,
    ForUpdate:(),
    QueryParams: ImagingOrderTestQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService
    ]
);

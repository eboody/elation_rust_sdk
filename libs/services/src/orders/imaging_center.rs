use crate::prelude::*;
use models::orders::{ImagingCenter, ImagingCenterQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: ImagingCenterService,
    Resource: ImagingCenter,
    ForCreate: (),
    ForUpdate: (),
    QueryParams: ImagingCenterQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService
    ]
);

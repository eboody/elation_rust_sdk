use crate::prelude::*;
use models::orders::{AncillaryCompany, AncillaryCompanyQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: AncillaryCompanyService,
    Resource: AncillaryCompany,
    ForCreate: (),
    ForUpdate: (),
    QueryParams: AncillaryCompanyQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService
    ]
);

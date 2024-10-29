use crate::prelude::*;
use models::orders::{LabVendor, LabVendorForCreate, LabVendorForUpdate, LabVendorQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: LabVendorService,
    Resource: LabVendor,
    ForCreate: LabVendorForCreate,
    ForUpdate: LabVendorForUpdate,
    QueryParams: LabVendorQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        PatchService,
        PutService,
        DeleteService
    ]
);

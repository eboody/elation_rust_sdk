use crate::prelude::*;
use models::orders::{LabOrderSet, LabOrderSetForCreate, LabOrderSetForUpdate};

use crate::impl_service;

impl_service!(
    ServiceName: LabOrderSetService,
    Resource: LabOrderSet,
    ForCreate: LabOrderSetForCreate,
    ForUpdate: LabOrderSetForUpdate,
    QueryParams: (),
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

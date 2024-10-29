use crate::prelude::*;
use models::orders::{
    LabOrderCompendium, LabOrderCompendiumForCreate, LabOrderCompendiumForUpdate,
    LabOrderCompendiumQueryParams,
};

use crate::impl_service;

impl_service!(
    ServiceName: LabOrderCompendiumService,
    Resource: LabOrderCompendium,
    ForCreate: LabOrderCompendiumForCreate,
    ForUpdate: LabOrderCompendiumForUpdate,
    QueryParams: LabOrderCompendiumQueryParams,
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

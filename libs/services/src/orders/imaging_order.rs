use crate::prelude::*;
use models::orders::{
    ImagingOrder, ImagingOrderForCreate, ImagingOrderForUpdate, ImagingOrderQueryParams,
};

use crate::impl_service;

impl_service!(
    ServiceName: ImagingOrderService,
    Resource: ImagingOrder,
    ForCreate: ImagingOrderForCreate,
    ForUpdate: ImagingOrderForUpdate,
    QueryParams: ImagingOrderQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        PutService,
        PatchService,
        DeleteService
    ]
);

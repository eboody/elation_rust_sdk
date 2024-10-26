use crate::prelude::*;
use models::orders::{CardiacOrderTest, CardiacOrderTestForCreate, CardiacOrderTestQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: CardiacOrderTestService,
    Resource: CardiacOrderTest,
    ForCreate: CardiacOrderTestForCreate,
    ForUpdate: (),
    QueryParams: CardiacOrderTestQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService
    ]
);

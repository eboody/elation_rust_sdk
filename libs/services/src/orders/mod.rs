use crate::base_service::BaseService;

use models::orders::{LabOrder, LabOrderForCreate, LabOrderForUpdate};

pub type LabOrderService<'a> = BaseService<'a, LabOrder, LabOrderForCreate, LabOrderForUpdate>;

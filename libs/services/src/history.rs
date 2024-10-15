use crate::base_service::BaseService;
use models::patient_profile::{History, HistoryForCreate, HistoryForUpdate};

pub type HistoryService<'a> = BaseService<'a, History, HistoryForCreate, HistoryForUpdate>;

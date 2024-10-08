use lib_utils::{serde::deserialize_date_option, time::Rfc3339};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};

use std::fmt;

time::serde::format_description!(one_true_date, Date, "[year]-[month]-[day]");

#[derive(Debug, Serialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum MedicationType {
    #[default]
    Otc,
    Prescription,
    Discontinued,
    Unknown,
}

impl<'de> Deserialize<'de> for MedicationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MedicationTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for MedicationTypeVisitor {
            type Value = MedicationType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid medication type string")
            }

            fn visit_str<E>(self, value: &str) -> Result<MedicationType, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "otc" => Ok(MedicationType::Otc),
                    "prescription" => Ok(MedicationType::Prescription),
                    "discontinued" => Ok(MedicationType::Discontinued),
                    _ => Ok(MedicationType::Unknown),
                }
            }
        }

        deserializer.deserialize_str(MedicationTypeVisitor)
    }
}

impl std::fmt::Display for MedicationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MedicationType::Otc => write!(f, "otc"),
            MedicationType::Prescription => write!(f, "prescription"),
            MedicationType::Discontinued => write!(f, "discontinued"),
            MedicationType::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentState {
    New,
    Sending,
    Sent,
    PendingComplete,
    Complete,
    Failure,
    Error,
}

impl std::fmt::Display for FulfillmentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FulfillmentState::New => write!(f, "new"),
            FulfillmentState::Sending => write!(f, "sending"),
            FulfillmentState::Sent => write!(f, "sent"),
            FulfillmentState::PendingComplete => write!(f, "pending_complete"),
            FulfillmentState::Complete => write!(f, "complete"),
            FulfillmentState::Failure => write!(f, "failure"),
            FulfillmentState::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentType {
    Fax,
    Mdtoolboxepcs,
    Paper,
    PaperNonfillable,
    Phone,
    Surescripts,
    SurescriptsCancel,
    SurescriptsChange,
}

impl std::fmt::Display for FulfillmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FulfillmentType::Fax => write!(f, "fax"),
            FulfillmentType::Mdtoolboxepcs => write!(f, "mdtoolboxepcs"),
            FulfillmentType::Paper => write!(f, "paper"),
            FulfillmentType::PaperNonfillable => write!(f, "paper_nonfillable"),
            FulfillmentType::Phone => write!(f, "phone"),
            FulfillmentType::Surescripts => write!(f, "surescripts"),
            FulfillmentType::SurescriptsCancel => write!(f, "surescripts_cancel"),
            FulfillmentType::SurescriptsChange => write!(f, "surescripts_change"),
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RxNormCui {
    pub id: i64,
    pub medication_id: i64,
    pub rxnorm_cui: i64,
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub enum QtyUnit {
    Bag,
    Bottle,
    Box,
    Capsule,
    Cartridge,
    Container,
    Drop,
    Gram,
    Inhaler,
    InternationalUnite,
    Kit,
    Liter,
    Lozenge,
    Milligram,
    Milliliter,
    MillionUnits,
    MutuallyDefined,
    FluidOunce,
    NotSpecified,
    Pack,
    Packet,
    Pint,
    Suppository,
    Syringe,
    Tablespoon,
    Tablet,
    Teaspoon,
    TransdermalPatch,
    Tube,
    Unit,
    Vial,
    Each,
    Gum,
    Ampule,
    Applicator,
    Applicatorful,
    Bar,
    Bead,
    Blister,
    Block,
    Bolus,
    Can,
    Canister,
    Caplet,
    Carton,
    Case,
    Cassette,
    Cylinder,
    Disk,
    DosePack,
    DualPack,
    Film,
    Gallon,
    Implant,
    Inhalation,
    InhalerRefill,
    Insert,
    IntravenousBag,
    Kilogram,
    MetricDrop,
    Millimeter,
    Nebule,
    NeedleFreeInjection,
    OcularSystem,
    Ounce,
    Package,
    Pad,
    Paper,
    Patch,
    PenNeedle,
    Pouch,
    Pound,
    PreFilledPenSyringe,
    Puff,
    Quart,
    Ring,
    Sachet,
    Scoopful,
    Sponge,
    Spray,
    Stick,
    Strip,
    Swab,
    Tabminder,
    Tampon,
    Tray,
    Troche,
    Wafer,
}

impl fmt::Display for QtyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self);
        let screaming_snake_case = s
            .chars()
            .flat_map(|c| {
                if c.is_uppercase() {
                    vec!['_', c]
                } else {
                    vec![c]
                }
            })
            .skip(1)
            .collect::<String>()
            .to_uppercase();
        write!(f, "{}", screaming_snake_case)
    }
}

impl Serialize for QtyUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for QtyUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let normalized_str = s.replace(" ", "_").to_uppercase();
        match normalized_str.as_str() {
            "BAG" => Ok(QtyUnit::Bag),
            "BOTTLE" => Ok(QtyUnit::Bottle),
            "BOX" => Ok(QtyUnit::Box),
            "CAPSULE" => Ok(QtyUnit::Capsule),
            "CARTRIDGE" => Ok(QtyUnit::Cartridge),
            "CONTAINER" => Ok(QtyUnit::Container),
            "DROP" => Ok(QtyUnit::Drop),
            "GRAM" => Ok(QtyUnit::Gram),
            "INHALER" => Ok(QtyUnit::Inhaler),
            "INTERNATIONAL_UNITE" => Ok(QtyUnit::InternationalUnite),
            "KIT" => Ok(QtyUnit::Kit),
            "LITER" => Ok(QtyUnit::Liter),
            "LOZENGE" => Ok(QtyUnit::Lozenge),
            "MILLIGRAM" => Ok(QtyUnit::Milligram),
            "MILLILITER" => Ok(QtyUnit::Milliliter),
            "MILLION_UNITS" => Ok(QtyUnit::MillionUnits),
            "MUTUALLY_DEFINED" => Ok(QtyUnit::MutuallyDefined),
            "FLUID_OUNCE" => Ok(QtyUnit::FluidOunce),
            "PACK" => Ok(QtyUnit::Pack),
            "PACKET" => Ok(QtyUnit::Packet),
            "PINT" => Ok(QtyUnit::Pint),
            "SUPPOSITORY" => Ok(QtyUnit::Suppository),
            "SYRINGE" => Ok(QtyUnit::Syringe),
            "TABLESPOON" => Ok(QtyUnit::Tablespoon),
            "TABLET" => Ok(QtyUnit::Tablet),
            "TEASPOON" => Ok(QtyUnit::Teaspoon),
            "TRANSDERMAL_PATCH" => Ok(QtyUnit::TransdermalPatch),
            "TUBE" => Ok(QtyUnit::Tube),
            "UNIT" => Ok(QtyUnit::Unit),
            "VIAL" => Ok(QtyUnit::Vial),
            "EACH" => Ok(QtyUnit::Each),
            "GUM" => Ok(QtyUnit::Gum),
            "AMPULE" => Ok(QtyUnit::Ampule),
            "APPLICATOR" => Ok(QtyUnit::Applicator),
            "APPLICATORFUL" => Ok(QtyUnit::Applicatorful),
            "BAR" => Ok(QtyUnit::Bar),
            "BEAD" => Ok(QtyUnit::Bead),
            "BLISTER" => Ok(QtyUnit::Blister),
            "BLOCK" => Ok(QtyUnit::Block),
            "BOLUS" => Ok(QtyUnit::Bolus),
            "CAN" => Ok(QtyUnit::Can),
            "CANISTER" => Ok(QtyUnit::Canister),
            "CAPLET" => Ok(QtyUnit::Caplet),
            "CARTON" => Ok(QtyUnit::Carton),
            "CASE" => Ok(QtyUnit::Case),
            "CASSETTE" => Ok(QtyUnit::Cassette),
            "CYLINDER" => Ok(QtyUnit::Cylinder),
            "DISK" => Ok(QtyUnit::Disk),
            "DOSE_PACK" => Ok(QtyUnit::DosePack),
            "DUAL_PACK" => Ok(QtyUnit::DualPack),
            "FILM" => Ok(QtyUnit::Film),
            "GALLON" => Ok(QtyUnit::Gallon),
            "IMPLANT" => Ok(QtyUnit::Implant),
            "INHALATION" => Ok(QtyUnit::Inhalation),
            "INHALER_REFILL" => Ok(QtyUnit::InhalerRefill),
            "INSERT" => Ok(QtyUnit::Insert),
            "INTRAVENOUS_BAG" => Ok(QtyUnit::IntravenousBag),
            "KILOGRAM" => Ok(QtyUnit::Kilogram),
            "METRIC_DROP" => Ok(QtyUnit::MetricDrop),
            "MILLIMETER" => Ok(QtyUnit::Millimeter),
            "NEBULE" => Ok(QtyUnit::Nebule),
            "NEEDLE_FREE_INJECTION" => Ok(QtyUnit::NeedleFreeInjection),
            "OCULAR_SYSTEM" => Ok(QtyUnit::OcularSystem),
            "OUNCE" => Ok(QtyUnit::Ounce),
            "PACKAGE" => Ok(QtyUnit::Package),
            "PAD" => Ok(QtyUnit::Pad),
            "PAPER" => Ok(QtyUnit::Paper),
            "PATCH" => Ok(QtyUnit::Patch),
            "PEN_NEEDLE" => Ok(QtyUnit::PenNeedle),
            "POUCH" => Ok(QtyUnit::Pouch),
            "POUND" => Ok(QtyUnit::Pound),
            "PRE_FILLED_PEN_SYRINGE" => Ok(QtyUnit::PreFilledPenSyringe),
            "PUFF" => Ok(QtyUnit::Puff),
            "QUART" => Ok(QtyUnit::Quart),
            "RING" => Ok(QtyUnit::Ring),
            "SACHET" => Ok(QtyUnit::Sachet),
            "SCOOPFUL" => Ok(QtyUnit::Scoopful),
            "SPONGE" => Ok(QtyUnit::Sponge),
            "SPRAY" => Ok(QtyUnit::Spray),
            "STICK" => Ok(QtyUnit::Stick),
            "STRIP" => Ok(QtyUnit::Strip),
            "SWAB" => Ok(QtyUnit::Swab),
            "TABMINDER" => Ok(QtyUnit::Tabminder),
            "TAMPON" => Ok(QtyUnit::Tampon),
            "TRAY" => Ok(QtyUnit::Tray),
            "TROCHE" => Ok(QtyUnit::Troche),
            "WAFER" => Ok(QtyUnit::Wafer),
            _ => Ok(QtyUnit::NotSpecified),
        }
    }
}

#[derive(
    Debug, Clone, derive_more::Display, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum OrderType {
    New,
    Refill,
    DoseChange,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct Medication {
    pub id: i64,
    pub rxnorm_cuis: Vec<String>,
    pub ndcs: Vec<String>,
    pub name: Option<String>,
    pub brand_name: Option<String>,
    pub generic_name: Option<String>,
    pub is_controlled: Option<bool>,
    pub r#type: MedicationType,
    pub route: Option<String>,
    pub strength: Option<String>,
    pub form: Option<String>,
    pub practice: Option<i64>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    pub creation_type: Option<String>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub market_end_date: Option<OffsetDateTime>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub obsolete_date: Option<OffsetDateTime>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MedicationForCreate {
    pub id: Option<i64>,
}

// ServiceLocation Struct
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceLocation {
    pub id: i64,
    pub name: String,
    pub is_primary: bool,
    pub place_of_service: Option<String>,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub phone: Option<String>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
}

// Fulfillment Struct
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fulfillment {
    pub detail: Option<String>,
    pub pharmacy_ncpdpid: Option<String>,
    pub service_location: Option<ServiceLocation>,
    #[serde(rename = "state")]
    pub fulfillment_state: FulfillmentState,
    #[serde_as(as = "Option<Rfc3339>")]
    pub time_completed: Option<OffsetDateTime>,
    #[serde(rename = "type")]
    pub fulfillment_type: FulfillmentType,
}

// Thread Struct
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thread {
    pub id: i64,
    #[serde(deserialize_with = "deserialize_date_option")]
    pub dc_date: Option<Date>,
    pub is_permanent: bool,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreadForCreate {
    pub dc_date: Option<Date>,
    pub is_permanent: bool,
}

// ICD10Code Struct
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Icd10Code {
    pub code: String,
    pub description: String,
}

use std::str::FromStr;

#[derive(Clone, Debug, derive_more::Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ReportType {
    Lab,
    Cardiac,
    Imaging,
    Hospital,
    Consultation,
    Legal,
    Misc,
    Allergist,
    Audiology,
    Billing,
    Business,
    Forms,
    Correspondence,
    Cosmetic,
    Diabetes,
    Directives,
    DME,
    EKG,
    Endocrinology,
    Exams,
    EyeVision,
    GI,
    History,
    Home,
    Hospice,
    Incoming,
    Injection,
    Insurance,
    Intake,
    Neuro,
    Nursing,
    OBGYN,
    OT,
    OldRecord,
    Oncology,
    Ophth,
    Ortho,
    ENT,
    Outgoing,
    Pathology,
    Consent,
    Pediatric,
    Pharmacy,
    PT,
    Preop,
    PaperRx,
    PriorAuth,
    Procedures,
    Psych,
    Pulmonary,
    Quality,
    Radiology,
    Screening,
    SNF,
    Sleep,
    Surgery,
    Testing,
    Therapy,
    Ultrasound,
    UrgentCare,
    Urology,
    Women,
    CarePlan,
    Dermatology,
    PatientForm, //NOTE: This one isnt explicitly listed as a valid variant, yet we have records
                 //with this value
}

#[derive(Clone, Debug, derive_more::Display, Serialize)]
pub enum Status {
    #[serde(rename = "C")]
    Corrected,
    #[serde(rename = "D")]
    Deleted,
    #[serde(rename = "F")]
    Final,
    #[serde(rename = "I")]
    Pending,
    #[serde(rename = "P")]
    Preliminary,
    #[serde(rename = "R")]
    ResultsEnteredNotVerified,
    #[serde(rename = "S")]
    Partial,
    #[serde(rename = "U")]
    ResultsStatusChangeToFinal,
    #[serde(rename = "X")]
    ResultCanceledDueToNonPerformance,
    #[serde(rename = "E")]
    Error,
    #[serde(rename = "A")]
    Amended,
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "C" | "CORRECTED" => Ok(Status::Corrected),
            "D" | "DELETED" => Ok(Status::Deleted),
            "F" | "FINAL" => Ok(Status::Final),
            "I" | "PENDING" => Ok(Status::Pending),
            "P" | "PRELIMINARY" => Ok(Status::Preliminary),
            "R" | "RESULTS ENTERED -- NOT VERIFIED" => Ok(Status::ResultsEnteredNotVerified),
            "S" | "PARTIAL" => Ok(Status::Partial),
            "U"
            | "RESULTS STATUS CHANGE TO FINAL. RESULTS DID NOT CHANGE (DON'T TRANSMIT TEST)." => {
                Ok(Status::ResultsStatusChangeToFinal)
            }
            "X" | "RESULT CANCELED DUE TO NON-PERFORMANCE" => {
                Ok(Status::ResultCanceledDueToNonPerformance)
            }
            "E" | "ERROR" => Ok(Status::Error),
            "A" | "AMENDED" => Ok(Status::Amended),
            _ => Err(serde::de::Error::custom(format!("Unknown Status: {}", s))),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "C" | "CORRECTED" => Ok(Status::Corrected),
            "D" | "DELETED" => Ok(Status::Deleted),
            "F" | "FINAL" => Ok(Status::Final),
            "I" | "PENDING" => Ok(Status::Pending),
            "P" | "PRELIMINARY" => Ok(Status::Preliminary),
            "R" | "RESULTS ENTERED -- NOT VERIFIED" => Ok(Status::ResultsEnteredNotVerified),
            "S" | "PARTIAL" => Ok(Status::Partial),
            "U"
            | "RESULTS STATUS CHANGE TO FINAL. RESULTS DID NOT CHANGE (DON'T TRANSMIT TEST)." => {
                Ok(Status::ResultsStatusChangeToFinal)
            }
            "X" | "RESULT CANCELED DUE TO NON-PERFORMANCE" => {
                Ok(Status::ResultCanceledDueToNonPerformance)
            }
            "E" | "ERROR" => Ok(Status::Error),
            "A" | "AMENDED" => Ok(Status::Amended),
            _ => Err(format!("Unknown Status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum AbnormalFlag {
    BelowLowNormal,
    AboveHighNormal,
    BelowLowerPanicLimits,
    AboveUpperPanicLimits,
    BelowAbsoluteLowOffInstrumentScale,
    AboveAbsoluteHighOffInstrumentScale,
    Normal,
    Abnormal,
    VeryAbnormal,
    PositiveResult,
    NegativeResult,
    IntermediateResult,
    SignificantChangeUp,
    SignificantChangeDown,
    Better,
    Worse,
    Susceptible,
    Resistant,
    Intermediate,
    ModeratelySusceptible,
    VerySusceptible,
    Critical,
    BetaLactamasePos,
    NotApplicable,
    VeryHigh,
}

impl<'de> Deserialize<'de> for AbnormalFlag {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        AbnormalFlag::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for AbnormalFlag {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "below_low_normal" | "below low normal" => Ok(AbnormalFlag::BelowLowNormal),
            "above_high_normal" | "above high normal" => Ok(AbnormalFlag::AboveHighNormal),
            "below_lower_panic_limits" | "below lower panic limits" => {
                Ok(AbnormalFlag::BelowLowerPanicLimits)
            }
            "above_upper_panic_limits" | "above upper panic limits" => {
                Ok(AbnormalFlag::AboveUpperPanicLimits)
            }
            "below_absolute_low_off_instrument_scale"
            | "below absolute low off instrument scale"
            | "below absolute low-off instrument scale" => {
                Ok(AbnormalFlag::BelowAbsoluteLowOffInstrumentScale)
            }
            "above_absolute_high_off_instrument_scale"
            | "above absolute high off instrument scale" => {
                Ok(AbnormalFlag::AboveAbsoluteHighOffInstrumentScale)
            }
            "normal" | "normal (applies to non-numeric results)" => Ok(AbnormalFlag::Normal),
            "abnormal" | "abnormal (applies to non-numeric results)" => Ok(AbnormalFlag::Abnormal),
            "very_abnormal" | "very abnormal" => Ok(AbnormalFlag::VeryAbnormal),
            "positive_result" | "positive result" => Ok(AbnormalFlag::PositiveResult),
            "negative_result" | "negative result" => Ok(AbnormalFlag::NegativeResult),
            "intermediate_result" | "intermediate result" => Ok(AbnormalFlag::IntermediateResult),
            "significant_change_up" | "significant change up" => {
                Ok(AbnormalFlag::SignificantChangeUp)
            }
            "significant_change_down" | "significant change down" => {
                Ok(AbnormalFlag::SignificantChangeDown)
            }
            "better" => Ok(AbnormalFlag::Better),
            "worse" => Ok(AbnormalFlag::Worse),
            "susceptible" => Ok(AbnormalFlag::Susceptible),
            "resistant" => Ok(AbnormalFlag::Resistant),
            "intermediate" => Ok(AbnormalFlag::Intermediate),
            "moderately_susceptible" | "moderately susceptible" => {
                Ok(AbnormalFlag::ModeratelySusceptible)
            }
            "very_susceptible" | "very susceptible" => Ok(AbnormalFlag::VerySusceptible),
            "critical" => Ok(AbnormalFlag::Critical),
            "beta_lactamase_pos" | "beta lactamase pos" => Ok(AbnormalFlag::BetaLactamasePos),
            "not_applicable" | "not applicable" => Ok(AbnormalFlag::NotApplicable),
            "very_high" | "very high" => Ok(AbnormalFlag::VeryHigh),
            _ => Err(format!("Unknown AbnormalFlag: {}", s)),
        }
    }
}

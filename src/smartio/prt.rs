#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "SYNC_CTL register accessor: an alias for `Reg<SYNC_CTL_SPEC>`"]
pub type SYNC_CTL = crate::Reg<sync_ctl::SYNC_CTL_SPEC>;
#[doc = "Synchronization control register"]
pub mod sync_ctl;
#[doc = "LUT_SEL register accessor: an alias for `Reg<LUT_SEL_SPEC>`"]
pub type LUT_SEL = crate::Reg<lut_sel::LUT_SEL_SPEC>;
#[doc = "LUT component input selection"]
pub mod lut_sel;
#[doc = "LUT_CTL register accessor: an alias for `Reg<LUT_CTL_SPEC>`"]
pub type LUT_CTL = crate::Reg<lut_ctl::LUT_CTL_SPEC>;
#[doc = "LUT component control register"]
pub mod lut_ctl;
#[doc = "DU_SEL register accessor: an alias for `Reg<DU_SEL_SPEC>`"]
pub type DU_SEL = crate::Reg<du_sel::DU_SEL_SPEC>;
#[doc = "Data unit component input selection"]
pub mod du_sel;
#[doc = "DU_CTL register accessor: an alias for `Reg<DU_CTL_SPEC>`"]
pub type DU_CTL = crate::Reg<du_ctl::DU_CTL_SPEC>;
#[doc = "Data unit component control register"]
pub mod du_ctl;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;

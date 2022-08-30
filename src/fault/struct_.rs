#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Fault control"]
pub mod ctl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Fault status"]
pub mod status;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Fault data"]
pub mod data;
#[doc = "PENDING0 register accessor: an alias for `Reg<PENDING0_SPEC>`"]
pub type PENDING0 = crate::Reg<pending0::PENDING0_SPEC>;
#[doc = "Fault pending 0"]
pub mod pending0;
#[doc = "PENDING1 register accessor: an alias for `Reg<PENDING1_SPEC>`"]
pub type PENDING1 = crate::Reg<pending1::PENDING1_SPEC>;
#[doc = "Fault pending 1"]
pub mod pending1;
#[doc = "PENDING2 register accessor: an alias for `Reg<PENDING2_SPEC>`"]
pub type PENDING2 = crate::Reg<pending2::PENDING2_SPEC>;
#[doc = "Fault pending 2"]
pub mod pending2;
#[doc = "MASK0 register accessor: an alias for `Reg<MASK0_SPEC>`"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "Fault mask 0"]
pub mod mask0;
#[doc = "MASK1 register accessor: an alias for `Reg<MASK1_SPEC>`"]
pub type MASK1 = crate::Reg<mask1::MASK1_SPEC>;
#[doc = "Fault mask 1"]
pub mod mask1;
#[doc = "MASK2 register accessor: an alias for `Reg<MASK2_SPEC>`"]
pub type MASK2 = crate::Reg<mask2::MASK2_SPEC>;
#[doc = "Fault mask 2"]
pub mod mask2;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_masked;

#[doc = "CH_CTL register accessor: an alias for `Reg<CH_CTL_SPEC>`"]
pub type CH_CTL = crate::Reg<ch_ctl::CH_CTL_SPEC>;
#[doc = "Channel control"]
pub mod ch_ctl;
#[doc = "CH_STATUS register accessor: an alias for `Reg<CH_STATUS_SPEC>`"]
pub type CH_STATUS = crate::Reg<ch_status::CH_STATUS_SPEC>;
#[doc = "Channel status"]
pub mod ch_status;
#[doc = "CH_IDX register accessor: an alias for `Reg<CH_IDX_SPEC>`"]
pub type CH_IDX = crate::Reg<ch_idx::CH_IDX_SPEC>;
#[doc = "Channel current indices"]
pub mod ch_idx;
#[doc = "CH_CURR_PTR register accessor: an alias for `Reg<CH_CURR_PTR_SPEC>`"]
pub type CH_CURR_PTR = crate::Reg<ch_curr_ptr::CH_CURR_PTR_SPEC>;
#[doc = "Channel current descriptor pointer"]
pub mod ch_curr_ptr;
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

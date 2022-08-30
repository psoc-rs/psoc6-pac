#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Port output data register"]
pub mod out;
#[doc = "OUT_CLR register accessor: an alias for `Reg<OUT_CLR_SPEC>`"]
pub type OUT_CLR = crate::Reg<out_clr::OUT_CLR_SPEC>;
#[doc = "Port output data clear register"]
pub mod out_clr;
#[doc = "OUT_SET register accessor: an alias for `Reg<OUT_SET_SPEC>`"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "Port output data set register"]
pub mod out_set;
#[doc = "OUT_INV register accessor: an alias for `Reg<OUT_INV_SPEC>`"]
pub type OUT_INV = crate::Reg<out_inv::OUT_INV_SPEC>;
#[doc = "Port output data invert register"]
pub mod out_inv;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Port input state register"]
pub mod in_;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Port interrupt status register"]
pub mod intr;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Port interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Port interrupt masked status register"]
pub mod intr_masked;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Port interrupt set register"]
pub mod intr_set;
#[doc = "INTR_CFG register accessor: an alias for `Reg<INTR_CFG_SPEC>`"]
pub type INTR_CFG = crate::Reg<intr_cfg::INTR_CFG_SPEC>;
#[doc = "Port interrupt configuration register"]
pub mod intr_cfg;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Port configuration register"]
pub mod cfg;
#[doc = "CFG_IN register accessor: an alias for `Reg<CFG_IN_SPEC>`"]
pub type CFG_IN = crate::Reg<cfg_in::CFG_IN_SPEC>;
#[doc = "Port input buffer configuration register"]
pub mod cfg_in;
#[doc = "CFG_OUT register accessor: an alias for `Reg<CFG_OUT_SPEC>`"]
pub type CFG_OUT = crate::Reg<cfg_out::CFG_OUT_SPEC>;
#[doc = "Port output buffer configuration register"]
pub mod cfg_out;
#[doc = "CFG_SIO register accessor: an alias for `Reg<CFG_SIO_SPEC>`"]
pub type CFG_SIO = crate::Reg<cfg_sio::CFG_SIO_SPEC>;
#[doc = "Port SIO configuration register"]
pub mod cfg_sio;
#[doc = "CFG_IN_GPIO5V register accessor: an alias for `Reg<CFG_IN_GPIO5V_SPEC>`"]
pub type CFG_IN_GPIO5V = crate::Reg<cfg_in_gpio5v::CFG_IN_GPIO5V_SPEC>;
#[doc = "Port GPIO5V input buffer configuration register"]
pub mod cfg_in_gpio5v;

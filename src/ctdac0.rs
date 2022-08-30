#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global CTDAC control"]
    pub ctdac_ctrl: crate::Reg<ctdac_ctrl::CTDAC_CTRL_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Interrupt request register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x24 - Interrupt request set register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0x28 - Interrupt request mask"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x2c - Interrupt request masked"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
    _reserved5: [u8; 0x80],
    #[doc = "0xb0 - CTDAC switch control"]
    pub ctdac_sw: crate::Reg<ctdac_sw::CTDAC_SW_SPEC>,
    #[doc = "0xb4 - CTDAC switch control clear"]
    pub ctdac_sw_clear: crate::Reg<ctdac_sw_clear::CTDAC_SW_CLEAR_SPEC>,
    _reserved7: [u8; 0x48],
    #[doc = "0x100 - DAC Value"]
    pub ctdac_val: crate::Reg<ctdac_val::CTDAC_VAL_SPEC>,
    #[doc = "0x104 - Next DAC value (double buffering)"]
    pub ctdac_val_nxt: crate::Reg<ctdac_val_nxt::CTDAC_VAL_NXT_SPEC>,
}
#[doc = "CTDAC_CTRL register accessor: an alias for `Reg<CTDAC_CTRL_SPEC>`"]
pub type CTDAC_CTRL = crate::Reg<ctdac_ctrl::CTDAC_CTRL_SPEC>;
#[doc = "Global CTDAC control"]
pub mod ctdac_ctrl;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt request set register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt request mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt request masked"]
pub mod intr_masked;
#[doc = "CTDAC_SW register accessor: an alias for `Reg<CTDAC_SW_SPEC>`"]
pub type CTDAC_SW = crate::Reg<ctdac_sw::CTDAC_SW_SPEC>;
#[doc = "CTDAC switch control"]
pub mod ctdac_sw;
#[doc = "CTDAC_SW_CLEAR register accessor: an alias for `Reg<CTDAC_SW_CLEAR_SPEC>`"]
pub type CTDAC_SW_CLEAR = crate::Reg<ctdac_sw_clear::CTDAC_SW_CLEAR_SPEC>;
#[doc = "CTDAC switch control clear"]
pub mod ctdac_sw_clear;
#[doc = "CTDAC_VAL register accessor: an alias for `Reg<CTDAC_VAL_SPEC>`"]
pub type CTDAC_VAL = crate::Reg<ctdac_val::CTDAC_VAL_SPEC>;
#[doc = "DAC Value"]
pub mod ctdac_val;
#[doc = "CTDAC_VAL_NXT register accessor: an alias for `Reg<CTDAC_VAL_NXT_SPEC>`"]
pub type CTDAC_VAL_NXT = crate::Reg<ctdac_val_nxt::CTDAC_VAL_NXT_SPEC>;
#[doc = "Next DAC value (double buffering)"]
pub mod ctdac_val_nxt;

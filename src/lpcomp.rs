#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LPCOMP Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - LPCOMP Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - LPCOMP Interrupt request register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x14 - LPCOMP Interrupt set register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0x18 - LPCOMP Interrupt request mask"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x1c - LPCOMP Interrupt request masked"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
    _reserved6: [u8; 0x20],
    #[doc = "0x40 - Comparator 0 control Register"]
    pub cmp0_ctrl: crate::Reg<cmp0_ctrl::CMP0_CTRL_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x50 - Comparator 0 switch control"]
    pub cmp0_sw: crate::Reg<cmp0_sw::CMP0_SW_SPEC>,
    #[doc = "0x54 - Comparator 0 switch control clear"]
    pub cmp0_sw_clear: crate::Reg<cmp0_sw_clear::CMP0_SW_CLEAR_SPEC>,
    _reserved9: [u8; 0x28],
    #[doc = "0x80 - Comparator 1 control Register"]
    pub cmp1_ctrl: crate::Reg<cmp1_ctrl::CMP1_CTRL_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x90 - Comparator 1 switch control"]
    pub cmp1_sw: crate::Reg<cmp1_sw::CMP1_SW_SPEC>,
    #[doc = "0x94 - Comparator 1 switch control clear"]
    pub cmp1_sw_clear: crate::Reg<cmp1_sw_clear::CMP1_SW_CLEAR_SPEC>,
}
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "LPCOMP Configuration Register"]
pub mod config;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "LPCOMP Status Register"]
pub mod status;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "LPCOMP Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "LPCOMP Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "LPCOMP Interrupt request mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "LPCOMP Interrupt request masked"]
pub mod intr_masked;
#[doc = "CMP0_CTRL register accessor: an alias for `Reg<CMP0_CTRL_SPEC>`"]
pub type CMP0_CTRL = crate::Reg<cmp0_ctrl::CMP0_CTRL_SPEC>;
#[doc = "Comparator 0 control Register"]
pub mod cmp0_ctrl;
#[doc = "CMP0_SW register accessor: an alias for `Reg<CMP0_SW_SPEC>`"]
pub type CMP0_SW = crate::Reg<cmp0_sw::CMP0_SW_SPEC>;
#[doc = "Comparator 0 switch control"]
pub mod cmp0_sw;
#[doc = "CMP0_SW_CLEAR register accessor: an alias for `Reg<CMP0_SW_CLEAR_SPEC>`"]
pub type CMP0_SW_CLEAR = crate::Reg<cmp0_sw_clear::CMP0_SW_CLEAR_SPEC>;
#[doc = "Comparator 0 switch control clear"]
pub mod cmp0_sw_clear;
#[doc = "CMP1_CTRL register accessor: an alias for `Reg<CMP1_CTRL_SPEC>`"]
pub type CMP1_CTRL = crate::Reg<cmp1_ctrl::CMP1_CTRL_SPEC>;
#[doc = "Comparator 1 control Register"]
pub mod cmp1_ctrl;
#[doc = "CMP1_SW register accessor: an alias for `Reg<CMP1_SW_SPEC>`"]
pub type CMP1_SW = crate::Reg<cmp1_sw::CMP1_SW_SPEC>;
#[doc = "Comparator 1 switch control"]
pub mod cmp1_sw;
#[doc = "CMP1_SW_CLEAR register accessor: an alias for `Reg<CMP1_SW_CLEAR_SPEC>`"]
pub type CMP1_SW_CLEAR = crate::Reg<cmp1_sw_clear::CMP1_SW_CLEAR_SPEC>;
#[doc = "Comparator 1 switch control clear"]
pub mod cmp1_sw_clear;

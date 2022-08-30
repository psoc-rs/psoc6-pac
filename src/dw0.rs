#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Pending channels"]
    pub pending: crate::Reg<pending::PENDING_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - System interrupt control"]
    pub status_intr: crate::Reg<status_intr::STATUS_INTR_SPEC>,
    #[doc = "0x14 - Status of interrupts masked"]
    pub status_intr_masked: crate::Reg<status_intr_masked::STATUS_INTR_MASKED_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Active descriptor control"]
    pub act_descr_ctl: crate::Reg<act_descr_ctl::ACT_DESCR_CTL_SPEC>,
    #[doc = "0x24 - Active descriptor source"]
    pub act_descr_src: crate::Reg<act_descr_src::ACT_DESCR_SRC_SPEC>,
    #[doc = "0x28 - Active descriptor destination"]
    pub act_descr_dst: crate::Reg<act_descr_dst::ACT_DESCR_DST_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - Active descriptor X loop control"]
    pub act_descr_x_ctl: crate::Reg<act_descr_x_ctl::ACT_DESCR_X_CTL_SPEC>,
    #[doc = "0x34 - Active descriptor Y loop control"]
    pub act_descr_y_ctl: crate::Reg<act_descr_y_ctl::ACT_DESCR_Y_CTL_SPEC>,
    #[doc = "0x38 - Active descriptor next pointer"]
    pub act_descr_next_ptr: crate::Reg<act_descr_next_ptr::ACT_DESCR_NEXT_PTR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x40 - Active source"]
    pub act_src: crate::Reg<act_src::ACT_SRC_SPEC>,
    #[doc = "0x44 - Active destination"]
    pub act_dst: crate::Reg<act_dst::ACT_DST_SPEC>,
    _reserved13: [u8; 0x07b8],
    #[doc = "0x800..0xa00 - DW channel structure"]
    pub ch_struct: [CH_STRUCT; 16],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH_STRUCT {
    #[doc = "0x00 - Channel control"]
    pub ch_ctl: crate::Reg<self::ch_struct::ch_ctl::CH_CTL_SPEC>,
    #[doc = "0x04 - Channel status"]
    pub ch_status: crate::Reg<self::ch_struct::ch_status::CH_STATUS_SPEC>,
    #[doc = "0x08 - Channel current indices"]
    pub ch_idx: crate::Reg<self::ch_struct::ch_idx::CH_IDX_SPEC>,
    #[doc = "0x0c - Channel current descriptor pointer"]
    pub ch_curr_ptr: crate::Reg<self::ch_struct::ch_curr_ptr::CH_CURR_PTR_SPEC>,
    #[doc = "0x10 - Interrupt"]
    pub intr: crate::Reg<self::ch_struct::intr::INTR_SPEC>,
    #[doc = "0x14 - Interrupt set"]
    pub intr_set: crate::Reg<self::ch_struct::intr_set::INTR_SET_SPEC>,
    #[doc = "0x18 - Interrupt mask"]
    pub intr_mask: crate::Reg<self::ch_struct::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x1c - Interrupt masked"]
    pub intr_masked: crate::Reg<self::ch_struct::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "DW channel structure"]
pub mod ch_struct;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "PENDING register accessor: an alias for `Reg<PENDING_SPEC>`"]
pub type PENDING = crate::Reg<pending::PENDING_SPEC>;
#[doc = "Pending channels"]
pub mod pending;
#[doc = "STATUS_INTR register accessor: an alias for `Reg<STATUS_INTR_SPEC>`"]
pub type STATUS_INTR = crate::Reg<status_intr::STATUS_INTR_SPEC>;
#[doc = "System interrupt control"]
pub mod status_intr;
#[doc = "STATUS_INTR_MASKED register accessor: an alias for `Reg<STATUS_INTR_MASKED_SPEC>`"]
pub type STATUS_INTR_MASKED = crate::Reg<status_intr_masked::STATUS_INTR_MASKED_SPEC>;
#[doc = "Status of interrupts masked"]
pub mod status_intr_masked;
#[doc = "ACT_DESCR_CTL register accessor: an alias for `Reg<ACT_DESCR_CTL_SPEC>`"]
pub type ACT_DESCR_CTL = crate::Reg<act_descr_ctl::ACT_DESCR_CTL_SPEC>;
#[doc = "Active descriptor control"]
pub mod act_descr_ctl;
#[doc = "ACT_DESCR_SRC register accessor: an alias for `Reg<ACT_DESCR_SRC_SPEC>`"]
pub type ACT_DESCR_SRC = crate::Reg<act_descr_src::ACT_DESCR_SRC_SPEC>;
#[doc = "Active descriptor source"]
pub mod act_descr_src;
#[doc = "ACT_DESCR_DST register accessor: an alias for `Reg<ACT_DESCR_DST_SPEC>`"]
pub type ACT_DESCR_DST = crate::Reg<act_descr_dst::ACT_DESCR_DST_SPEC>;
#[doc = "Active descriptor destination"]
pub mod act_descr_dst;
#[doc = "ACT_DESCR_X_CTL register accessor: an alias for `Reg<ACT_DESCR_X_CTL_SPEC>`"]
pub type ACT_DESCR_X_CTL = crate::Reg<act_descr_x_ctl::ACT_DESCR_X_CTL_SPEC>;
#[doc = "Active descriptor X loop control"]
pub mod act_descr_x_ctl;
#[doc = "ACT_DESCR_Y_CTL register accessor: an alias for `Reg<ACT_DESCR_Y_CTL_SPEC>`"]
pub type ACT_DESCR_Y_CTL = crate::Reg<act_descr_y_ctl::ACT_DESCR_Y_CTL_SPEC>;
#[doc = "Active descriptor Y loop control"]
pub mod act_descr_y_ctl;
#[doc = "ACT_DESCR_NEXT_PTR register accessor: an alias for `Reg<ACT_DESCR_NEXT_PTR_SPEC>`"]
pub type ACT_DESCR_NEXT_PTR = crate::Reg<act_descr_next_ptr::ACT_DESCR_NEXT_PTR_SPEC>;
#[doc = "Active descriptor next pointer"]
pub mod act_descr_next_ptr;
#[doc = "ACT_SRC register accessor: an alias for `Reg<ACT_SRC_SPEC>`"]
pub type ACT_SRC = crate::Reg<act_src::ACT_SRC_SPEC>;
#[doc = "Active source"]
pub mod act_src;
#[doc = "ACT_DST register accessor: an alias for `Reg<ACT_DST_SPEC>`"]
pub type ACT_DST = crate::Reg<act_dst::ACT_DST_SPEC>;
#[doc = "Active destination"]
pub mod act_dst;

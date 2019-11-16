#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Pending channels"]
    pub pending: PENDING,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - System interrupt control"]
    pub status_intr: STATUS_INTR,
    #[doc = "0x14 - Status of interrupts masked"]
    pub status_intr_masked: STATUS_INTR_MASKED,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - Active descriptor control"]
    pub act_descr_ctl: ACT_DESCR_CTL,
    #[doc = "0x24 - Active descriptor source"]
    pub act_descr_src: ACT_DESCR_SRC,
    #[doc = "0x28 - Active descriptor destination"]
    pub act_descr_dst: ACT_DESCR_DST,
    _reserved8: [u8; 4usize],
    #[doc = "0x30 - Active descriptor X loop control"]
    pub act_descr_x_ctl: ACT_DESCR_X_CTL,
    #[doc = "0x34 - Active descriptor Y loop control"]
    pub act_descr_y_ctl: ACT_DESCR_Y_CTL,
    #[doc = "0x38 - Active descriptor next pointer"]
    pub act_descr_next_ptr: ACT_DESCR_NEXT_PTR,
    _reserved11: [u8; 4usize],
    #[doc = "0x40 - Active source"]
    pub act_src: ACT_SRC,
    #[doc = "0x44 - Active destination"]
    pub act_dst: ACT_DST,
    _reserved13: [u8; 1976usize],
    #[doc = "0x800 - DW channel structure"]
    pub ch_struct: [CH_STRUCT; 16],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH_STRUCT {
    #[doc = "0x00 - Channel control"]
    pub ch_ctl: self::ch_struct::CH_CTL,
    #[doc = "0x04 - Channel status"]
    pub ch_status: self::ch_struct::CH_STATUS,
    #[doc = "0x08 - Channel current indices"]
    pub ch_idx: self::ch_struct::CH_IDX,
    #[doc = "0x0c - Channel current descriptor pointer"]
    pub ch_curr_ptr: self::ch_struct::CH_CURR_PTR,
    #[doc = "0x10 - Interrupt"]
    pub intr: self::ch_struct::INTR,
    #[doc = "0x14 - Interrupt set"]
    pub intr_set: self::ch_struct::INTR_SET,
    #[doc = "0x18 - Interrupt mask"]
    pub intr_mask: self::ch_struct::INTR_MASK,
    #[doc = "0x1c - Interrupt masked"]
    pub intr_masked: self::ch_struct::INTR_MASKED,
}
#[doc = r"Register block"]
#[doc = "DW channel structure"]
pub mod ch_struct;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Pending channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending](pending) module"]
pub type PENDING = crate::Reg<u32, _PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PENDING;
#[doc = "`read()` method returns [pending::R](pending::R) reader structure"]
impl crate::Readable for PENDING {}
#[doc = "Pending channels"]
pub mod pending;
#[doc = "System interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_intr](status_intr) module"]
pub type STATUS_INTR = crate::Reg<u32, _STATUS_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS_INTR;
#[doc = "`read()` method returns [status_intr::R](status_intr::R) reader structure"]
impl crate::Readable for STATUS_INTR {}
#[doc = "System interrupt control"]
pub mod status_intr;
#[doc = "Status of interrupts masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_intr_masked](status_intr_masked) module"]
pub type STATUS_INTR_MASKED = crate::Reg<u32, _STATUS_INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS_INTR_MASKED;
#[doc = "`read()` method returns [status_intr_masked::R](status_intr_masked::R) reader structure"]
impl crate::Readable for STATUS_INTR_MASKED {}
#[doc = "Status of interrupts masked"]
pub mod status_intr_masked;
#[doc = "Active descriptor control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_ctl](act_descr_ctl) module"]
pub type ACT_DESCR_CTL = crate::Reg<u32, _ACT_DESCR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_CTL;
#[doc = "`read()` method returns [act_descr_ctl::R](act_descr_ctl::R) reader structure"]
impl crate::Readable for ACT_DESCR_CTL {}
#[doc = "Active descriptor control"]
pub mod act_descr_ctl;
#[doc = "Active descriptor source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_src](act_descr_src) module"]
pub type ACT_DESCR_SRC = crate::Reg<u32, _ACT_DESCR_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_SRC;
#[doc = "`read()` method returns [act_descr_src::R](act_descr_src::R) reader structure"]
impl crate::Readable for ACT_DESCR_SRC {}
#[doc = "Active descriptor source"]
pub mod act_descr_src;
#[doc = "Active descriptor destination\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_dst](act_descr_dst) module"]
pub type ACT_DESCR_DST = crate::Reg<u32, _ACT_DESCR_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_DST;
#[doc = "`read()` method returns [act_descr_dst::R](act_descr_dst::R) reader structure"]
impl crate::Readable for ACT_DESCR_DST {}
#[doc = "Active descriptor destination"]
pub mod act_descr_dst;
#[doc = "Active descriptor X loop control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_x_ctl](act_descr_x_ctl) module"]
pub type ACT_DESCR_X_CTL = crate::Reg<u32, _ACT_DESCR_X_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_X_CTL;
#[doc = "`read()` method returns [act_descr_x_ctl::R](act_descr_x_ctl::R) reader structure"]
impl crate::Readable for ACT_DESCR_X_CTL {}
#[doc = "Active descriptor X loop control"]
pub mod act_descr_x_ctl;
#[doc = "Active descriptor Y loop control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_y_ctl](act_descr_y_ctl) module"]
pub type ACT_DESCR_Y_CTL = crate::Reg<u32, _ACT_DESCR_Y_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_Y_CTL;
#[doc = "`read()` method returns [act_descr_y_ctl::R](act_descr_y_ctl::R) reader structure"]
impl crate::Readable for ACT_DESCR_Y_CTL {}
#[doc = "Active descriptor Y loop control"]
pub mod act_descr_y_ctl;
#[doc = "Active descriptor next pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_next_ptr](act_descr_next_ptr) module"]
pub type ACT_DESCR_NEXT_PTR = crate::Reg<u32, _ACT_DESCR_NEXT_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_NEXT_PTR;
#[doc = "`read()` method returns [act_descr_next_ptr::R](act_descr_next_ptr::R) reader structure"]
impl crate::Readable for ACT_DESCR_NEXT_PTR {}
#[doc = "Active descriptor next pointer"]
pub mod act_descr_next_ptr;
#[doc = "Active source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_src](act_src) module"]
pub type ACT_SRC = crate::Reg<u32, _ACT_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_SRC;
#[doc = "`read()` method returns [act_src::R](act_src::R) reader structure"]
impl crate::Readable for ACT_SRC {}
#[doc = "Active source"]
pub mod act_src;
#[doc = "Active destination\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_dst](act_dst) module"]
pub type ACT_DST = crate::Reg<u32, _ACT_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DST;
#[doc = "`read()` method returns [act_dst::R](act_dst::R) reader structure"]
impl crate::Readable for ACT_DST {}
#[doc = "Active destination"]
pub mod act_dst;

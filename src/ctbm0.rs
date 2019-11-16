#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - global CTB and power control"]
    pub ctb_ctrl: CTB_CTRL,
    #[doc = "0x04 - Opamp0 and resistor0 control"]
    pub oa_res0_ctrl: OA_RES0_CTRL,
    #[doc = "0x08 - Opamp1 and resistor1 control"]
    pub oa_res1_ctrl: OA_RES1_CTRL,
    #[doc = "0x0c - Comparator status"]
    pub comp_stat: COMP_STAT,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - Interrupt request register"]
    pub intr: INTR,
    #[doc = "0x24 - Interrupt request set register"]
    pub intr_set: INTR_SET,
    #[doc = "0x28 - Interrupt request mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x2c - Interrupt request masked"]
    pub intr_masked: INTR_MASKED,
    _reserved8: [u8; 80usize],
    #[doc = "0x80 - Opamp0 switch control"]
    pub oa0_sw: OA0_SW,
    #[doc = "0x84 - Opamp0 switch control clear"]
    pub oa0_sw_clear: OA0_SW_CLEAR,
    #[doc = "0x88 - Opamp1 switch control"]
    pub oa1_sw: OA1_SW,
    #[doc = "0x8c - Opamp1 switch control clear"]
    pub oa1_sw_clear: OA1_SW_CLEAR,
    _reserved12: [u8; 16usize],
    #[doc = "0xa0 - CTDAC connection switch control"]
    pub ctd_sw: CTD_SW,
    #[doc = "0xa4 - CTDAC connection switch control clear"]
    pub ctd_sw_clear: CTD_SW_CLEAR,
    _reserved14: [u8; 24usize],
    #[doc = "0xc0 - CTB bus switch control"]
    pub ctb_sw_ds_ctrl: CTB_SW_DS_CTRL,
    #[doc = "0xc4 - CTB bus switch Sar Sequencer control"]
    pub ctb_sw_sq_ctrl: CTB_SW_SQ_CTRL,
    #[doc = "0xc8 - CTB bus switch control status"]
    pub ctb_sw_status: CTB_SW_STATUS,
    _reserved17: [u8; 3636usize],
    #[doc = "0xf00 - Opamp0 trim control"]
    pub oa0_offset_trim: OA0_OFFSET_TRIM,
    #[doc = "0xf04 - Opamp0 trim control"]
    pub oa0_slope_offset_trim: OA0_SLOPE_OFFSET_TRIM,
    #[doc = "0xf08 - Opamp0 trim control"]
    pub oa0_comp_trim: OA0_COMP_TRIM,
    #[doc = "0xf0c - Opamp1 trim control"]
    pub oa1_offset_trim: OA1_OFFSET_TRIM,
    #[doc = "0xf10 - Opamp1 trim control"]
    pub oa1_slope_offset_trim: OA1_SLOPE_OFFSET_TRIM,
    #[doc = "0xf14 - Opamp1 trim control"]
    pub oa1_comp_trim: OA1_COMP_TRIM,
}
#[doc = "global CTB and power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_ctrl](ctb_ctrl) module"]
pub type CTB_CTRL = crate::Reg<u32, _CTB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTB_CTRL;
#[doc = "`read()` method returns [ctb_ctrl::R](ctb_ctrl::R) reader structure"]
impl crate::Readable for CTB_CTRL {}
#[doc = "`write(|w| ..)` method takes [ctb_ctrl::W](ctb_ctrl::W) writer structure"]
impl crate::Writable for CTB_CTRL {}
#[doc = "global CTB and power control"]
pub mod ctb_ctrl;
#[doc = "Opamp0 and resistor0 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa_res0_ctrl](oa_res0_ctrl) module"]
pub type OA_RES0_CTRL = crate::Reg<u32, _OA_RES0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA_RES0_CTRL;
#[doc = "`read()` method returns [oa_res0_ctrl::R](oa_res0_ctrl::R) reader structure"]
impl crate::Readable for OA_RES0_CTRL {}
#[doc = "`write(|w| ..)` method takes [oa_res0_ctrl::W](oa_res0_ctrl::W) writer structure"]
impl crate::Writable for OA_RES0_CTRL {}
#[doc = "Opamp0 and resistor0 control"]
pub mod oa_res0_ctrl;
#[doc = "Opamp1 and resistor1 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa_res1_ctrl](oa_res1_ctrl) module"]
pub type OA_RES1_CTRL = crate::Reg<u32, _OA_RES1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA_RES1_CTRL;
#[doc = "`read()` method returns [oa_res1_ctrl::R](oa_res1_ctrl::R) reader structure"]
impl crate::Readable for OA_RES1_CTRL {}
#[doc = "`write(|w| ..)` method takes [oa_res1_ctrl::W](oa_res1_ctrl::W) writer structure"]
impl crate::Writable for OA_RES1_CTRL {}
#[doc = "Opamp1 and resistor1 control"]
pub mod oa_res1_ctrl;
#[doc = "Comparator status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_stat](comp_stat) module"]
pub type COMP_STAT = crate::Reg<u32, _COMP_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_STAT;
#[doc = "`read()` method returns [comp_stat::R](comp_stat::R) reader structure"]
impl crate::Readable for COMP_STAT {}
#[doc = "Comparator status"]
pub mod comp_stat;
#[doc = "Interrupt request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "Interrupt request set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Interrupt request set register"]
pub mod intr_set;
#[doc = "Interrupt request mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Interrupt request mask"]
pub mod intr_mask;
#[doc = "Interrupt request masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Interrupt request masked"]
pub mod intr_masked;
#[doc = "Opamp0 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_sw](oa0_sw) module"]
pub type OA0_SW = crate::Reg<u32, _OA0_SW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA0_SW;
#[doc = "`read()` method returns [oa0_sw::R](oa0_sw::R) reader structure"]
impl crate::Readable for OA0_SW {}
#[doc = "`write(|w| ..)` method takes [oa0_sw::W](oa0_sw::W) writer structure"]
impl crate::Writable for OA0_SW {}
#[doc = "Opamp0 switch control"]
pub mod oa0_sw;
#[doc = "Opamp0 switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_sw_clear](oa0_sw_clear) module"]
pub type OA0_SW_CLEAR = crate::Reg<u32, _OA0_SW_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA0_SW_CLEAR;
#[doc = "`read()` method returns [oa0_sw_clear::R](oa0_sw_clear::R) reader structure"]
impl crate::Readable for OA0_SW_CLEAR {}
#[doc = "`write(|w| ..)` method takes [oa0_sw_clear::W](oa0_sw_clear::W) writer structure"]
impl crate::Writable for OA0_SW_CLEAR {}
#[doc = "Opamp0 switch control clear"]
pub mod oa0_sw_clear;
#[doc = "Opamp1 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_sw](oa1_sw) module"]
pub type OA1_SW = crate::Reg<u32, _OA1_SW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA1_SW;
#[doc = "`read()` method returns [oa1_sw::R](oa1_sw::R) reader structure"]
impl crate::Readable for OA1_SW {}
#[doc = "`write(|w| ..)` method takes [oa1_sw::W](oa1_sw::W) writer structure"]
impl crate::Writable for OA1_SW {}
#[doc = "Opamp1 switch control"]
pub mod oa1_sw;
#[doc = "Opamp1 switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_sw_clear](oa1_sw_clear) module"]
pub type OA1_SW_CLEAR = crate::Reg<u32, _OA1_SW_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA1_SW_CLEAR;
#[doc = "`read()` method returns [oa1_sw_clear::R](oa1_sw_clear::R) reader structure"]
impl crate::Readable for OA1_SW_CLEAR {}
#[doc = "`write(|w| ..)` method takes [oa1_sw_clear::W](oa1_sw_clear::W) writer structure"]
impl crate::Writable for OA1_SW_CLEAR {}
#[doc = "Opamp1 switch control clear"]
pub mod oa1_sw_clear;
#[doc = "CTDAC connection switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctd_sw](ctd_sw) module"]
pub type CTD_SW = crate::Reg<u32, _CTD_SW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTD_SW;
#[doc = "`read()` method returns [ctd_sw::R](ctd_sw::R) reader structure"]
impl crate::Readable for CTD_SW {}
#[doc = "`write(|w| ..)` method takes [ctd_sw::W](ctd_sw::W) writer structure"]
impl crate::Writable for CTD_SW {}
#[doc = "CTDAC connection switch control"]
pub mod ctd_sw;
#[doc = "CTDAC connection switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctd_sw_clear](ctd_sw_clear) module"]
pub type CTD_SW_CLEAR = crate::Reg<u32, _CTD_SW_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTD_SW_CLEAR;
#[doc = "`read()` method returns [ctd_sw_clear::R](ctd_sw_clear::R) reader structure"]
impl crate::Readable for CTD_SW_CLEAR {}
#[doc = "`write(|w| ..)` method takes [ctd_sw_clear::W](ctd_sw_clear::W) writer structure"]
impl crate::Writable for CTD_SW_CLEAR {}
#[doc = "CTDAC connection switch control clear"]
pub mod ctd_sw_clear;
#[doc = "CTB bus switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_sw_ds_ctrl](ctb_sw_ds_ctrl) module"]
pub type CTB_SW_DS_CTRL = crate::Reg<u32, _CTB_SW_DS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTB_SW_DS_CTRL;
#[doc = "`read()` method returns [ctb_sw_ds_ctrl::R](ctb_sw_ds_ctrl::R) reader structure"]
impl crate::Readable for CTB_SW_DS_CTRL {}
#[doc = "`write(|w| ..)` method takes [ctb_sw_ds_ctrl::W](ctb_sw_ds_ctrl::W) writer structure"]
impl crate::Writable for CTB_SW_DS_CTRL {}
#[doc = "CTB bus switch control"]
pub mod ctb_sw_ds_ctrl;
#[doc = "CTB bus switch Sar Sequencer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_sw_sq_ctrl](ctb_sw_sq_ctrl) module"]
pub type CTB_SW_SQ_CTRL = crate::Reg<u32, _CTB_SW_SQ_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTB_SW_SQ_CTRL;
#[doc = "`read()` method returns [ctb_sw_sq_ctrl::R](ctb_sw_sq_ctrl::R) reader structure"]
impl crate::Readable for CTB_SW_SQ_CTRL {}
#[doc = "`write(|w| ..)` method takes [ctb_sw_sq_ctrl::W](ctb_sw_sq_ctrl::W) writer structure"]
impl crate::Writable for CTB_SW_SQ_CTRL {}
#[doc = "CTB bus switch Sar Sequencer control"]
pub mod ctb_sw_sq_ctrl;
#[doc = "CTB bus switch control status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_sw_status](ctb_sw_status) module"]
pub type CTB_SW_STATUS = crate::Reg<u32, _CTB_SW_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTB_SW_STATUS;
#[doc = "`read()` method returns [ctb_sw_status::R](ctb_sw_status::R) reader structure"]
impl crate::Readable for CTB_SW_STATUS {}
#[doc = "CTB bus switch control status"]
pub mod ctb_sw_status;
#[doc = "Opamp0 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_offset_trim](oa0_offset_trim) module"]
pub type OA0_OFFSET_TRIM = crate::Reg<u32, _OA0_OFFSET_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA0_OFFSET_TRIM;
#[doc = "`read()` method returns [oa0_offset_trim::R](oa0_offset_trim::R) reader structure"]
impl crate::Readable for OA0_OFFSET_TRIM {}
#[doc = "`write(|w| ..)` method takes [oa0_offset_trim::W](oa0_offset_trim::W) writer structure"]
impl crate::Writable for OA0_OFFSET_TRIM {}
#[doc = "Opamp0 trim control"]
pub mod oa0_offset_trim;
#[doc = "Opamp0 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_slope_offset_trim](oa0_slope_offset_trim) module"]
pub type OA0_SLOPE_OFFSET_TRIM = crate::Reg<u32, _OA0_SLOPE_OFFSET_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA0_SLOPE_OFFSET_TRIM;
#[doc = "`read()` method returns [oa0_slope_offset_trim::R](oa0_slope_offset_trim::R) reader structure"]
impl crate::Readable for OA0_SLOPE_OFFSET_TRIM {}
#[doc = "`write(|w| ..)` method takes [oa0_slope_offset_trim::W](oa0_slope_offset_trim::W) writer structure"]
impl crate::Writable for OA0_SLOPE_OFFSET_TRIM {}
#[doc = "Opamp0 trim control"]
pub mod oa0_slope_offset_trim;
#[doc = "Opamp0 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_comp_trim](oa0_comp_trim) module"]
pub type OA0_COMP_TRIM = crate::Reg<u32, _OA0_COMP_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA0_COMP_TRIM;
#[doc = "`read()` method returns [oa0_comp_trim::R](oa0_comp_trim::R) reader structure"]
impl crate::Readable for OA0_COMP_TRIM {}
#[doc = "`write(|w| ..)` method takes [oa0_comp_trim::W](oa0_comp_trim::W) writer structure"]
impl crate::Writable for OA0_COMP_TRIM {}
#[doc = "Opamp0 trim control"]
pub mod oa0_comp_trim;
#[doc = "Opamp1 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_offset_trim](oa1_offset_trim) module"]
pub type OA1_OFFSET_TRIM = crate::Reg<u32, _OA1_OFFSET_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA1_OFFSET_TRIM;
#[doc = "`read()` method returns [oa1_offset_trim::R](oa1_offset_trim::R) reader structure"]
impl crate::Readable for OA1_OFFSET_TRIM {}
#[doc = "`write(|w| ..)` method takes [oa1_offset_trim::W](oa1_offset_trim::W) writer structure"]
impl crate::Writable for OA1_OFFSET_TRIM {}
#[doc = "Opamp1 trim control"]
pub mod oa1_offset_trim;
#[doc = "Opamp1 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_slope_offset_trim](oa1_slope_offset_trim) module"]
pub type OA1_SLOPE_OFFSET_TRIM = crate::Reg<u32, _OA1_SLOPE_OFFSET_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA1_SLOPE_OFFSET_TRIM;
#[doc = "`read()` method returns [oa1_slope_offset_trim::R](oa1_slope_offset_trim::R) reader structure"]
impl crate::Readable for OA1_SLOPE_OFFSET_TRIM {}
#[doc = "`write(|w| ..)` method takes [oa1_slope_offset_trim::W](oa1_slope_offset_trim::W) writer structure"]
impl crate::Writable for OA1_SLOPE_OFFSET_TRIM {}
#[doc = "Opamp1 trim control"]
pub mod oa1_slope_offset_trim;
#[doc = "Opamp1 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_comp_trim](oa1_comp_trim) module"]
pub type OA1_COMP_TRIM = crate::Reg<u32, _OA1_COMP_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OA1_COMP_TRIM;
#[doc = "`read()` method returns [oa1_comp_trim::R](oa1_comp_trim::R) reader structure"]
impl crate::Readable for OA1_COMP_TRIM {}
#[doc = "`write(|w| ..)` method takes [oa1_comp_trim::W](oa1_comp_trim::W) writer structure"]
impl crate::Writable for OA1_COMP_TRIM {}
#[doc = "Opamp1 trim control"]
pub mod oa1_comp_trim;

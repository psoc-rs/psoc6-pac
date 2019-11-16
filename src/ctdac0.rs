#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global CTDAC control"]
    pub ctdac_ctrl: CTDAC_CTRL,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - Interrupt request register"]
    pub intr: INTR,
    #[doc = "0x24 - Interrupt request set register"]
    pub intr_set: INTR_SET,
    #[doc = "0x28 - Interrupt request mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x2c - Interrupt request masked"]
    pub intr_masked: INTR_MASKED,
    _reserved5: [u8; 128usize],
    #[doc = "0xb0 - CTDAC switch control"]
    pub ctdac_sw: CTDAC_SW,
    #[doc = "0xb4 - CTDAC switch control clear"]
    pub ctdac_sw_clear: CTDAC_SW_CLEAR,
    _reserved7: [u8; 72usize],
    #[doc = "0x100 - DAC Value"]
    pub ctdac_val: CTDAC_VAL,
    #[doc = "0x104 - Next DAC value (double buffering)"]
    pub ctdac_val_nxt: CTDAC_VAL_NXT,
}
#[doc = "Global CTDAC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_ctrl](ctdac_ctrl) module"]
pub type CTDAC_CTRL = crate::Reg<u32, _CTDAC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTDAC_CTRL;
#[doc = "`read()` method returns [ctdac_ctrl::R](ctdac_ctrl::R) reader structure"]
impl crate::Readable for CTDAC_CTRL {}
#[doc = "`write(|w| ..)` method takes [ctdac_ctrl::W](ctdac_ctrl::W) writer structure"]
impl crate::Writable for CTDAC_CTRL {}
#[doc = "Global CTDAC control"]
pub mod ctdac_ctrl;
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
#[doc = "CTDAC switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_sw](ctdac_sw) module"]
pub type CTDAC_SW = crate::Reg<u32, _CTDAC_SW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTDAC_SW;
#[doc = "`read()` method returns [ctdac_sw::R](ctdac_sw::R) reader structure"]
impl crate::Readable for CTDAC_SW {}
#[doc = "`write(|w| ..)` method takes [ctdac_sw::W](ctdac_sw::W) writer structure"]
impl crate::Writable for CTDAC_SW {}
#[doc = "CTDAC switch control"]
pub mod ctdac_sw;
#[doc = "CTDAC switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_sw_clear](ctdac_sw_clear) module"]
pub type CTDAC_SW_CLEAR = crate::Reg<u32, _CTDAC_SW_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTDAC_SW_CLEAR;
#[doc = "`read()` method returns [ctdac_sw_clear::R](ctdac_sw_clear::R) reader structure"]
impl crate::Readable for CTDAC_SW_CLEAR {}
#[doc = "`write(|w| ..)` method takes [ctdac_sw_clear::W](ctdac_sw_clear::W) writer structure"]
impl crate::Writable for CTDAC_SW_CLEAR {}
#[doc = "CTDAC switch control clear"]
pub mod ctdac_sw_clear;
#[doc = "DAC Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_val](ctdac_val) module"]
pub type CTDAC_VAL = crate::Reg<u32, _CTDAC_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTDAC_VAL;
#[doc = "`read()` method returns [ctdac_val::R](ctdac_val::R) reader structure"]
impl crate::Readable for CTDAC_VAL {}
#[doc = "`write(|w| ..)` method takes [ctdac_val::W](ctdac_val::W) writer structure"]
impl crate::Writable for CTDAC_VAL {}
#[doc = "DAC Value"]
pub mod ctdac_val;
#[doc = "Next DAC value (double buffering)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_val_nxt](ctdac_val_nxt) module"]
pub type CTDAC_VAL_NXT = crate::Reg<u32, _CTDAC_VAL_NXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTDAC_VAL_NXT;
#[doc = "`read()` method returns [ctdac_val_nxt::R](ctdac_val_nxt::R) reader structure"]
impl crate::Readable for CTDAC_VAL_NXT {}
#[doc = "`write(|w| ..)` method takes [ctdac_val_nxt::W](ctdac_val_nxt::W) writer structure"]
impl crate::Writable for CTDAC_VAL_NXT {}
#[doc = "Next DAC value (double buffering)"]
pub mod ctdac_val_nxt;

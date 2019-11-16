#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LPCOMP Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x04 - LPCOMP Status Register"]
    pub status: STATUS,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - LPCOMP Interrupt request register"]
    pub intr: INTR,
    #[doc = "0x14 - LPCOMP Interrupt set register"]
    pub intr_set: INTR_SET,
    #[doc = "0x18 - LPCOMP Interrupt request mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x1c - LPCOMP Interrupt request masked"]
    pub intr_masked: INTR_MASKED,
    _reserved6: [u8; 32usize],
    #[doc = "0x40 - Comparator 0 control Register"]
    pub cmp0_ctrl: CMP0_CTRL,
    _reserved7: [u8; 12usize],
    #[doc = "0x50 - Comparator 0 switch control"]
    pub cmp0_sw: CMP0_SW,
    #[doc = "0x54 - Comparator 0 switch control clear"]
    pub cmp0_sw_clear: CMP0_SW_CLEAR,
    _reserved9: [u8; 40usize],
    #[doc = "0x80 - Comparator 1 control Register"]
    pub cmp1_ctrl: CMP1_CTRL,
    _reserved10: [u8; 12usize],
    #[doc = "0x90 - Comparator 1 switch control"]
    pub cmp1_sw: CMP1_SW,
    #[doc = "0x94 - Comparator 1 switch control clear"]
    pub cmp1_sw_clear: CMP1_SW_CLEAR,
}
#[doc = "LPCOMP Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "LPCOMP Configuration Register"]
pub mod config;
#[doc = "LPCOMP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "LPCOMP Status Register"]
pub mod status;
#[doc = "LPCOMP Interrupt request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "LPCOMP Interrupt request register"]
pub mod intr;
#[doc = "LPCOMP Interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "LPCOMP Interrupt set register"]
pub mod intr_set;
#[doc = "LPCOMP Interrupt request mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "LPCOMP Interrupt request mask"]
pub mod intr_mask;
#[doc = "LPCOMP Interrupt request masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "LPCOMP Interrupt request masked"]
pub mod intr_masked;
#[doc = "Comparator 0 control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp0_ctrl](cmp0_ctrl) module"]
pub type CMP0_CTRL = crate::Reg<u32, _CMP0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP0_CTRL;
#[doc = "`read()` method returns [cmp0_ctrl::R](cmp0_ctrl::R) reader structure"]
impl crate::Readable for CMP0_CTRL {}
#[doc = "`write(|w| ..)` method takes [cmp0_ctrl::W](cmp0_ctrl::W) writer structure"]
impl crate::Writable for CMP0_CTRL {}
#[doc = "Comparator 0 control Register"]
pub mod cmp0_ctrl;
#[doc = "Comparator 0 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp0_sw](cmp0_sw) module"]
pub type CMP0_SW = crate::Reg<u32, _CMP0_SW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP0_SW;
#[doc = "`read()` method returns [cmp0_sw::R](cmp0_sw::R) reader structure"]
impl crate::Readable for CMP0_SW {}
#[doc = "`write(|w| ..)` method takes [cmp0_sw::W](cmp0_sw::W) writer structure"]
impl crate::Writable for CMP0_SW {}
#[doc = "Comparator 0 switch control"]
pub mod cmp0_sw;
#[doc = "Comparator 0 switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp0_sw_clear](cmp0_sw_clear) module"]
pub type CMP0_SW_CLEAR = crate::Reg<u32, _CMP0_SW_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP0_SW_CLEAR;
#[doc = "`read()` method returns [cmp0_sw_clear::R](cmp0_sw_clear::R) reader structure"]
impl crate::Readable for CMP0_SW_CLEAR {}
#[doc = "`write(|w| ..)` method takes [cmp0_sw_clear::W](cmp0_sw_clear::W) writer structure"]
impl crate::Writable for CMP0_SW_CLEAR {}
#[doc = "Comparator 0 switch control clear"]
pub mod cmp0_sw_clear;
#[doc = "Comparator 1 control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1_ctrl](cmp1_ctrl) module"]
pub type CMP1_CTRL = crate::Reg<u32, _CMP1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1_CTRL;
#[doc = "`read()` method returns [cmp1_ctrl::R](cmp1_ctrl::R) reader structure"]
impl crate::Readable for CMP1_CTRL {}
#[doc = "`write(|w| ..)` method takes [cmp1_ctrl::W](cmp1_ctrl::W) writer structure"]
impl crate::Writable for CMP1_CTRL {}
#[doc = "Comparator 1 control Register"]
pub mod cmp1_ctrl;
#[doc = "Comparator 1 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1_sw](cmp1_sw) module"]
pub type CMP1_SW = crate::Reg<u32, _CMP1_SW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1_SW;
#[doc = "`read()` method returns [cmp1_sw::R](cmp1_sw::R) reader structure"]
impl crate::Readable for CMP1_SW {}
#[doc = "`write(|w| ..)` method takes [cmp1_sw::W](cmp1_sw::W) writer structure"]
impl crate::Writable for CMP1_SW {}
#[doc = "Comparator 1 switch control"]
pub mod cmp1_sw;
#[doc = "Comparator 1 switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1_sw_clear](cmp1_sw_clear) module"]
pub type CMP1_SW_CLEAR = crate::Reg<u32, _CMP1_SW_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1_SW_CLEAR;
#[doc = "`read()` method returns [cmp1_sw_clear::R](cmp1_sw_clear::R) reader structure"]
impl crate::Readable for CMP1_SW_CLEAR {}
#[doc = "`write(|w| ..)` method takes [cmp1_sw_clear::W](cmp1_sw_clear::W) writer structure"]
impl crate::Writable for CMP1_SW_CLEAR {}
#[doc = "Comparator 1 switch control clear"]
pub mod cmp1_sw_clear;

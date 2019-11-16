#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Synchronization control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync_ctl](sync_ctl) module"]
pub type SYNC_CTL = crate::Reg<u32, _SYNC_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC_CTL;
#[doc = "`read()` method returns [sync_ctl::R](sync_ctl::R) reader structure"]
impl crate::Readable for SYNC_CTL {}
#[doc = "`write(|w| ..)` method takes [sync_ctl::W](sync_ctl::W) writer structure"]
impl crate::Writable for SYNC_CTL {}
#[doc = "Synchronization control register"]
pub mod sync_ctl;
#[doc = "LUT component input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut_sel](lut_sel) module"]
pub type LUT_SEL = crate::Reg<u32, _LUT_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT_SEL;
#[doc = "`read()` method returns [lut_sel::R](lut_sel::R) reader structure"]
impl crate::Readable for LUT_SEL {}
#[doc = "`write(|w| ..)` method takes [lut_sel::W](lut_sel::W) writer structure"]
impl crate::Writable for LUT_SEL {}
#[doc = "LUT component input selection"]
pub mod lut_sel;
#[doc = "LUT component control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut_ctl](lut_ctl) module"]
pub type LUT_CTL = crate::Reg<u32, _LUT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT_CTL;
#[doc = "`read()` method returns [lut_ctl::R](lut_ctl::R) reader structure"]
impl crate::Readable for LUT_CTL {}
#[doc = "`write(|w| ..)` method takes [lut_ctl::W](lut_ctl::W) writer structure"]
impl crate::Writable for LUT_CTL {}
#[doc = "LUT component control register"]
pub mod lut_ctl;
#[doc = "Data unit component input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [du_sel](du_sel) module"]
pub type DU_SEL = crate::Reg<u32, _DU_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DU_SEL;
#[doc = "`read()` method returns [du_sel::R](du_sel::R) reader structure"]
impl crate::Readable for DU_SEL {}
#[doc = "`write(|w| ..)` method takes [du_sel::W](du_sel::W) writer structure"]
impl crate::Writable for DU_SEL {}
#[doc = "Data unit component input selection"]
pub mod du_sel;
#[doc = "Data unit component control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [du_ctl](du_ctl) module"]
pub type DU_CTL = crate::Reg<u32, _DU_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DU_CTL;
#[doc = "`read()` method returns [du_ctl::R](du_ctl::R) reader structure"]
impl crate::Readable for DU_CTL {}
#[doc = "`write(|w| ..)` method takes [du_ctl::W](du_ctl::W) writer structure"]
impl crate::Writable for DU_CTL {}
#[doc = "Data unit component control register"]
pub mod du_ctl;
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Data register"]
pub mod data;

#[doc = "Fault control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Fault control"]
pub mod ctl;
#[doc = "Fault status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Fault status"]
pub mod status;
#[doc = "Fault data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "Fault data"]
pub mod data;
#[doc = "Fault pending 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending0](pending0) module"]
pub type PENDING0 = crate::Reg<u32, _PENDING0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PENDING0;
#[doc = "`read()` method returns [pending0::R](pending0::R) reader structure"]
impl crate::Readable for PENDING0 {}
#[doc = "Fault pending 0"]
pub mod pending0;
#[doc = "Fault pending 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending1](pending1) module"]
pub type PENDING1 = crate::Reg<u32, _PENDING1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PENDING1;
#[doc = "`read()` method returns [pending1::R](pending1::R) reader structure"]
impl crate::Readable for PENDING1 {}
#[doc = "Fault pending 1"]
pub mod pending1;
#[doc = "Fault pending 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending2](pending2) module"]
pub type PENDING2 = crate::Reg<u32, _PENDING2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PENDING2;
#[doc = "`read()` method returns [pending2::R](pending2::R) reader structure"]
impl crate::Readable for PENDING2 {}
#[doc = "Fault pending 2"]
pub mod pending2;
#[doc = "Fault mask 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask0](mask0) module"]
pub type MASK0 = crate::Reg<u32, _MASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK0;
#[doc = "`read()` method returns [mask0::R](mask0::R) reader structure"]
impl crate::Readable for MASK0 {}
#[doc = "`write(|w| ..)` method takes [mask0::W](mask0::W) writer structure"]
impl crate::Writable for MASK0 {}
#[doc = "Fault mask 0"]
pub mod mask0;
#[doc = "Fault mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask1](mask1) module"]
pub type MASK1 = crate::Reg<u32, _MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK1;
#[doc = "`read()` method returns [mask1::R](mask1::R) reader structure"]
impl crate::Readable for MASK1 {}
#[doc = "`write(|w| ..)` method takes [mask1::W](mask1::W) writer structure"]
impl crate::Writable for MASK1 {}
#[doc = "Fault mask 1"]
pub mod mask1;
#[doc = "Fault mask 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask2](mask2) module"]
pub type MASK2 = crate::Reg<u32, _MASK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK2;
#[doc = "`read()` method returns [mask2::R](mask2::R) reader structure"]
impl crate::Readable for MASK2 {}
#[doc = "`write(|w| ..)` method takes [mask2::W](mask2::W) writer structure"]
impl crate::Writable for MASK2 {}
#[doc = "Fault mask 2"]
pub mod mask2;
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt"]
pub mod intr;
#[doc = "Interrupt set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "Interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Interrupt masked"]
pub mod intr_masked;

#[doc = "Counter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Counter control register"]
pub mod ctrl;
#[doc = "Counter status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Counter status register"]
pub mod status;
#[doc = "Counter count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter](counter) module"]
pub type COUNTER = crate::Reg<u32, _COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTER;
#[doc = "`read()` method returns [counter::R](counter::R) reader structure"]
impl crate::Readable for COUNTER {}
#[doc = "`write(|w| ..)` method takes [counter::W](counter::W) writer structure"]
impl crate::Writable for COUNTER {}
#[doc = "Counter count register"]
pub mod counter;
#[doc = "Counter compare/capture register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Counter compare/capture register"]
pub mod cc;
#[doc = "Counter buffered compare/capture register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_buff](cc_buff) module"]
pub type CC_BUFF = crate::Reg<u32, _CC_BUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_BUFF;
#[doc = "`read()` method returns [cc_buff::R](cc_buff::R) reader structure"]
impl crate::Readable for CC_BUFF {}
#[doc = "`write(|w| ..)` method takes [cc_buff::W](cc_buff::W) writer structure"]
impl crate::Writable for CC_BUFF {}
#[doc = "Counter buffered compare/capture register"]
pub mod cc_buff;
#[doc = "Counter period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period](period) module"]
pub type PERIOD = crate::Reg<u32, _PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIOD;
#[doc = "`read()` method returns [period::R](period::R) reader structure"]
impl crate::Readable for PERIOD {}
#[doc = "`write(|w| ..)` method takes [period::W](period::W) writer structure"]
impl crate::Writable for PERIOD {}
#[doc = "Counter period register"]
pub mod period;
#[doc = "Counter buffered period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period_buff](period_buff) module"]
pub type PERIOD_BUFF = crate::Reg<u32, _PERIOD_BUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIOD_BUFF;
#[doc = "`read()` method returns [period_buff::R](period_buff::R) reader structure"]
impl crate::Readable for PERIOD_BUFF {}
#[doc = "`write(|w| ..)` method takes [period_buff::W](period_buff::W) writer structure"]
impl crate::Writable for PERIOD_BUFF {}
#[doc = "Counter buffered period register"]
pub mod period_buff;
#[doc = "Counter trigger control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctrl0](tr_ctrl0) module"]
pub type TR_CTRL0 = crate::Reg<u32, _TR_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CTRL0;
#[doc = "`read()` method returns [tr_ctrl0::R](tr_ctrl0::R) reader structure"]
impl crate::Readable for TR_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [tr_ctrl0::W](tr_ctrl0::W) writer structure"]
impl crate::Writable for TR_CTRL0 {}
#[doc = "Counter trigger control register 0"]
pub mod tr_ctrl0;
#[doc = "Counter trigger control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctrl1](tr_ctrl1) module"]
pub type TR_CTRL1 = crate::Reg<u32, _TR_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CTRL1;
#[doc = "`read()` method returns [tr_ctrl1::R](tr_ctrl1::R) reader structure"]
impl crate::Readable for TR_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [tr_ctrl1::W](tr_ctrl1::W) writer structure"]
impl crate::Writable for TR_CTRL1 {}
#[doc = "Counter trigger control register 1"]
pub mod tr_ctrl1;
#[doc = "Counter trigger control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctrl2](tr_ctrl2) module"]
pub type TR_CTRL2 = crate::Reg<u32, _TR_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CTRL2;
#[doc = "`read()` method returns [tr_ctrl2::R](tr_ctrl2::R) reader structure"]
impl crate::Readable for TR_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [tr_ctrl2::W](tr_ctrl2::W) writer structure"]
impl crate::Writable for TR_CTRL2 {}
#[doc = "Counter trigger control register 2"]
pub mod tr_ctrl2;
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
#[doc = "Interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "Interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Interrupt masked request register"]
pub mod intr_masked;

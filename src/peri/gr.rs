#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](clock_ctl) module"]
pub type CLOCK_CTL = crate::Reg<u32, _CLOCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_CTL;
#[doc = "`read()` method returns [clock_ctl::R](clock_ctl::R) reader structure"]
impl crate::Readable for CLOCK_CTL {}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](clock_ctl::W) writer structure"]
impl crate::Writable for CLOCK_CTL {}
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "Slave control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_ctl](sl_ctl) module"]
pub type SL_CTL = crate::Reg<u32, _SL_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL_CTL;
#[doc = "`read()` method returns [sl_ctl::R](sl_ctl::R) reader structure"]
impl crate::Readable for SL_CTL {}
#[doc = "`write(|w| ..)` method takes [sl_ctl::W](sl_ctl::W) writer structure"]
impl crate::Writable for SL_CTL {}
#[doc = "Slave control"]
pub mod sl_ctl;
#[doc = "Timeout control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout_ctl](timeout_ctl) module"]
pub type TIMEOUT_CTL = crate::Reg<u32, _TIMEOUT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT_CTL;
#[doc = "`read()` method returns [timeout_ctl::R](timeout_ctl::R) reader structure"]
impl crate::Readable for TIMEOUT_CTL {}
#[doc = "`write(|w| ..)` method takes [timeout_ctl::W](timeout_ctl::W) writer structure"]
impl crate::Writable for TIMEOUT_CTL {}
#[doc = "Timeout control"]
pub mod timeout_ctl;

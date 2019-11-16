#[doc = "Multi-Counter Watchdog Sub-counters 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_cntlow](mcwdt_cntlow) module"]
pub type MCWDT_CNTLOW = crate::Reg<u32, _MCWDT_CNTLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_CNTLOW;
#[doc = "`read()` method returns [mcwdt_cntlow::R](mcwdt_cntlow::R) reader structure"]
impl crate::Readable for MCWDT_CNTLOW {}
#[doc = "`write(|w| ..)` method takes [mcwdt_cntlow::W](mcwdt_cntlow::W) writer structure"]
impl crate::Writable for MCWDT_CNTLOW {}
#[doc = "Multi-Counter Watchdog Sub-counters 0/1"]
pub mod mcwdt_cntlow;
#[doc = "Multi-Counter Watchdog Sub-counter 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_cnthigh](mcwdt_cnthigh) module"]
pub type MCWDT_CNTHIGH = crate::Reg<u32, _MCWDT_CNTHIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_CNTHIGH;
#[doc = "`read()` method returns [mcwdt_cnthigh::R](mcwdt_cnthigh::R) reader structure"]
impl crate::Readable for MCWDT_CNTHIGH {}
#[doc = "`write(|w| ..)` method takes [mcwdt_cnthigh::W](mcwdt_cnthigh::W) writer structure"]
impl crate::Writable for MCWDT_CNTHIGH {}
#[doc = "Multi-Counter Watchdog Sub-counter 2"]
pub mod mcwdt_cnthigh;
#[doc = "Multi-Counter Watchdog Counter Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_match](mcwdt_match) module"]
pub type MCWDT_MATCH = crate::Reg<u32, _MCWDT_MATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_MATCH;
#[doc = "`read()` method returns [mcwdt_match::R](mcwdt_match::R) reader structure"]
impl crate::Readable for MCWDT_MATCH {}
#[doc = "`write(|w| ..)` method takes [mcwdt_match::W](mcwdt_match::W) writer structure"]
impl crate::Writable for MCWDT_MATCH {}
#[doc = "Multi-Counter Watchdog Counter Match Register"]
pub mod mcwdt_match;
#[doc = "Multi-Counter Watchdog Counter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_config](mcwdt_config) module"]
pub type MCWDT_CONFIG = crate::Reg<u32, _MCWDT_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_CONFIG;
#[doc = "`read()` method returns [mcwdt_config::R](mcwdt_config::R) reader structure"]
impl crate::Readable for MCWDT_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mcwdt_config::W](mcwdt_config::W) writer structure"]
impl crate::Writable for MCWDT_CONFIG {}
#[doc = "Multi-Counter Watchdog Counter Configuration"]
pub mod mcwdt_config;
#[doc = "Multi-Counter Watchdog Counter Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_ctl](mcwdt_ctl) module"]
pub type MCWDT_CTL = crate::Reg<u32, _MCWDT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_CTL;
#[doc = "`read()` method returns [mcwdt_ctl::R](mcwdt_ctl::R) reader structure"]
impl crate::Readable for MCWDT_CTL {}
#[doc = "`write(|w| ..)` method takes [mcwdt_ctl::W](mcwdt_ctl::W) writer structure"]
impl crate::Writable for MCWDT_CTL {}
#[doc = "Multi-Counter Watchdog Counter Control"]
pub mod mcwdt_ctl;
#[doc = "Multi-Counter Watchdog Counter Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_intr](mcwdt_intr) module"]
pub type MCWDT_INTR = crate::Reg<u32, _MCWDT_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_INTR;
#[doc = "`read()` method returns [mcwdt_intr::R](mcwdt_intr::R) reader structure"]
impl crate::Readable for MCWDT_INTR {}
#[doc = "`write(|w| ..)` method takes [mcwdt_intr::W](mcwdt_intr::W) writer structure"]
impl crate::Writable for MCWDT_INTR {}
#[doc = "Multi-Counter Watchdog Counter Interrupt Register"]
pub mod mcwdt_intr;
#[doc = "Multi-Counter Watchdog Counter Interrupt Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_intr_set](mcwdt_intr_set) module"]
pub type MCWDT_INTR_SET = crate::Reg<u32, _MCWDT_INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_INTR_SET;
#[doc = "`read()` method returns [mcwdt_intr_set::R](mcwdt_intr_set::R) reader structure"]
impl crate::Readable for MCWDT_INTR_SET {}
#[doc = "`write(|w| ..)` method takes [mcwdt_intr_set::W](mcwdt_intr_set::W) writer structure"]
impl crate::Writable for MCWDT_INTR_SET {}
#[doc = "Multi-Counter Watchdog Counter Interrupt Set Register"]
pub mod mcwdt_intr_set;
#[doc = "Multi-Counter Watchdog Counter Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_intr_mask](mcwdt_intr_mask) module"]
pub type MCWDT_INTR_MASK = crate::Reg<u32, _MCWDT_INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_INTR_MASK;
#[doc = "`read()` method returns [mcwdt_intr_mask::R](mcwdt_intr_mask::R) reader structure"]
impl crate::Readable for MCWDT_INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [mcwdt_intr_mask::W](mcwdt_intr_mask::W) writer structure"]
impl crate::Writable for MCWDT_INTR_MASK {}
#[doc = "Multi-Counter Watchdog Counter Interrupt Mask Register"]
pub mod mcwdt_intr_mask;
#[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_intr_masked](mcwdt_intr_masked) module"]
pub type MCWDT_INTR_MASKED = crate::Reg<u32, _MCWDT_INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_INTR_MASKED;
#[doc = "`read()` method returns [mcwdt_intr_masked::R](mcwdt_intr_masked::R) reader structure"]
impl crate::Readable for MCWDT_INTR_MASKED {}
#[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register"]
pub mod mcwdt_intr_masked;
#[doc = "Multi-Counter Watchdog Counter Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_lock](mcwdt_lock) module"]
pub type MCWDT_LOCK = crate::Reg<u32, _MCWDT_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCWDT_LOCK;
#[doc = "`read()` method returns [mcwdt_lock::R](mcwdt_lock::R) reader structure"]
impl crate::Readable for MCWDT_LOCK {}
#[doc = "`write(|w| ..)` method takes [mcwdt_lock::W](mcwdt_lock::W) writer structure"]
impl crate::Writable for MCWDT_LOCK {}
#[doc = "Multi-Counter Watchdog Counter Lock Register"]
pub mod mcwdt_lock;

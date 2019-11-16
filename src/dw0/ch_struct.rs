#[doc = "Channel control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ctl](ch_ctl) module"]
pub type CH_CTL = crate::Reg<u32, _CH_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_CTL;
#[doc = "`read()` method returns [ch_ctl::R](ch_ctl::R) reader structure"]
impl crate::Readable for CH_CTL {}
#[doc = "`write(|w| ..)` method takes [ch_ctl::W](ch_ctl::W) writer structure"]
impl crate::Writable for CH_CTL {}
#[doc = "Channel control"]
pub mod ch_ctl;
#[doc = "Channel status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_status](ch_status) module"]
pub type CH_STATUS = crate::Reg<u32, _CH_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_STATUS;
#[doc = "`read()` method returns [ch_status::R](ch_status::R) reader structure"]
impl crate::Readable for CH_STATUS {}
#[doc = "Channel status"]
pub mod ch_status;
#[doc = "Channel current indices\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_idx](ch_idx) module"]
pub type CH_IDX = crate::Reg<u32, _CH_IDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_IDX;
#[doc = "`read()` method returns [ch_idx::R](ch_idx::R) reader structure"]
impl crate::Readable for CH_IDX {}
#[doc = "`write(|w| ..)` method takes [ch_idx::W](ch_idx::W) writer structure"]
impl crate::Writable for CH_IDX {}
#[doc = "Channel current indices"]
pub mod ch_idx;
#[doc = "Channel current descriptor pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_curr_ptr](ch_curr_ptr) module"]
pub type CH_CURR_PTR = crate::Reg<u32, _CH_CURR_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_CURR_PTR;
#[doc = "`read()` method returns [ch_curr_ptr::R](ch_curr_ptr::R) reader structure"]
impl crate::Readable for CH_CURR_PTR {}
#[doc = "`write(|w| ..)` method takes [ch_curr_ptr::W](ch_curr_ptr::W) writer structure"]
impl crate::Writable for CH_CURR_PTR {}
#[doc = "Channel current descriptor pointer"]
pub mod ch_curr_ptr;
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

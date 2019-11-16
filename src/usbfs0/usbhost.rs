#[doc = "Host Control 0 Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctl0](host_ctl0) module"]
pub type HOST_CTL0 = crate::Reg<u32, _HOST_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CTL0;
#[doc = "`read()` method returns [host_ctl0::R](host_ctl0::R) reader structure"]
impl crate::Readable for HOST_CTL0 {}
#[doc = "`write(|w| ..)` method takes [host_ctl0::W](host_ctl0::W) writer structure"]
impl crate::Writable for HOST_CTL0 {}
#[doc = "Host Control 0 Register."]
pub mod host_ctl0;
#[doc = "Host Control 1 Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctl1](host_ctl1) module"]
pub type HOST_CTL1 = crate::Reg<u32, _HOST_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CTL1;
#[doc = "`read()` method returns [host_ctl1::R](host_ctl1::R) reader structure"]
impl crate::Readable for HOST_CTL1 {}
#[doc = "`write(|w| ..)` method takes [host_ctl1::W](host_ctl1::W) writer structure"]
impl crate::Writable for HOST_CTL1 {}
#[doc = "Host Control 1 Register."]
pub mod host_ctl1;
#[doc = "Host Control 2 Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctl2](host_ctl2) module"]
pub type HOST_CTL2 = crate::Reg<u32, _HOST_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CTL2;
#[doc = "`read()` method returns [host_ctl2::R](host_ctl2::R) reader structure"]
impl crate::Readable for HOST_CTL2 {}
#[doc = "`write(|w| ..)` method takes [host_ctl2::W](host_ctl2::W) writer structure"]
impl crate::Writable for HOST_CTL2 {}
#[doc = "Host Control 2 Register."]
pub mod host_ctl2;
#[doc = "Host Error Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_err](host_err) module"]
pub type HOST_ERR = crate::Reg<u32, _HOST_ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_ERR;
#[doc = "`read()` method returns [host_err::R](host_err::R) reader structure"]
impl crate::Readable for HOST_ERR {}
#[doc = "`write(|w| ..)` method takes [host_err::W](host_err::W) writer structure"]
impl crate::Writable for HOST_ERR {}
#[doc = "Host Error Status Register."]
pub mod host_err;
#[doc = "Host Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_status](host_status) module"]
pub type HOST_STATUS = crate::Reg<u32, _HOST_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_STATUS;
#[doc = "`read()` method returns [host_status::R](host_status::R) reader structure"]
impl crate::Readable for HOST_STATUS {}
#[doc = "`write(|w| ..)` method takes [host_status::W](host_status::W) writer structure"]
impl crate::Writable for HOST_STATUS {}
#[doc = "Host Status Register."]
pub mod host_status;
#[doc = "Host SOF Interrupt Frame Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_fcomp](host_fcomp) module"]
pub type HOST_FCOMP = crate::Reg<u32, _HOST_FCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_FCOMP;
#[doc = "`read()` method returns [host_fcomp::R](host_fcomp::R) reader structure"]
impl crate::Readable for HOST_FCOMP {}
#[doc = "`write(|w| ..)` method takes [host_fcomp::W](host_fcomp::W) writer structure"]
impl crate::Writable for HOST_FCOMP {}
#[doc = "Host SOF Interrupt Frame Compare Register"]
pub mod host_fcomp;
#[doc = "Host Retry Timer Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_rtimer](host_rtimer) module"]
pub type HOST_RTIMER = crate::Reg<u32, _HOST_RTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_RTIMER;
#[doc = "`read()` method returns [host_rtimer::R](host_rtimer::R) reader structure"]
impl crate::Readable for HOST_RTIMER {}
#[doc = "`write(|w| ..)` method takes [host_rtimer::W](host_rtimer::W) writer structure"]
impl crate::Writable for HOST_RTIMER {}
#[doc = "Host Retry Timer Setup Register"]
pub mod host_rtimer;
#[doc = "Host Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_addr](host_addr) module"]
pub type HOST_ADDR = crate::Reg<u32, _HOST_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_ADDR;
#[doc = "`read()` method returns [host_addr::R](host_addr::R) reader structure"]
impl crate::Readable for HOST_ADDR {}
#[doc = "`write(|w| ..)` method takes [host_addr::W](host_addr::W) writer structure"]
impl crate::Writable for HOST_ADDR {}
#[doc = "Host Address Register"]
pub mod host_addr;
#[doc = "Host EOF Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_eof](host_eof) module"]
pub type HOST_EOF = crate::Reg<u32, _HOST_EOF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EOF;
#[doc = "`read()` method returns [host_eof::R](host_eof::R) reader structure"]
impl crate::Readable for HOST_EOF {}
#[doc = "`write(|w| ..)` method takes [host_eof::W](host_eof::W) writer structure"]
impl crate::Writable for HOST_EOF {}
#[doc = "Host EOF Setup Register"]
pub mod host_eof;
#[doc = "Host Frame Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_frame](host_frame) module"]
pub type HOST_FRAME = crate::Reg<u32, _HOST_FRAME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_FRAME;
#[doc = "`read()` method returns [host_frame::R](host_frame::R) reader structure"]
impl crate::Readable for HOST_FRAME {}
#[doc = "`write(|w| ..)` method takes [host_frame::W](host_frame::W) writer structure"]
impl crate::Writable for HOST_FRAME {}
#[doc = "Host Frame Setup Register"]
pub mod host_frame;
#[doc = "Host Token Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_token](host_token) module"]
pub type HOST_TOKEN = crate::Reg<u32, _HOST_TOKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_TOKEN;
#[doc = "`read()` method returns [host_token::R](host_token::R) reader structure"]
impl crate::Readable for HOST_TOKEN {}
#[doc = "`write(|w| ..)` method takes [host_token::W](host_token::W) writer structure"]
impl crate::Writable for HOST_TOKEN {}
#[doc = "Host Token Endpoint Register"]
pub mod host_token;
#[doc = "Host Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep1_ctl](host_ep1_ctl) module"]
pub type HOST_EP1_CTL = crate::Reg<u32, _HOST_EP1_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP1_CTL;
#[doc = "`read()` method returns [host_ep1_ctl::R](host_ep1_ctl::R) reader structure"]
impl crate::Readable for HOST_EP1_CTL {}
#[doc = "`write(|w| ..)` method takes [host_ep1_ctl::W](host_ep1_ctl::W) writer structure"]
impl crate::Writable for HOST_EP1_CTL {}
#[doc = "Host Endpoint 1 Control Register"]
pub mod host_ep1_ctl;
#[doc = "Host Endpoint 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep1_status](host_ep1_status) module"]
pub type HOST_EP1_STATUS = crate::Reg<u32, _HOST_EP1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP1_STATUS;
#[doc = "`read()` method returns [host_ep1_status::R](host_ep1_status::R) reader structure"]
impl crate::Readable for HOST_EP1_STATUS {}
#[doc = "Host Endpoint 1 Status Register"]
pub mod host_ep1_status;
#[doc = "Host Endpoint 1 Data 1-Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep1_rw1_dr](host_ep1_rw1_dr) module"]
pub type HOST_EP1_RW1_DR = crate::Reg<u32, _HOST_EP1_RW1_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP1_RW1_DR;
#[doc = "`read()` method returns [host_ep1_rw1_dr::R](host_ep1_rw1_dr::R) reader structure"]
impl crate::Readable for HOST_EP1_RW1_DR {}
#[doc = "`write(|w| ..)` method takes [host_ep1_rw1_dr::W](host_ep1_rw1_dr::W) writer structure"]
impl crate::Writable for HOST_EP1_RW1_DR {}
#[doc = "Host Endpoint 1 Data 1-Byte Register"]
pub mod host_ep1_rw1_dr;
#[doc = "Host Endpoint 1 Data 2-Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep1_rw2_dr](host_ep1_rw2_dr) module"]
pub type HOST_EP1_RW2_DR = crate::Reg<u32, _HOST_EP1_RW2_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP1_RW2_DR;
#[doc = "`read()` method returns [host_ep1_rw2_dr::R](host_ep1_rw2_dr::R) reader structure"]
impl crate::Readable for HOST_EP1_RW2_DR {}
#[doc = "`write(|w| ..)` method takes [host_ep1_rw2_dr::W](host_ep1_rw2_dr::W) writer structure"]
impl crate::Writable for HOST_EP1_RW2_DR {}
#[doc = "Host Endpoint 1 Data 2-Byte Register"]
pub mod host_ep1_rw2_dr;
#[doc = "Host Endpoint 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep2_ctl](host_ep2_ctl) module"]
pub type HOST_EP2_CTL = crate::Reg<u32, _HOST_EP2_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP2_CTL;
#[doc = "`read()` method returns [host_ep2_ctl::R](host_ep2_ctl::R) reader structure"]
impl crate::Readable for HOST_EP2_CTL {}
#[doc = "`write(|w| ..)` method takes [host_ep2_ctl::W](host_ep2_ctl::W) writer structure"]
impl crate::Writable for HOST_EP2_CTL {}
#[doc = "Host Endpoint 2 Control Register"]
pub mod host_ep2_ctl;
#[doc = "Host Endpoint 2 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep2_status](host_ep2_status) module"]
pub type HOST_EP2_STATUS = crate::Reg<u32, _HOST_EP2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP2_STATUS;
#[doc = "`read()` method returns [host_ep2_status::R](host_ep2_status::R) reader structure"]
impl crate::Readable for HOST_EP2_STATUS {}
#[doc = "Host Endpoint 2 Status Register"]
pub mod host_ep2_status;
#[doc = "Host Endpoint 2 Data 1-Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep2_rw1_dr](host_ep2_rw1_dr) module"]
pub type HOST_EP2_RW1_DR = crate::Reg<u32, _HOST_EP2_RW1_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP2_RW1_DR;
#[doc = "`read()` method returns [host_ep2_rw1_dr::R](host_ep2_rw1_dr::R) reader structure"]
impl crate::Readable for HOST_EP2_RW1_DR {}
#[doc = "`write(|w| ..)` method takes [host_ep2_rw1_dr::W](host_ep2_rw1_dr::W) writer structure"]
impl crate::Writable for HOST_EP2_RW1_DR {}
#[doc = "Host Endpoint 2 Data 1-Byte Register"]
pub mod host_ep2_rw1_dr;
#[doc = "Host Endpoint 2 Data 2-Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep2_rw2_dr](host_ep2_rw2_dr) module"]
pub type HOST_EP2_RW2_DR = crate::Reg<u32, _HOST_EP2_RW2_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP2_RW2_DR;
#[doc = "`read()` method returns [host_ep2_rw2_dr::R](host_ep2_rw2_dr::R) reader structure"]
impl crate::Readable for HOST_EP2_RW2_DR {}
#[doc = "`write(|w| ..)` method takes [host_ep2_rw2_dr::W](host_ep2_rw2_dr::W) writer structure"]
impl crate::Writable for HOST_EP2_RW2_DR {}
#[doc = "Host Endpoint 2 Data 2-Byte Register"]
pub mod host_ep2_rw2_dr;
#[doc = "Host Interrupt Level 1 Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_lvl1_sel](host_lvl1_sel) module"]
pub type HOST_LVL1_SEL = crate::Reg<u32, _HOST_LVL1_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_LVL1_SEL;
#[doc = "`read()` method returns [host_lvl1_sel::R](host_lvl1_sel::R) reader structure"]
impl crate::Readable for HOST_LVL1_SEL {}
#[doc = "`write(|w| ..)` method takes [host_lvl1_sel::W](host_lvl1_sel::W) writer structure"]
impl crate::Writable for HOST_LVL1_SEL {}
#[doc = "Host Interrupt Level 1 Selection Register"]
pub mod host_lvl1_sel;
#[doc = "Host Interrupt Level 2 Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_lvl2_sel](host_lvl2_sel) module"]
pub type HOST_LVL2_SEL = crate::Reg<u32, _HOST_LVL2_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_LVL2_SEL;
#[doc = "`read()` method returns [host_lvl2_sel::R](host_lvl2_sel::R) reader structure"]
impl crate::Readable for HOST_LVL2_SEL {}
#[doc = "`write(|w| ..)` method takes [host_lvl2_sel::W](host_lvl2_sel::W) writer structure"]
impl crate::Writable for HOST_LVL2_SEL {}
#[doc = "Host Interrupt Level 2 Selection Register"]
pub mod host_lvl2_sel;
#[doc = "Interrupt USB Host Cause High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_cause_hi](intr_usbhost_cause_hi) module"]
pub type INTR_USBHOST_CAUSE_HI = crate::Reg<u32, _INTR_USBHOST_CAUSE_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_USBHOST_CAUSE_HI;
#[doc = "`read()` method returns [intr_usbhost_cause_hi::R](intr_usbhost_cause_hi::R) reader structure"]
impl crate::Readable for INTR_USBHOST_CAUSE_HI {}
#[doc = "Interrupt USB Host Cause High Register"]
pub mod intr_usbhost_cause_hi;
#[doc = "Interrupt USB Host Cause Medium Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_cause_med](intr_usbhost_cause_med) module"]
pub type INTR_USBHOST_CAUSE_MED = crate::Reg<u32, _INTR_USBHOST_CAUSE_MED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_USBHOST_CAUSE_MED;
#[doc = "`read()` method returns [intr_usbhost_cause_med::R](intr_usbhost_cause_med::R) reader structure"]
impl crate::Readable for INTR_USBHOST_CAUSE_MED {}
#[doc = "Interrupt USB Host Cause Medium Register"]
pub mod intr_usbhost_cause_med;
#[doc = "Interrupt USB Host Cause Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_cause_lo](intr_usbhost_cause_lo) module"]
pub type INTR_USBHOST_CAUSE_LO = crate::Reg<u32, _INTR_USBHOST_CAUSE_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_USBHOST_CAUSE_LO;
#[doc = "`read()` method returns [intr_usbhost_cause_lo::R](intr_usbhost_cause_lo::R) reader structure"]
impl crate::Readable for INTR_USBHOST_CAUSE_LO {}
#[doc = "Interrupt USB Host Cause Low Register"]
pub mod intr_usbhost_cause_lo;
#[doc = "Interrupt USB Host Endpoint Cause High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_cause_hi](intr_host_ep_cause_hi) module"]
pub type INTR_HOST_EP_CAUSE_HI = crate::Reg<u32, _INTR_HOST_EP_CAUSE_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_HOST_EP_CAUSE_HI;
#[doc = "`read()` method returns [intr_host_ep_cause_hi::R](intr_host_ep_cause_hi::R) reader structure"]
impl crate::Readable for INTR_HOST_EP_CAUSE_HI {}
#[doc = "Interrupt USB Host Endpoint Cause High Register"]
pub mod intr_host_ep_cause_hi;
#[doc = "Interrupt USB Host Endpoint Cause Medium Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_cause_med](intr_host_ep_cause_med) module"]
pub type INTR_HOST_EP_CAUSE_MED = crate::Reg<u32, _INTR_HOST_EP_CAUSE_MED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_HOST_EP_CAUSE_MED;
#[doc = "`read()` method returns [intr_host_ep_cause_med::R](intr_host_ep_cause_med::R) reader structure"]
impl crate::Readable for INTR_HOST_EP_CAUSE_MED {}
#[doc = "Interrupt USB Host Endpoint Cause Medium Register"]
pub mod intr_host_ep_cause_med;
#[doc = "Interrupt USB Host Endpoint Cause Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_cause_lo](intr_host_ep_cause_lo) module"]
pub type INTR_HOST_EP_CAUSE_LO = crate::Reg<u32, _INTR_HOST_EP_CAUSE_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_HOST_EP_CAUSE_LO;
#[doc = "`read()` method returns [intr_host_ep_cause_lo::R](intr_host_ep_cause_lo::R) reader structure"]
impl crate::Readable for INTR_HOST_EP_CAUSE_LO {}
#[doc = "Interrupt USB Host Endpoint Cause Low Register"]
pub mod intr_host_ep_cause_lo;
#[doc = "Interrupt USB Host Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost](intr_usbhost) module"]
pub type INTR_USBHOST = crate::Reg<u32, _INTR_USBHOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_USBHOST;
#[doc = "`read()` method returns [intr_usbhost::R](intr_usbhost::R) reader structure"]
impl crate::Readable for INTR_USBHOST {}
#[doc = "`write(|w| ..)` method takes [intr_usbhost::W](intr_usbhost::W) writer structure"]
impl crate::Writable for INTR_USBHOST {}
#[doc = "Interrupt USB Host Register"]
pub mod intr_usbhost;
#[doc = "Interrupt USB Host Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_set](intr_usbhost_set) module"]
pub type INTR_USBHOST_SET = crate::Reg<u32, _INTR_USBHOST_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_USBHOST_SET;
#[doc = "`read()` method returns [intr_usbhost_set::R](intr_usbhost_set::R) reader structure"]
impl crate::Readable for INTR_USBHOST_SET {}
#[doc = "`write(|w| ..)` method takes [intr_usbhost_set::W](intr_usbhost_set::W) writer structure"]
impl crate::Writable for INTR_USBHOST_SET {}
#[doc = "Interrupt USB Host Set Register"]
pub mod intr_usbhost_set;
#[doc = "Interrupt USB Host Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_mask](intr_usbhost_mask) module"]
pub type INTR_USBHOST_MASK = crate::Reg<u32, _INTR_USBHOST_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_USBHOST_MASK;
#[doc = "`read()` method returns [intr_usbhost_mask::R](intr_usbhost_mask::R) reader structure"]
impl crate::Readable for INTR_USBHOST_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_usbhost_mask::W](intr_usbhost_mask::W) writer structure"]
impl crate::Writable for INTR_USBHOST_MASK {}
#[doc = "Interrupt USB Host Mask Register"]
pub mod intr_usbhost_mask;
#[doc = "Interrupt USB Host Masked Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_masked](intr_usbhost_masked) module"]
pub type INTR_USBHOST_MASKED = crate::Reg<u32, _INTR_USBHOST_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_USBHOST_MASKED;
#[doc = "`read()` method returns [intr_usbhost_masked::R](intr_usbhost_masked::R) reader structure"]
impl crate::Readable for INTR_USBHOST_MASKED {}
#[doc = "Interrupt USB Host Masked Register"]
pub mod intr_usbhost_masked;
#[doc = "Interrupt USB Host Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep](intr_host_ep) module"]
pub type INTR_HOST_EP = crate::Reg<u32, _INTR_HOST_EP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_HOST_EP;
#[doc = "`read()` method returns [intr_host_ep::R](intr_host_ep::R) reader structure"]
impl crate::Readable for INTR_HOST_EP {}
#[doc = "`write(|w| ..)` method takes [intr_host_ep::W](intr_host_ep::W) writer structure"]
impl crate::Writable for INTR_HOST_EP {}
#[doc = "Interrupt USB Host Endpoint Register"]
pub mod intr_host_ep;
#[doc = "Interrupt USB Host Endpoint Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_set](intr_host_ep_set) module"]
pub type INTR_HOST_EP_SET = crate::Reg<u32, _INTR_HOST_EP_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_HOST_EP_SET;
#[doc = "`read()` method returns [intr_host_ep_set::R](intr_host_ep_set::R) reader structure"]
impl crate::Readable for INTR_HOST_EP_SET {}
#[doc = "`write(|w| ..)` method takes [intr_host_ep_set::W](intr_host_ep_set::W) writer structure"]
impl crate::Writable for INTR_HOST_EP_SET {}
#[doc = "Interrupt USB Host Endpoint Set Register"]
pub mod intr_host_ep_set;
#[doc = "Interrupt USB Host Endpoint Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_mask](intr_host_ep_mask) module"]
pub type INTR_HOST_EP_MASK = crate::Reg<u32, _INTR_HOST_EP_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_HOST_EP_MASK;
#[doc = "`read()` method returns [intr_host_ep_mask::R](intr_host_ep_mask::R) reader structure"]
impl crate::Readable for INTR_HOST_EP_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_host_ep_mask::W](intr_host_ep_mask::W) writer structure"]
impl crate::Writable for INTR_HOST_EP_MASK {}
#[doc = "Interrupt USB Host Endpoint Mask Register"]
pub mod intr_host_ep_mask;
#[doc = "Interrupt USB Host Endpoint Masked Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_masked](intr_host_ep_masked) module"]
pub type INTR_HOST_EP_MASKED = crate::Reg<u32, _INTR_HOST_EP_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_HOST_EP_MASKED;
#[doc = "`read()` method returns [intr_host_ep_masked::R](intr_host_ep_masked::R) reader structure"]
impl crate::Readable for INTR_HOST_EP_MASKED {}
#[doc = "Interrupt USB Host Endpoint Masked Register"]
pub mod intr_host_ep_masked;
#[doc = "Host DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_dma_enbl](host_dma_enbl) module"]
pub type HOST_DMA_ENBL = crate::Reg<u32, _HOST_DMA_ENBL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_DMA_ENBL;
#[doc = "`read()` method returns [host_dma_enbl::R](host_dma_enbl::R) reader structure"]
impl crate::Readable for HOST_DMA_ENBL {}
#[doc = "`write(|w| ..)` method takes [host_dma_enbl::W](host_dma_enbl::W) writer structure"]
impl crate::Writable for HOST_DMA_ENBL {}
#[doc = "Host DMA Enable Register"]
pub mod host_dma_enbl;
#[doc = "Host Endpoint 1 Block Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep1_blk](host_ep1_blk) module"]
pub type HOST_EP1_BLK = crate::Reg<u32, _HOST_EP1_BLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP1_BLK;
#[doc = "`read()` method returns [host_ep1_blk::R](host_ep1_blk::R) reader structure"]
impl crate::Readable for HOST_EP1_BLK {}
#[doc = "`write(|w| ..)` method takes [host_ep1_blk::W](host_ep1_blk::W) writer structure"]
impl crate::Writable for HOST_EP1_BLK {}
#[doc = "Host Endpoint 1 Block Register"]
pub mod host_ep1_blk;
#[doc = "Host Endpoint 2 Block Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep2_blk](host_ep2_blk) module"]
pub type HOST_EP2_BLK = crate::Reg<u32, _HOST_EP2_BLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_EP2_BLK;
#[doc = "`read()` method returns [host_ep2_blk::R](host_ep2_blk::R) reader structure"]
impl crate::Readable for HOST_EP2_BLK {}
#[doc = "`write(|w| ..)` method takes [host_ep2_blk::W](host_ep2_blk::W) writer structure"]
impl crate::Writable for HOST_EP2_BLK {}
#[doc = "Host Endpoint 2 Block Register"]
pub mod host_ep2_blk;

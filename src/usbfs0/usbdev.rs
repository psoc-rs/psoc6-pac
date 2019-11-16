#[doc = "Control End point EP0 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_dr](ep0_dr) module"]
pub type EP0_DR = crate::Reg<u32, _EP0_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0_DR;
#[doc = "`read()` method returns [ep0_dr::R](ep0_dr::R) reader structure"]
impl crate::Readable for EP0_DR {}
#[doc = "`write(|w| ..)` method takes [ep0_dr::W](ep0_dr::W) writer structure"]
impl crate::Writable for EP0_DR {}
#[doc = "Control End point EP0 Data Register"]
pub mod ep0_dr;
#[doc = "USB control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](cr0) module"]
pub type CR0 = crate::Reg<u32, _CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR0;
#[doc = "`read()` method returns [cr0::R](cr0::R) reader structure"]
impl crate::Readable for CR0 {}
#[doc = "`write(|w| ..)` method takes [cr0::W](cr0::W) writer structure"]
impl crate::Writable for CR0 {}
#[doc = "USB control 0 Register"]
pub mod cr0;
#[doc = "USB control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "USB control 1 Register"]
pub mod cr1;
#[doc = "USB SIE Data Endpoints Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep_int_en](sie_ep_int_en) module"]
pub type SIE_EP_INT_EN = crate::Reg<u32, _SIE_EP_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP_INT_EN;
#[doc = "`read()` method returns [sie_ep_int_en::R](sie_ep_int_en::R) reader structure"]
impl crate::Readable for SIE_EP_INT_EN {}
#[doc = "`write(|w| ..)` method takes [sie_ep_int_en::W](sie_ep_int_en::W) writer structure"]
impl crate::Writable for SIE_EP_INT_EN {}
#[doc = "USB SIE Data Endpoints Interrupt Enable Register"]
pub mod sie_ep_int_en;
#[doc = "USB SIE Data Endpoint Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep_int_sr](sie_ep_int_sr) module"]
pub type SIE_EP_INT_SR = crate::Reg<u32, _SIE_EP_INT_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP_INT_SR;
#[doc = "`read()` method returns [sie_ep_int_sr::R](sie_ep_int_sr::R) reader structure"]
impl crate::Readable for SIE_EP_INT_SR {}
#[doc = "`write(|w| ..)` method takes [sie_ep_int_sr::W](sie_ep_int_sr::W) writer structure"]
impl crate::Writable for SIE_EP_INT_SR {}
#[doc = "USB SIE Data Endpoint Interrupt Status"]
pub mod sie_ep_int_sr;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep1_cnt0](sie_ep1_cnt0) module"]
pub type SIE_EP1_CNT0 = crate::Reg<u32, _SIE_EP1_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP1_CNT0;
#[doc = "`read()` method returns [sie_ep1_cnt0::R](sie_ep1_cnt0::R) reader structure"]
impl crate::Readable for SIE_EP1_CNT0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep1_cnt0::W](sie_ep1_cnt0::W) writer structure"]
impl crate::Writable for SIE_EP1_CNT0 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep1_cnt1](sie_ep1_cnt1) module"]
pub type SIE_EP1_CNT1 = crate::Reg<u32, _SIE_EP1_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP1_CNT1;
#[doc = "`read()` method returns [sie_ep1_cnt1::R](sie_ep1_cnt1::R) reader structure"]
impl crate::Readable for SIE_EP1_CNT1 {}
#[doc = "`write(|w| ..)` method takes [sie_ep1_cnt1::W](sie_ep1_cnt1::W) writer structure"]
impl crate::Writable for SIE_EP1_CNT1 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt1;
#[doc = "Non-control endpoint's control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep1_cr0](sie_ep1_cr0) module"]
pub type SIE_EP1_CR0 = crate::Reg<u32, _SIE_EP1_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP1_CR0;
#[doc = "`read()` method returns [sie_ep1_cr0::R](sie_ep1_cr0::R) reader structure"]
impl crate::Readable for SIE_EP1_CR0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep1_cr0::W](sie_ep1_cr0::W) writer structure"]
impl crate::Writable for SIE_EP1_CR0 {}
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep1_cr0;
#[doc = "USBIO Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbio_cr0](usbio_cr0) module"]
pub type USBIO_CR0 = crate::Reg<u32, _USBIO_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIO_CR0;
#[doc = "`read()` method returns [usbio_cr0::R](usbio_cr0::R) reader structure"]
impl crate::Readable for USBIO_CR0 {}
#[doc = "`write(|w| ..)` method takes [usbio_cr0::W](usbio_cr0::W) writer structure"]
impl crate::Writable for USBIO_CR0 {}
#[doc = "USBIO Control 0 Register"]
pub mod usbio_cr0;
#[doc = "USBIO control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbio_cr2](usbio_cr2) module"]
pub type USBIO_CR2 = crate::Reg<u32, _USBIO_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIO_CR2;
#[doc = "`read()` method returns [usbio_cr2::R](usbio_cr2::R) reader structure"]
impl crate::Readable for USBIO_CR2 {}
#[doc = "`write(|w| ..)` method takes [usbio_cr2::W](usbio_cr2::W) writer structure"]
impl crate::Writable for USBIO_CR2 {}
#[doc = "USBIO control 2 Register"]
pub mod usbio_cr2;
#[doc = "USBIO control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbio_cr1](usbio_cr1) module"]
pub type USBIO_CR1 = crate::Reg<u32, _USBIO_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIO_CR1;
#[doc = "`read()` method returns [usbio_cr1::R](usbio_cr1::R) reader structure"]
impl crate::Readable for USBIO_CR1 {}
#[doc = "`write(|w| ..)` method takes [usbio_cr1::W](usbio_cr1::W) writer structure"]
impl crate::Writable for USBIO_CR1 {}
#[doc = "USBIO control 1 Register"]
pub mod usbio_cr1;
#[doc = "USB Dynamic reconfiguration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dyn_reconfig](dyn_reconfig) module"]
pub type DYN_RECONFIG = crate::Reg<u32, _DYN_RECONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYN_RECONFIG;
#[doc = "`read()` method returns [dyn_reconfig::R](dyn_reconfig::R) reader structure"]
impl crate::Readable for DYN_RECONFIG {}
#[doc = "`write(|w| ..)` method takes [dyn_reconfig::W](dyn_reconfig::W) writer structure"]
impl crate::Writable for DYN_RECONFIG {}
#[doc = "USB Dynamic reconfiguration register"]
pub mod dyn_reconfig;
#[doc = "Start Of Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof0](sof0) module"]
pub type SOF0 = crate::Reg<u32, _SOF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOF0;
#[doc = "`read()` method returns [sof0::R](sof0::R) reader structure"]
impl crate::Readable for SOF0 {}
#[doc = "Start Of Frame Register"]
pub mod sof0;
#[doc = "Start Of Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof1](sof1) module"]
pub type SOF1 = crate::Reg<u32, _SOF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOF1;
#[doc = "`read()` method returns [sof1::R](sof1::R) reader structure"]
impl crate::Readable for SOF1 {}
#[doc = "Start Of Frame Register"]
pub mod sof1;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep2_cnt0](sie_ep2_cnt0) module"]
pub type SIE_EP2_CNT0 = crate::Reg<u32, _SIE_EP2_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP2_CNT0;
#[doc = "`read()` method returns [sie_ep2_cnt0::R](sie_ep2_cnt0::R) reader structure"]
impl crate::Readable for SIE_EP2_CNT0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep2_cnt0::W](sie_ep2_cnt0::W) writer structure"]
impl crate::Writable for SIE_EP2_CNT0 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep2_cnt1](sie_ep2_cnt1) module"]
pub type SIE_EP2_CNT1 = crate::Reg<u32, _SIE_EP2_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP2_CNT1;
#[doc = "`read()` method returns [sie_ep2_cnt1::R](sie_ep2_cnt1::R) reader structure"]
impl crate::Readable for SIE_EP2_CNT1 {}
#[doc = "`write(|w| ..)` method takes [sie_ep2_cnt1::W](sie_ep2_cnt1::W) writer structure"]
impl crate::Writable for SIE_EP2_CNT1 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt1;
#[doc = "Non-control endpoint's control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep2_cr0](sie_ep2_cr0) module"]
pub type SIE_EP2_CR0 = crate::Reg<u32, _SIE_EP2_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP2_CR0;
#[doc = "`read()` method returns [sie_ep2_cr0::R](sie_ep2_cr0::R) reader structure"]
impl crate::Readable for SIE_EP2_CR0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep2_cr0::W](sie_ep2_cr0::W) writer structure"]
impl crate::Writable for SIE_EP2_CR0 {}
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep2_cr0;
#[doc = "Oscillator lock data register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osclk_dr0](osclk_dr0) module"]
pub type OSCLK_DR0 = crate::Reg<u32, _OSCLK_DR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCLK_DR0;
#[doc = "`read()` method returns [osclk_dr0::R](osclk_dr0::R) reader structure"]
impl crate::Readable for OSCLK_DR0 {}
#[doc = "Oscillator lock data register 0"]
pub mod osclk_dr0;
#[doc = "Oscillator lock data register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osclk_dr1](osclk_dr1) module"]
pub type OSCLK_DR1 = crate::Reg<u32, _OSCLK_DR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCLK_DR1;
#[doc = "`read()` method returns [osclk_dr1::R](osclk_dr1::R) reader structure"]
impl crate::Readable for OSCLK_DR1 {}
#[doc = "Oscillator lock data register 1"]
pub mod osclk_dr1;
#[doc = "Endpoint0 control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_cr](ep0_cr) module"]
pub type EP0_CR = crate::Reg<u32, _EP0_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0_CR;
#[doc = "`read()` method returns [ep0_cr::R](ep0_cr::R) reader structure"]
impl crate::Readable for EP0_CR {}
#[doc = "`write(|w| ..)` method takes [ep0_cr::W](ep0_cr::W) writer structure"]
impl crate::Writable for EP0_CR {}
#[doc = "Endpoint0 control Register"]
pub mod ep0_cr;
#[doc = "Endpoint0 count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_cnt](ep0_cnt) module"]
pub type EP0_CNT = crate::Reg<u32, _EP0_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0_CNT;
#[doc = "`read()` method returns [ep0_cnt::R](ep0_cnt::R) reader structure"]
impl crate::Readable for EP0_CNT {}
#[doc = "`write(|w| ..)` method takes [ep0_cnt::W](ep0_cnt::W) writer structure"]
impl crate::Writable for EP0_CNT {}
#[doc = "Endpoint0 count Register"]
pub mod ep0_cnt;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep3_cnt0](sie_ep3_cnt0) module"]
pub type SIE_EP3_CNT0 = crate::Reg<u32, _SIE_EP3_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP3_CNT0;
#[doc = "`read()` method returns [sie_ep3_cnt0::R](sie_ep3_cnt0::R) reader structure"]
impl crate::Readable for SIE_EP3_CNT0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep3_cnt0::W](sie_ep3_cnt0::W) writer structure"]
impl crate::Writable for SIE_EP3_CNT0 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep3_cnt1](sie_ep3_cnt1) module"]
pub type SIE_EP3_CNT1 = crate::Reg<u32, _SIE_EP3_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP3_CNT1;
#[doc = "`read()` method returns [sie_ep3_cnt1::R](sie_ep3_cnt1::R) reader structure"]
impl crate::Readable for SIE_EP3_CNT1 {}
#[doc = "`write(|w| ..)` method takes [sie_ep3_cnt1::W](sie_ep3_cnt1::W) writer structure"]
impl crate::Writable for SIE_EP3_CNT1 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt1;
#[doc = "Non-control endpoint's control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep3_cr0](sie_ep3_cr0) module"]
pub type SIE_EP3_CR0 = crate::Reg<u32, _SIE_EP3_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP3_CR0;
#[doc = "`read()` method returns [sie_ep3_cr0::R](sie_ep3_cr0::R) reader structure"]
impl crate::Readable for SIE_EP3_CR0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep3_cr0::W](sie_ep3_cr0::W) writer structure"]
impl crate::Writable for SIE_EP3_CR0 {}
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep3_cr0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep4_cnt0](sie_ep4_cnt0) module"]
pub type SIE_EP4_CNT0 = crate::Reg<u32, _SIE_EP4_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP4_CNT0;
#[doc = "`read()` method returns [sie_ep4_cnt0::R](sie_ep4_cnt0::R) reader structure"]
impl crate::Readable for SIE_EP4_CNT0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep4_cnt0::W](sie_ep4_cnt0::W) writer structure"]
impl crate::Writable for SIE_EP4_CNT0 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep4_cnt1](sie_ep4_cnt1) module"]
pub type SIE_EP4_CNT1 = crate::Reg<u32, _SIE_EP4_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP4_CNT1;
#[doc = "`read()` method returns [sie_ep4_cnt1::R](sie_ep4_cnt1::R) reader structure"]
impl crate::Readable for SIE_EP4_CNT1 {}
#[doc = "`write(|w| ..)` method takes [sie_ep4_cnt1::W](sie_ep4_cnt1::W) writer structure"]
impl crate::Writable for SIE_EP4_CNT1 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt1;
#[doc = "Non-control endpoint's control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep4_cr0](sie_ep4_cr0) module"]
pub type SIE_EP4_CR0 = crate::Reg<u32, _SIE_EP4_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP4_CR0;
#[doc = "`read()` method returns [sie_ep4_cr0::R](sie_ep4_cr0::R) reader structure"]
impl crate::Readable for SIE_EP4_CR0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep4_cr0::W](sie_ep4_cr0::W) writer structure"]
impl crate::Writable for SIE_EP4_CR0 {}
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep4_cr0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep5_cnt0](sie_ep5_cnt0) module"]
pub type SIE_EP5_CNT0 = crate::Reg<u32, _SIE_EP5_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP5_CNT0;
#[doc = "`read()` method returns [sie_ep5_cnt0::R](sie_ep5_cnt0::R) reader structure"]
impl crate::Readable for SIE_EP5_CNT0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep5_cnt0::W](sie_ep5_cnt0::W) writer structure"]
impl crate::Writable for SIE_EP5_CNT0 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep5_cnt1](sie_ep5_cnt1) module"]
pub type SIE_EP5_CNT1 = crate::Reg<u32, _SIE_EP5_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP5_CNT1;
#[doc = "`read()` method returns [sie_ep5_cnt1::R](sie_ep5_cnt1::R) reader structure"]
impl crate::Readable for SIE_EP5_CNT1 {}
#[doc = "`write(|w| ..)` method takes [sie_ep5_cnt1::W](sie_ep5_cnt1::W) writer structure"]
impl crate::Writable for SIE_EP5_CNT1 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt1;
#[doc = "Non-control endpoint's control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep5_cr0](sie_ep5_cr0) module"]
pub type SIE_EP5_CR0 = crate::Reg<u32, _SIE_EP5_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP5_CR0;
#[doc = "`read()` method returns [sie_ep5_cr0::R](sie_ep5_cr0::R) reader structure"]
impl crate::Readable for SIE_EP5_CR0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep5_cr0::W](sie_ep5_cr0::W) writer structure"]
impl crate::Writable for SIE_EP5_CR0 {}
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep5_cr0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep6_cnt0](sie_ep6_cnt0) module"]
pub type SIE_EP6_CNT0 = crate::Reg<u32, _SIE_EP6_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP6_CNT0;
#[doc = "`read()` method returns [sie_ep6_cnt0::R](sie_ep6_cnt0::R) reader structure"]
impl crate::Readable for SIE_EP6_CNT0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep6_cnt0::W](sie_ep6_cnt0::W) writer structure"]
impl crate::Writable for SIE_EP6_CNT0 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep6_cnt1](sie_ep6_cnt1) module"]
pub type SIE_EP6_CNT1 = crate::Reg<u32, _SIE_EP6_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP6_CNT1;
#[doc = "`read()` method returns [sie_ep6_cnt1::R](sie_ep6_cnt1::R) reader structure"]
impl crate::Readable for SIE_EP6_CNT1 {}
#[doc = "`write(|w| ..)` method takes [sie_ep6_cnt1::W](sie_ep6_cnt1::W) writer structure"]
impl crate::Writable for SIE_EP6_CNT1 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt1;
#[doc = "Non-control endpoint's control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep6_cr0](sie_ep6_cr0) module"]
pub type SIE_EP6_CR0 = crate::Reg<u32, _SIE_EP6_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP6_CR0;
#[doc = "`read()` method returns [sie_ep6_cr0::R](sie_ep6_cr0::R) reader structure"]
impl crate::Readable for SIE_EP6_CR0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep6_cr0::W](sie_ep6_cr0::W) writer structure"]
impl crate::Writable for SIE_EP6_CR0 {}
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep6_cr0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep7_cnt0](sie_ep7_cnt0) module"]
pub type SIE_EP7_CNT0 = crate::Reg<u32, _SIE_EP7_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP7_CNT0;
#[doc = "`read()` method returns [sie_ep7_cnt0::R](sie_ep7_cnt0::R) reader structure"]
impl crate::Readable for SIE_EP7_CNT0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep7_cnt0::W](sie_ep7_cnt0::W) writer structure"]
impl crate::Writable for SIE_EP7_CNT0 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep7_cnt1](sie_ep7_cnt1) module"]
pub type SIE_EP7_CNT1 = crate::Reg<u32, _SIE_EP7_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP7_CNT1;
#[doc = "`read()` method returns [sie_ep7_cnt1::R](sie_ep7_cnt1::R) reader structure"]
impl crate::Readable for SIE_EP7_CNT1 {}
#[doc = "`write(|w| ..)` method takes [sie_ep7_cnt1::W](sie_ep7_cnt1::W) writer structure"]
impl crate::Writable for SIE_EP7_CNT1 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt1;
#[doc = "Non-control endpoint's control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep7_cr0](sie_ep7_cr0) module"]
pub type SIE_EP7_CR0 = crate::Reg<u32, _SIE_EP7_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP7_CR0;
#[doc = "`read()` method returns [sie_ep7_cr0::R](sie_ep7_cr0::R) reader structure"]
impl crate::Readable for SIE_EP7_CR0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep7_cr0::W](sie_ep7_cr0::W) writer structure"]
impl crate::Writable for SIE_EP7_CR0 {}
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep7_cr0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep8_cnt0](sie_ep8_cnt0) module"]
pub type SIE_EP8_CNT0 = crate::Reg<u32, _SIE_EP8_CNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP8_CNT0;
#[doc = "`read()` method returns [sie_ep8_cnt0::R](sie_ep8_cnt0::R) reader structure"]
impl crate::Readable for SIE_EP8_CNT0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep8_cnt0::W](sie_ep8_cnt0::W) writer structure"]
impl crate::Writable for SIE_EP8_CNT0 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt0;
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep8_cnt1](sie_ep8_cnt1) module"]
pub type SIE_EP8_CNT1 = crate::Reg<u32, _SIE_EP8_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP8_CNT1;
#[doc = "`read()` method returns [sie_ep8_cnt1::R](sie_ep8_cnt1::R) reader structure"]
impl crate::Readable for SIE_EP8_CNT1 {}
#[doc = "`write(|w| ..)` method takes [sie_ep8_cnt1::W](sie_ep8_cnt1::W) writer structure"]
impl crate::Writable for SIE_EP8_CNT1 {}
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt1;
#[doc = "Non-control endpoint's control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep8_cr0](sie_ep8_cr0) module"]
pub type SIE_EP8_CR0 = crate::Reg<u32, _SIE_EP8_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE_EP8_CR0;
#[doc = "`read()` method returns [sie_ep8_cr0::R](sie_ep8_cr0::R) reader structure"]
impl crate::Readable for SIE_EP8_CR0 {}
#[doc = "`write(|w| ..)` method takes [sie_ep8_cr0::W](sie_ep8_cr0::W) writer structure"]
impl crate::Writable for SIE_EP8_CR0 {}
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep8_cr0;
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep1_cfg](arb_ep1_cfg) module"]
pub type ARB_EP1_CFG = crate::Reg<u32, _ARB_EP1_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP1_CFG;
#[doc = "`read()` method returns [arb_ep1_cfg::R](arb_ep1_cfg::R) reader structure"]
impl crate::Readable for ARB_EP1_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_ep1_cfg::W](arb_ep1_cfg::W) writer structure"]
impl crate::Writable for ARB_EP1_CFG {}
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep1_cfg;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep1_int_en](arb_ep1_int_en) module"]
pub type ARB_EP1_INT_EN = crate::Reg<u32, _ARB_EP1_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP1_INT_EN;
#[doc = "`read()` method returns [arb_ep1_int_en::R](arb_ep1_int_en::R) reader structure"]
impl crate::Readable for ARB_EP1_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_ep1_int_en::W](arb_ep1_int_en::W) writer structure"]
impl crate::Writable for ARB_EP1_INT_EN {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_int_en;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep1_sr](arb_ep1_sr) module"]
pub type ARB_EP1_SR = crate::Reg<u32, _ARB_EP1_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP1_SR;
#[doc = "`read()` method returns [arb_ep1_sr::R](arb_ep1_sr::R) reader structure"]
impl crate::Readable for ARB_EP1_SR {}
#[doc = "`write(|w| ..)` method takes [arb_ep1_sr::W](arb_ep1_sr::W) writer structure"]
impl crate::Writable for ARB_EP1_SR {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_sr;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_wa](arb_rw1_wa) module"]
pub type ARB_RW1_WA = crate::Reg<u32, _ARB_RW1_WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW1_WA;
#[doc = "`read()` method returns [arb_rw1_wa::R](arb_rw1_wa::R) reader structure"]
impl crate::Readable for ARB_RW1_WA {}
#[doc = "`write(|w| ..)` method takes [arb_rw1_wa::W](arb_rw1_wa::W) writer structure"]
impl crate::Writable for ARB_RW1_WA {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw1_wa;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_wa_msb](arb_rw1_wa_msb) module"]
pub type ARB_RW1_WA_MSB = crate::Reg<u32, _ARB_RW1_WA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW1_WA_MSB;
#[doc = "`read()` method returns [arb_rw1_wa_msb::R](arb_rw1_wa_msb::R) reader structure"]
impl crate::Readable for ARB_RW1_WA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw1_wa_msb::W](arb_rw1_wa_msb::W) writer structure"]
impl crate::Writable for ARB_RW1_WA_MSB {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw1_wa_msb;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_ra](arb_rw1_ra) module"]
pub type ARB_RW1_RA = crate::Reg<u32, _ARB_RW1_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW1_RA;
#[doc = "`read()` method returns [arb_rw1_ra::R](arb_rw1_ra::R) reader structure"]
impl crate::Readable for ARB_RW1_RA {}
#[doc = "`write(|w| ..)` method takes [arb_rw1_ra::W](arb_rw1_ra::W) writer structure"]
impl crate::Writable for ARB_RW1_RA {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw1_ra;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_ra_msb](arb_rw1_ra_msb) module"]
pub type ARB_RW1_RA_MSB = crate::Reg<u32, _ARB_RW1_RA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW1_RA_MSB;
#[doc = "`read()` method returns [arb_rw1_ra_msb::R](arb_rw1_ra_msb::R) reader structure"]
impl crate::Readable for ARB_RW1_RA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw1_ra_msb::W](arb_rw1_ra_msb::W) writer structure"]
impl crate::Writable for ARB_RW1_RA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw1_ra_msb;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_dr](arb_rw1_dr) module"]
pub type ARB_RW1_DR = crate::Reg<u32, _ARB_RW1_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW1_DR;
#[doc = "`read()` method returns [arb_rw1_dr::R](arb_rw1_dr::R) reader structure"]
impl crate::Readable for ARB_RW1_DR {}
#[doc = "`write(|w| ..)` method takes [arb_rw1_dr::W](arb_rw1_dr::W) writer structure"]
impl crate::Writable for ARB_RW1_DR {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr;
#[doc = "Dedicated Endpoint Buffer Size Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_size](buf_size) module"]
pub type BUF_SIZE = crate::Reg<u32, _BUF_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF_SIZE;
#[doc = "`read()` method returns [buf_size::R](buf_size::R) reader structure"]
impl crate::Readable for BUF_SIZE {}
#[doc = "`write(|w| ..)` method takes [buf_size::W](buf_size::W) writer structure"]
impl crate::Writable for BUF_SIZE {}
#[doc = "Dedicated Endpoint Buffer Size Register *1"]
pub mod buf_size;
#[doc = "Endpoint Active Indication Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_active](ep_active) module"]
pub type EP_ACTIVE = crate::Reg<u32, _EP_ACTIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP_ACTIVE;
#[doc = "`read()` method returns [ep_active::R](ep_active::R) reader structure"]
impl crate::Readable for EP_ACTIVE {}
#[doc = "`write(|w| ..)` method takes [ep_active::W](ep_active::W) writer structure"]
impl crate::Writable for EP_ACTIVE {}
#[doc = "Endpoint Active Indication Register *1"]
pub mod ep_active;
#[doc = "Endpoint Type (IN/OUT) Indication *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_type](ep_type) module"]
pub type EP_TYPE = crate::Reg<u32, _EP_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP_TYPE;
#[doc = "`read()` method returns [ep_type::R](ep_type::R) reader structure"]
impl crate::Readable for EP_TYPE {}
#[doc = "`write(|w| ..)` method takes [ep_type::W](ep_type::W) writer structure"]
impl crate::Writable for EP_TYPE {}
#[doc = "Endpoint Type (IN/OUT) Indication *1"]
pub mod ep_type;
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep2_cfg](arb_ep2_cfg) module"]
pub type ARB_EP2_CFG = crate::Reg<u32, _ARB_EP2_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP2_CFG;
#[doc = "`read()` method returns [arb_ep2_cfg::R](arb_ep2_cfg::R) reader structure"]
impl crate::Readable for ARB_EP2_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_ep2_cfg::W](arb_ep2_cfg::W) writer structure"]
impl crate::Writable for ARB_EP2_CFG {}
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep2_cfg;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep2_int_en](arb_ep2_int_en) module"]
pub type ARB_EP2_INT_EN = crate::Reg<u32, _ARB_EP2_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP2_INT_EN;
#[doc = "`read()` method returns [arb_ep2_int_en::R](arb_ep2_int_en::R) reader structure"]
impl crate::Readable for ARB_EP2_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_ep2_int_en::W](arb_ep2_int_en::W) writer structure"]
impl crate::Writable for ARB_EP2_INT_EN {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_int_en;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep2_sr](arb_ep2_sr) module"]
pub type ARB_EP2_SR = crate::Reg<u32, _ARB_EP2_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP2_SR;
#[doc = "`read()` method returns [arb_ep2_sr::R](arb_ep2_sr::R) reader structure"]
impl crate::Readable for ARB_EP2_SR {}
#[doc = "`write(|w| ..)` method takes [arb_ep2_sr::W](arb_ep2_sr::W) writer structure"]
impl crate::Writable for ARB_EP2_SR {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_sr;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw2_wa](arb_rw2_wa) module"]
pub type ARB_RW2_WA = crate::Reg<u32, _ARB_RW2_WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW2_WA;
#[doc = "`read()` method returns [arb_rw2_wa::R](arb_rw2_wa::R) reader structure"]
impl crate::Readable for ARB_RW2_WA {}
#[doc = "`write(|w| ..)` method takes [arb_rw2_wa::W](arb_rw2_wa::W) writer structure"]
impl crate::Writable for ARB_RW2_WA {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw2_wa;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw2_wa_msb](arb_rw2_wa_msb) module"]
pub type ARB_RW2_WA_MSB = crate::Reg<u32, _ARB_RW2_WA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW2_WA_MSB;
#[doc = "`read()` method returns [arb_rw2_wa_msb::R](arb_rw2_wa_msb::R) reader structure"]
impl crate::Readable for ARB_RW2_WA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw2_wa_msb::W](arb_rw2_wa_msb::W) writer structure"]
impl crate::Writable for ARB_RW2_WA_MSB {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw2_wa_msb;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw2_ra](arb_rw2_ra) module"]
pub type ARB_RW2_RA = crate::Reg<u32, _ARB_RW2_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW2_RA;
#[doc = "`read()` method returns [arb_rw2_ra::R](arb_rw2_ra::R) reader structure"]
impl crate::Readable for ARB_RW2_RA {}
#[doc = "`write(|w| ..)` method takes [arb_rw2_ra::W](arb_rw2_ra::W) writer structure"]
impl crate::Writable for ARB_RW2_RA {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw2_ra;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw2_ra_msb](arb_rw2_ra_msb) module"]
pub type ARB_RW2_RA_MSB = crate::Reg<u32, _ARB_RW2_RA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW2_RA_MSB;
#[doc = "`read()` method returns [arb_rw2_ra_msb::R](arb_rw2_ra_msb::R) reader structure"]
impl crate::Readable for ARB_RW2_RA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw2_ra_msb::W](arb_rw2_ra_msb::W) writer structure"]
impl crate::Writable for ARB_RW2_RA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw2_ra_msb;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw2_dr](arb_rw2_dr) module"]
pub type ARB_RW2_DR = crate::Reg<u32, _ARB_RW2_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW2_DR;
#[doc = "`read()` method returns [arb_rw2_dr::R](arb_rw2_dr::R) reader structure"]
impl crate::Readable for ARB_RW2_DR {}
#[doc = "`write(|w| ..)` method takes [arb_rw2_dr::W](arb_rw2_dr::W) writer structure"]
impl crate::Writable for ARB_RW2_DR {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr;
#[doc = "Arbiter Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_cfg](arb_cfg) module"]
pub type ARB_CFG = crate::Reg<u32, _ARB_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_CFG;
#[doc = "`read()` method returns [arb_cfg::R](arb_cfg::R) reader structure"]
impl crate::Readable for ARB_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_cfg::W](arb_cfg::W) writer structure"]
impl crate::Writable for ARB_CFG {}
#[doc = "Arbiter Configuration Register *1"]
pub mod arb_cfg;
#[doc = "USB Block Clock Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_clk_en](usb_clk_en) module"]
pub type USB_CLK_EN = crate::Reg<u32, _USB_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CLK_EN;
#[doc = "`read()` method returns [usb_clk_en::R](usb_clk_en::R) reader structure"]
impl crate::Readable for USB_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [usb_clk_en::W](usb_clk_en::W) writer structure"]
impl crate::Writable for USB_CLK_EN {}
#[doc = "USB Block Clock Enable Register"]
pub mod usb_clk_en;
#[doc = "Arbiter Interrupt Enable *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_int_en](arb_int_en) module"]
pub type ARB_INT_EN = crate::Reg<u32, _ARB_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_INT_EN;
#[doc = "`read()` method returns [arb_int_en::R](arb_int_en::R) reader structure"]
impl crate::Readable for ARB_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_int_en::W](arb_int_en::W) writer structure"]
impl crate::Writable for ARB_INT_EN {}
#[doc = "Arbiter Interrupt Enable *1"]
pub mod arb_int_en;
#[doc = "Arbiter Interrupt Status *1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_int_sr](arb_int_sr) module"]
pub type ARB_INT_SR = crate::Reg<u32, _ARB_INT_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_INT_SR;
#[doc = "`read()` method returns [arb_int_sr::R](arb_int_sr::R) reader structure"]
impl crate::Readable for ARB_INT_SR {}
#[doc = "Arbiter Interrupt Status *1"]
pub mod arb_int_sr;
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep3_cfg](arb_ep3_cfg) module"]
pub type ARB_EP3_CFG = crate::Reg<u32, _ARB_EP3_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP3_CFG;
#[doc = "`read()` method returns [arb_ep3_cfg::R](arb_ep3_cfg::R) reader structure"]
impl crate::Readable for ARB_EP3_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_ep3_cfg::W](arb_ep3_cfg::W) writer structure"]
impl crate::Writable for ARB_EP3_CFG {}
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep3_cfg;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep3_int_en](arb_ep3_int_en) module"]
pub type ARB_EP3_INT_EN = crate::Reg<u32, _ARB_EP3_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP3_INT_EN;
#[doc = "`read()` method returns [arb_ep3_int_en::R](arb_ep3_int_en::R) reader structure"]
impl crate::Readable for ARB_EP3_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_ep3_int_en::W](arb_ep3_int_en::W) writer structure"]
impl crate::Writable for ARB_EP3_INT_EN {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_int_en;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep3_sr](arb_ep3_sr) module"]
pub type ARB_EP3_SR = crate::Reg<u32, _ARB_EP3_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP3_SR;
#[doc = "`read()` method returns [arb_ep3_sr::R](arb_ep3_sr::R) reader structure"]
impl crate::Readable for ARB_EP3_SR {}
#[doc = "`write(|w| ..)` method takes [arb_ep3_sr::W](arb_ep3_sr::W) writer structure"]
impl crate::Writable for ARB_EP3_SR {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_sr;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_wa](arb_rw3_wa) module"]
pub type ARB_RW3_WA = crate::Reg<u32, _ARB_RW3_WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW3_WA;
#[doc = "`read()` method returns [arb_rw3_wa::R](arb_rw3_wa::R) reader structure"]
impl crate::Readable for ARB_RW3_WA {}
#[doc = "`write(|w| ..)` method takes [arb_rw3_wa::W](arb_rw3_wa::W) writer structure"]
impl crate::Writable for ARB_RW3_WA {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw3_wa;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_wa_msb](arb_rw3_wa_msb) module"]
pub type ARB_RW3_WA_MSB = crate::Reg<u32, _ARB_RW3_WA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW3_WA_MSB;
#[doc = "`read()` method returns [arb_rw3_wa_msb::R](arb_rw3_wa_msb::R) reader structure"]
impl crate::Readable for ARB_RW3_WA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw3_wa_msb::W](arb_rw3_wa_msb::W) writer structure"]
impl crate::Writable for ARB_RW3_WA_MSB {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw3_wa_msb;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_ra](arb_rw3_ra) module"]
pub type ARB_RW3_RA = crate::Reg<u32, _ARB_RW3_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW3_RA;
#[doc = "`read()` method returns [arb_rw3_ra::R](arb_rw3_ra::R) reader structure"]
impl crate::Readable for ARB_RW3_RA {}
#[doc = "`write(|w| ..)` method takes [arb_rw3_ra::W](arb_rw3_ra::W) writer structure"]
impl crate::Writable for ARB_RW3_RA {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw3_ra;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_ra_msb](arb_rw3_ra_msb) module"]
pub type ARB_RW3_RA_MSB = crate::Reg<u32, _ARB_RW3_RA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW3_RA_MSB;
#[doc = "`read()` method returns [arb_rw3_ra_msb::R](arb_rw3_ra_msb::R) reader structure"]
impl crate::Readable for ARB_RW3_RA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw3_ra_msb::W](arb_rw3_ra_msb::W) writer structure"]
impl crate::Writable for ARB_RW3_RA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw3_ra_msb;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_dr](arb_rw3_dr) module"]
pub type ARB_RW3_DR = crate::Reg<u32, _ARB_RW3_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW3_DR;
#[doc = "`read()` method returns [arb_rw3_dr::R](arb_rw3_dr::R) reader structure"]
impl crate::Readable for ARB_RW3_DR {}
#[doc = "`write(|w| ..)` method takes [arb_rw3_dr::W](arb_rw3_dr::W) writer structure"]
impl crate::Writable for ARB_RW3_DR {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr;
#[doc = "Common Area Write Address *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwa](cwa) module"]
pub type CWA = crate::Reg<u32, _CWA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWA;
#[doc = "`read()` method returns [cwa::R](cwa::R) reader structure"]
impl crate::Readable for CWA {}
#[doc = "`write(|w| ..)` method takes [cwa::W](cwa::W) writer structure"]
impl crate::Writable for CWA {}
#[doc = "Common Area Write Address *1"]
pub mod cwa;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwa_msb](cwa_msb) module"]
pub type CWA_MSB = crate::Reg<u32, _CWA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWA_MSB;
#[doc = "`read()` method returns [cwa_msb::R](cwa_msb::R) reader structure"]
impl crate::Readable for CWA_MSB {}
#[doc = "`write(|w| ..)` method takes [cwa_msb::W](cwa_msb::W) writer structure"]
impl crate::Writable for CWA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod cwa_msb;
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep4_cfg](arb_ep4_cfg) module"]
pub type ARB_EP4_CFG = crate::Reg<u32, _ARB_EP4_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP4_CFG;
#[doc = "`read()` method returns [arb_ep4_cfg::R](arb_ep4_cfg::R) reader structure"]
impl crate::Readable for ARB_EP4_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_ep4_cfg::W](arb_ep4_cfg::W) writer structure"]
impl crate::Writable for ARB_EP4_CFG {}
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep4_cfg;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep4_int_en](arb_ep4_int_en) module"]
pub type ARB_EP4_INT_EN = crate::Reg<u32, _ARB_EP4_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP4_INT_EN;
#[doc = "`read()` method returns [arb_ep4_int_en::R](arb_ep4_int_en::R) reader structure"]
impl crate::Readable for ARB_EP4_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_ep4_int_en::W](arb_ep4_int_en::W) writer structure"]
impl crate::Writable for ARB_EP4_INT_EN {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_int_en;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep4_sr](arb_ep4_sr) module"]
pub type ARB_EP4_SR = crate::Reg<u32, _ARB_EP4_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP4_SR;
#[doc = "`read()` method returns [arb_ep4_sr::R](arb_ep4_sr::R) reader structure"]
impl crate::Readable for ARB_EP4_SR {}
#[doc = "`write(|w| ..)` method takes [arb_ep4_sr::W](arb_ep4_sr::W) writer structure"]
impl crate::Writable for ARB_EP4_SR {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_sr;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw4_wa](arb_rw4_wa) module"]
pub type ARB_RW4_WA = crate::Reg<u32, _ARB_RW4_WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW4_WA;
#[doc = "`read()` method returns [arb_rw4_wa::R](arb_rw4_wa::R) reader structure"]
impl crate::Readable for ARB_RW4_WA {}
#[doc = "`write(|w| ..)` method takes [arb_rw4_wa::W](arb_rw4_wa::W) writer structure"]
impl crate::Writable for ARB_RW4_WA {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw4_wa;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw4_wa_msb](arb_rw4_wa_msb) module"]
pub type ARB_RW4_WA_MSB = crate::Reg<u32, _ARB_RW4_WA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW4_WA_MSB;
#[doc = "`read()` method returns [arb_rw4_wa_msb::R](arb_rw4_wa_msb::R) reader structure"]
impl crate::Readable for ARB_RW4_WA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw4_wa_msb::W](arb_rw4_wa_msb::W) writer structure"]
impl crate::Writable for ARB_RW4_WA_MSB {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw4_wa_msb;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw4_ra](arb_rw4_ra) module"]
pub type ARB_RW4_RA = crate::Reg<u32, _ARB_RW4_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW4_RA;
#[doc = "`read()` method returns [arb_rw4_ra::R](arb_rw4_ra::R) reader structure"]
impl crate::Readable for ARB_RW4_RA {}
#[doc = "`write(|w| ..)` method takes [arb_rw4_ra::W](arb_rw4_ra::W) writer structure"]
impl crate::Writable for ARB_RW4_RA {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw4_ra;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw4_ra_msb](arb_rw4_ra_msb) module"]
pub type ARB_RW4_RA_MSB = crate::Reg<u32, _ARB_RW4_RA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW4_RA_MSB;
#[doc = "`read()` method returns [arb_rw4_ra_msb::R](arb_rw4_ra_msb::R) reader structure"]
impl crate::Readable for ARB_RW4_RA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw4_ra_msb::W](arb_rw4_ra_msb::W) writer structure"]
impl crate::Writable for ARB_RW4_RA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw4_ra_msb;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw4_dr](arb_rw4_dr) module"]
pub type ARB_RW4_DR = crate::Reg<u32, _ARB_RW4_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW4_DR;
#[doc = "`read()` method returns [arb_rw4_dr::R](arb_rw4_dr::R) reader structure"]
impl crate::Readable for ARB_RW4_DR {}
#[doc = "`write(|w| ..)` method takes [arb_rw4_dr::W](arb_rw4_dr::W) writer structure"]
impl crate::Writable for ARB_RW4_DR {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr;
#[doc = "DMA Burst / Threshold Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_thres](dma_thres) module"]
pub type DMA_THRES = crate::Reg<u32, _DMA_THRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_THRES;
#[doc = "`read()` method returns [dma_thres::R](dma_thres::R) reader structure"]
impl crate::Readable for DMA_THRES {}
#[doc = "`write(|w| ..)` method takes [dma_thres::W](dma_thres::W) writer structure"]
impl crate::Writable for DMA_THRES {}
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres;
#[doc = "DMA Burst / Threshold Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_thres_msb](dma_thres_msb) module"]
pub type DMA_THRES_MSB = crate::Reg<u32, _DMA_THRES_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_THRES_MSB;
#[doc = "`read()` method returns [dma_thres_msb::R](dma_thres_msb::R) reader structure"]
impl crate::Readable for DMA_THRES_MSB {}
#[doc = "`write(|w| ..)` method takes [dma_thres_msb::W](dma_thres_msb::W) writer structure"]
impl crate::Writable for DMA_THRES_MSB {}
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres_msb;
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep5_cfg](arb_ep5_cfg) module"]
pub type ARB_EP5_CFG = crate::Reg<u32, _ARB_EP5_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP5_CFG;
#[doc = "`read()` method returns [arb_ep5_cfg::R](arb_ep5_cfg::R) reader structure"]
impl crate::Readable for ARB_EP5_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_ep5_cfg::W](arb_ep5_cfg::W) writer structure"]
impl crate::Writable for ARB_EP5_CFG {}
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep5_cfg;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep5_int_en](arb_ep5_int_en) module"]
pub type ARB_EP5_INT_EN = crate::Reg<u32, _ARB_EP5_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP5_INT_EN;
#[doc = "`read()` method returns [arb_ep5_int_en::R](arb_ep5_int_en::R) reader structure"]
impl crate::Readable for ARB_EP5_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_ep5_int_en::W](arb_ep5_int_en::W) writer structure"]
impl crate::Writable for ARB_EP5_INT_EN {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_int_en;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep5_sr](arb_ep5_sr) module"]
pub type ARB_EP5_SR = crate::Reg<u32, _ARB_EP5_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP5_SR;
#[doc = "`read()` method returns [arb_ep5_sr::R](arb_ep5_sr::R) reader structure"]
impl crate::Readable for ARB_EP5_SR {}
#[doc = "`write(|w| ..)` method takes [arb_ep5_sr::W](arb_ep5_sr::W) writer structure"]
impl crate::Writable for ARB_EP5_SR {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_sr;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_wa](arb_rw5_wa) module"]
pub type ARB_RW5_WA = crate::Reg<u32, _ARB_RW5_WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW5_WA;
#[doc = "`read()` method returns [arb_rw5_wa::R](arb_rw5_wa::R) reader structure"]
impl crate::Readable for ARB_RW5_WA {}
#[doc = "`write(|w| ..)` method takes [arb_rw5_wa::W](arb_rw5_wa::W) writer structure"]
impl crate::Writable for ARB_RW5_WA {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw5_wa;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_wa_msb](arb_rw5_wa_msb) module"]
pub type ARB_RW5_WA_MSB = crate::Reg<u32, _ARB_RW5_WA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW5_WA_MSB;
#[doc = "`read()` method returns [arb_rw5_wa_msb::R](arb_rw5_wa_msb::R) reader structure"]
impl crate::Readable for ARB_RW5_WA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw5_wa_msb::W](arb_rw5_wa_msb::W) writer structure"]
impl crate::Writable for ARB_RW5_WA_MSB {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw5_wa_msb;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_ra](arb_rw5_ra) module"]
pub type ARB_RW5_RA = crate::Reg<u32, _ARB_RW5_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW5_RA;
#[doc = "`read()` method returns [arb_rw5_ra::R](arb_rw5_ra::R) reader structure"]
impl crate::Readable for ARB_RW5_RA {}
#[doc = "`write(|w| ..)` method takes [arb_rw5_ra::W](arb_rw5_ra::W) writer structure"]
impl crate::Writable for ARB_RW5_RA {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw5_ra;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_ra_msb](arb_rw5_ra_msb) module"]
pub type ARB_RW5_RA_MSB = crate::Reg<u32, _ARB_RW5_RA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW5_RA_MSB;
#[doc = "`read()` method returns [arb_rw5_ra_msb::R](arb_rw5_ra_msb::R) reader structure"]
impl crate::Readable for ARB_RW5_RA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw5_ra_msb::W](arb_rw5_ra_msb::W) writer structure"]
impl crate::Writable for ARB_RW5_RA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw5_ra_msb;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_dr](arb_rw5_dr) module"]
pub type ARB_RW5_DR = crate::Reg<u32, _ARB_RW5_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW5_DR;
#[doc = "`read()` method returns [arb_rw5_dr::R](arb_rw5_dr::R) reader structure"]
impl crate::Readable for ARB_RW5_DR {}
#[doc = "`write(|w| ..)` method takes [arb_rw5_dr::W](arb_rw5_dr::W) writer structure"]
impl crate::Writable for ARB_RW5_DR {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr;
#[doc = "Bus Reset Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_rst_cnt](bus_rst_cnt) module"]
pub type BUS_RST_CNT = crate::Reg<u32, _BUS_RST_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_RST_CNT;
#[doc = "`read()` method returns [bus_rst_cnt::R](bus_rst_cnt::R) reader structure"]
impl crate::Readable for BUS_RST_CNT {}
#[doc = "`write(|w| ..)` method takes [bus_rst_cnt::W](bus_rst_cnt::W) writer structure"]
impl crate::Writable for BUS_RST_CNT {}
#[doc = "Bus Reset Count Register"]
pub mod bus_rst_cnt;
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep6_cfg](arb_ep6_cfg) module"]
pub type ARB_EP6_CFG = crate::Reg<u32, _ARB_EP6_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP6_CFG;
#[doc = "`read()` method returns [arb_ep6_cfg::R](arb_ep6_cfg::R) reader structure"]
impl crate::Readable for ARB_EP6_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_ep6_cfg::W](arb_ep6_cfg::W) writer structure"]
impl crate::Writable for ARB_EP6_CFG {}
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep6_cfg;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep6_int_en](arb_ep6_int_en) module"]
pub type ARB_EP6_INT_EN = crate::Reg<u32, _ARB_EP6_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP6_INT_EN;
#[doc = "`read()` method returns [arb_ep6_int_en::R](arb_ep6_int_en::R) reader structure"]
impl crate::Readable for ARB_EP6_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_ep6_int_en::W](arb_ep6_int_en::W) writer structure"]
impl crate::Writable for ARB_EP6_INT_EN {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_int_en;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep6_sr](arb_ep6_sr) module"]
pub type ARB_EP6_SR = crate::Reg<u32, _ARB_EP6_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP6_SR;
#[doc = "`read()` method returns [arb_ep6_sr::R](arb_ep6_sr::R) reader structure"]
impl crate::Readable for ARB_EP6_SR {}
#[doc = "`write(|w| ..)` method takes [arb_ep6_sr::W](arb_ep6_sr::W) writer structure"]
impl crate::Writable for ARB_EP6_SR {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_sr;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_wa](arb_rw6_wa) module"]
pub type ARB_RW6_WA = crate::Reg<u32, _ARB_RW6_WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW6_WA;
#[doc = "`read()` method returns [arb_rw6_wa::R](arb_rw6_wa::R) reader structure"]
impl crate::Readable for ARB_RW6_WA {}
#[doc = "`write(|w| ..)` method takes [arb_rw6_wa::W](arb_rw6_wa::W) writer structure"]
impl crate::Writable for ARB_RW6_WA {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw6_wa;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_wa_msb](arb_rw6_wa_msb) module"]
pub type ARB_RW6_WA_MSB = crate::Reg<u32, _ARB_RW6_WA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW6_WA_MSB;
#[doc = "`read()` method returns [arb_rw6_wa_msb::R](arb_rw6_wa_msb::R) reader structure"]
impl crate::Readable for ARB_RW6_WA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw6_wa_msb::W](arb_rw6_wa_msb::W) writer structure"]
impl crate::Writable for ARB_RW6_WA_MSB {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw6_wa_msb;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_ra](arb_rw6_ra) module"]
pub type ARB_RW6_RA = crate::Reg<u32, _ARB_RW6_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW6_RA;
#[doc = "`read()` method returns [arb_rw6_ra::R](arb_rw6_ra::R) reader structure"]
impl crate::Readable for ARB_RW6_RA {}
#[doc = "`write(|w| ..)` method takes [arb_rw6_ra::W](arb_rw6_ra::W) writer structure"]
impl crate::Writable for ARB_RW6_RA {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw6_ra;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_ra_msb](arb_rw6_ra_msb) module"]
pub type ARB_RW6_RA_MSB = crate::Reg<u32, _ARB_RW6_RA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW6_RA_MSB;
#[doc = "`read()` method returns [arb_rw6_ra_msb::R](arb_rw6_ra_msb::R) reader structure"]
impl crate::Readable for ARB_RW6_RA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw6_ra_msb::W](arb_rw6_ra_msb::W) writer structure"]
impl crate::Writable for ARB_RW6_RA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw6_ra_msb;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_dr](arb_rw6_dr) module"]
pub type ARB_RW6_DR = crate::Reg<u32, _ARB_RW6_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW6_DR;
#[doc = "`read()` method returns [arb_rw6_dr::R](arb_rw6_dr::R) reader structure"]
impl crate::Readable for ARB_RW6_DR {}
#[doc = "`write(|w| ..)` method takes [arb_rw6_dr::W](arb_rw6_dr::W) writer structure"]
impl crate::Writable for ARB_RW6_DR {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr;
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep7_cfg](arb_ep7_cfg) module"]
pub type ARB_EP7_CFG = crate::Reg<u32, _ARB_EP7_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP7_CFG;
#[doc = "`read()` method returns [arb_ep7_cfg::R](arb_ep7_cfg::R) reader structure"]
impl crate::Readable for ARB_EP7_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_ep7_cfg::W](arb_ep7_cfg::W) writer structure"]
impl crate::Writable for ARB_EP7_CFG {}
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep7_cfg;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep7_int_en](arb_ep7_int_en) module"]
pub type ARB_EP7_INT_EN = crate::Reg<u32, _ARB_EP7_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP7_INT_EN;
#[doc = "`read()` method returns [arb_ep7_int_en::R](arb_ep7_int_en::R) reader structure"]
impl crate::Readable for ARB_EP7_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_ep7_int_en::W](arb_ep7_int_en::W) writer structure"]
impl crate::Writable for ARB_EP7_INT_EN {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_int_en;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep7_sr](arb_ep7_sr) module"]
pub type ARB_EP7_SR = crate::Reg<u32, _ARB_EP7_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP7_SR;
#[doc = "`read()` method returns [arb_ep7_sr::R](arb_ep7_sr::R) reader structure"]
impl crate::Readable for ARB_EP7_SR {}
#[doc = "`write(|w| ..)` method takes [arb_ep7_sr::W](arb_ep7_sr::W) writer structure"]
impl crate::Writable for ARB_EP7_SR {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_sr;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_wa](arb_rw7_wa) module"]
pub type ARB_RW7_WA = crate::Reg<u32, _ARB_RW7_WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW7_WA;
#[doc = "`read()` method returns [arb_rw7_wa::R](arb_rw7_wa::R) reader structure"]
impl crate::Readable for ARB_RW7_WA {}
#[doc = "`write(|w| ..)` method takes [arb_rw7_wa::W](arb_rw7_wa::W) writer structure"]
impl crate::Writable for ARB_RW7_WA {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw7_wa;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_wa_msb](arb_rw7_wa_msb) module"]
pub type ARB_RW7_WA_MSB = crate::Reg<u32, _ARB_RW7_WA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW7_WA_MSB;
#[doc = "`read()` method returns [arb_rw7_wa_msb::R](arb_rw7_wa_msb::R) reader structure"]
impl crate::Readable for ARB_RW7_WA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw7_wa_msb::W](arb_rw7_wa_msb::W) writer structure"]
impl crate::Writable for ARB_RW7_WA_MSB {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw7_wa_msb;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_ra](arb_rw7_ra) module"]
pub type ARB_RW7_RA = crate::Reg<u32, _ARB_RW7_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW7_RA;
#[doc = "`read()` method returns [arb_rw7_ra::R](arb_rw7_ra::R) reader structure"]
impl crate::Readable for ARB_RW7_RA {}
#[doc = "`write(|w| ..)` method takes [arb_rw7_ra::W](arb_rw7_ra::W) writer structure"]
impl crate::Writable for ARB_RW7_RA {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw7_ra;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_ra_msb](arb_rw7_ra_msb) module"]
pub type ARB_RW7_RA_MSB = crate::Reg<u32, _ARB_RW7_RA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW7_RA_MSB;
#[doc = "`read()` method returns [arb_rw7_ra_msb::R](arb_rw7_ra_msb::R) reader structure"]
impl crate::Readable for ARB_RW7_RA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw7_ra_msb::W](arb_rw7_ra_msb::W) writer structure"]
impl crate::Writable for ARB_RW7_RA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw7_ra_msb;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_dr](arb_rw7_dr) module"]
pub type ARB_RW7_DR = crate::Reg<u32, _ARB_RW7_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW7_DR;
#[doc = "`read()` method returns [arb_rw7_dr::R](arb_rw7_dr::R) reader structure"]
impl crate::Readable for ARB_RW7_DR {}
#[doc = "`write(|w| ..)` method takes [arb_rw7_dr::W](arb_rw7_dr::W) writer structure"]
impl crate::Writable for ARB_RW7_DR {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr;
#[doc = "Endpoint Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep8_cfg](arb_ep8_cfg) module"]
pub type ARB_EP8_CFG = crate::Reg<u32, _ARB_EP8_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP8_CFG;
#[doc = "`read()` method returns [arb_ep8_cfg::R](arb_ep8_cfg::R) reader structure"]
impl crate::Readable for ARB_EP8_CFG {}
#[doc = "`write(|w| ..)` method takes [arb_ep8_cfg::W](arb_ep8_cfg::W) writer structure"]
impl crate::Writable for ARB_EP8_CFG {}
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep8_cfg;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep8_int_en](arb_ep8_int_en) module"]
pub type ARB_EP8_INT_EN = crate::Reg<u32, _ARB_EP8_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP8_INT_EN;
#[doc = "`read()` method returns [arb_ep8_int_en::R](arb_ep8_int_en::R) reader structure"]
impl crate::Readable for ARB_EP8_INT_EN {}
#[doc = "`write(|w| ..)` method takes [arb_ep8_int_en::W](arb_ep8_int_en::W) writer structure"]
impl crate::Writable for ARB_EP8_INT_EN {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_int_en;
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep8_sr](arb_ep8_sr) module"]
pub type ARB_EP8_SR = crate::Reg<u32, _ARB_EP8_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_EP8_SR;
#[doc = "`read()` method returns [arb_ep8_sr::R](arb_ep8_sr::R) reader structure"]
impl crate::Readable for ARB_EP8_SR {}
#[doc = "`write(|w| ..)` method takes [arb_ep8_sr::W](arb_ep8_sr::W) writer structure"]
impl crate::Writable for ARB_EP8_SR {}
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_sr;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw8_wa](arb_rw8_wa) module"]
pub type ARB_RW8_WA = crate::Reg<u32, _ARB_RW8_WA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW8_WA;
#[doc = "`read()` method returns [arb_rw8_wa::R](arb_rw8_wa::R) reader structure"]
impl crate::Readable for ARB_RW8_WA {}
#[doc = "`write(|w| ..)` method takes [arb_rw8_wa::W](arb_rw8_wa::W) writer structure"]
impl crate::Writable for ARB_RW8_WA {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw8_wa;
#[doc = "Endpoint Write Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw8_wa_msb](arb_rw8_wa_msb) module"]
pub type ARB_RW8_WA_MSB = crate::Reg<u32, _ARB_RW8_WA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW8_WA_MSB;
#[doc = "`read()` method returns [arb_rw8_wa_msb::R](arb_rw8_wa_msb::R) reader structure"]
impl crate::Readable for ARB_RW8_WA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw8_wa_msb::W](arb_rw8_wa_msb::W) writer structure"]
impl crate::Writable for ARB_RW8_WA_MSB {}
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw8_wa_msb;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw8_ra](arb_rw8_ra) module"]
pub type ARB_RW8_RA = crate::Reg<u32, _ARB_RW8_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW8_RA;
#[doc = "`read()` method returns [arb_rw8_ra::R](arb_rw8_ra::R) reader structure"]
impl crate::Readable for ARB_RW8_RA {}
#[doc = "`write(|w| ..)` method takes [arb_rw8_ra::W](arb_rw8_ra::W) writer structure"]
impl crate::Writable for ARB_RW8_RA {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw8_ra;
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw8_ra_msb](arb_rw8_ra_msb) module"]
pub type ARB_RW8_RA_MSB = crate::Reg<u32, _ARB_RW8_RA_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW8_RA_MSB;
#[doc = "`read()` method returns [arb_rw8_ra_msb::R](arb_rw8_ra_msb::R) reader structure"]
impl crate::Readable for ARB_RW8_RA_MSB {}
#[doc = "`write(|w| ..)` method takes [arb_rw8_ra_msb::W](arb_rw8_ra_msb::W) writer structure"]
impl crate::Writable for ARB_RW8_RA_MSB {}
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw8_ra_msb;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw8_dr](arb_rw8_dr) module"]
pub type ARB_RW8_DR = crate::Reg<u32, _ARB_RW8_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW8_DR;
#[doc = "`read()` method returns [arb_rw8_dr::R](arb_rw8_dr::R) reader structure"]
impl crate::Readable for ARB_RW8_DR {}
#[doc = "`write(|w| ..)` method takes [arb_rw8_dr::W](arb_rw8_dr::W) writer structure"]
impl crate::Writable for ARB_RW8_DR {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr;
#[doc = "DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_data](mem_data) module"]
pub type MEM_DATA = crate::Reg<u32, _MEM_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_DATA;
#[doc = "`read()` method returns [mem_data::R](mem_data::R) reader structure"]
impl crate::Readable for MEM_DATA {}
#[doc = "`write(|w| ..)` method takes [mem_data::W](mem_data::W) writer structure"]
impl crate::Writable for MEM_DATA {}
#[doc = "DATA"]
pub mod mem_data;
#[doc = "Start Of Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof16](sof16) module"]
pub type SOF16 = crate::Reg<u32, _SOF16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOF16;
#[doc = "`read()` method returns [sof16::R](sof16::R) reader structure"]
impl crate::Readable for SOF16 {}
#[doc = "Start Of Frame Register"]
pub mod sof16;
#[doc = "Oscillator lock data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osclk_dr16](osclk_dr16) module"]
pub type OSCLK_DR16 = crate::Reg<u32, _OSCLK_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCLK_DR16;
#[doc = "`read()` method returns [osclk_dr16::R](osclk_dr16::R) reader structure"]
impl crate::Readable for OSCLK_DR16 {}
#[doc = "Oscillator lock data register"]
pub mod osclk_dr16;
#[doc = "Endpoint Write Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_wa16](arb_rw1_wa16) module"]
pub type ARB_RW1_WA16 = crate::Reg<u32, _ARB_RW1_WA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW1_WA16;
#[doc = "`read()` method returns [arb_rw1_wa16::R](arb_rw1_wa16::R) reader structure"]
impl crate::Readable for ARB_RW1_WA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw1_wa16::W](arb_rw1_wa16::W) writer structure"]
impl crate::Writable for ARB_RW1_WA16 {}
#[doc = "Endpoint Write Address value"]
pub mod arb_rw1_wa16;
#[doc = "Endpoint Read Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_ra16](arb_rw1_ra16) module"]
pub type ARB_RW1_RA16 = crate::Reg<u32, _ARB_RW1_RA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW1_RA16;
#[doc = "`read()` method returns [arb_rw1_ra16::R](arb_rw1_ra16::R) reader structure"]
impl crate::Readable for ARB_RW1_RA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw1_ra16::W](arb_rw1_ra16::W) writer structure"]
impl crate::Writable for ARB_RW1_RA16 {}
#[doc = "Endpoint Read Address value"]
pub mod arb_rw1_ra16;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_dr16](arb_rw1_dr16) module"]
pub type ARB_RW1_DR16 = crate::Reg<u32, _ARB_RW1_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW1_DR16;
#[doc = "`read()` method returns [arb_rw1_dr16::R](arb_rw1_dr16::R) reader structure"]
impl crate::Readable for ARB_RW1_DR16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw1_dr16::W](arb_rw1_dr16::W) writer structure"]
impl crate::Writable for ARB_RW1_DR16 {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr16;
#[doc = "Endpoint Write Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw2_wa16](arb_rw2_wa16) module"]
pub type ARB_RW2_WA16 = crate::Reg<u32, _ARB_RW2_WA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW2_WA16;
#[doc = "`read()` method returns [arb_rw2_wa16::R](arb_rw2_wa16::R) reader structure"]
impl crate::Readable for ARB_RW2_WA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw2_wa16::W](arb_rw2_wa16::W) writer structure"]
impl crate::Writable for ARB_RW2_WA16 {}
#[doc = "Endpoint Write Address value"]
pub mod arb_rw2_wa16;
#[doc = "Endpoint Read Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw2_ra16](arb_rw2_ra16) module"]
pub type ARB_RW2_RA16 = crate::Reg<u32, _ARB_RW2_RA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW2_RA16;
#[doc = "`read()` method returns [arb_rw2_ra16::R](arb_rw2_ra16::R) reader structure"]
impl crate::Readable for ARB_RW2_RA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw2_ra16::W](arb_rw2_ra16::W) writer structure"]
impl crate::Writable for ARB_RW2_RA16 {}
#[doc = "Endpoint Read Address value"]
pub mod arb_rw2_ra16;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw2_dr16](arb_rw2_dr16) module"]
pub type ARB_RW2_DR16 = crate::Reg<u32, _ARB_RW2_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW2_DR16;
#[doc = "`read()` method returns [arb_rw2_dr16::R](arb_rw2_dr16::R) reader structure"]
impl crate::Readable for ARB_RW2_DR16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw2_dr16::W](arb_rw2_dr16::W) writer structure"]
impl crate::Writable for ARB_RW2_DR16 {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr16;
#[doc = "Endpoint Write Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_wa16](arb_rw3_wa16) module"]
pub type ARB_RW3_WA16 = crate::Reg<u32, _ARB_RW3_WA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW3_WA16;
#[doc = "`read()` method returns [arb_rw3_wa16::R](arb_rw3_wa16::R) reader structure"]
impl crate::Readable for ARB_RW3_WA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw3_wa16::W](arb_rw3_wa16::W) writer structure"]
impl crate::Writable for ARB_RW3_WA16 {}
#[doc = "Endpoint Write Address value"]
pub mod arb_rw3_wa16;
#[doc = "Endpoint Read Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_ra16](arb_rw3_ra16) module"]
pub type ARB_RW3_RA16 = crate::Reg<u32, _ARB_RW3_RA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW3_RA16;
#[doc = "`read()` method returns [arb_rw3_ra16::R](arb_rw3_ra16::R) reader structure"]
impl crate::Readable for ARB_RW3_RA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw3_ra16::W](arb_rw3_ra16::W) writer structure"]
impl crate::Writable for ARB_RW3_RA16 {}
#[doc = "Endpoint Read Address value"]
pub mod arb_rw3_ra16;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_dr16](arb_rw3_dr16) module"]
pub type ARB_RW3_DR16 = crate::Reg<u32, _ARB_RW3_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW3_DR16;
#[doc = "`read()` method returns [arb_rw3_dr16::R](arb_rw3_dr16::R) reader structure"]
impl crate::Readable for ARB_RW3_DR16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw3_dr16::W](arb_rw3_dr16::W) writer structure"]
impl crate::Writable for ARB_RW3_DR16 {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr16;
#[doc = "Common Area Write Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwa16](cwa16) module"]
pub type CWA16 = crate::Reg<u32, _CWA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWA16;
#[doc = "`read()` method returns [cwa16::R](cwa16::R) reader structure"]
impl crate::Readable for CWA16 {}
#[doc = "`write(|w| ..)` method takes [cwa16::W](cwa16::W) writer structure"]
impl crate::Writable for CWA16 {}
#[doc = "Common Area Write Address"]
pub mod cwa16;
#[doc = "Endpoint Write Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw4_wa16](arb_rw4_wa16) module"]
pub type ARB_RW4_WA16 = crate::Reg<u32, _ARB_RW4_WA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW4_WA16;
#[doc = "`read()` method returns [arb_rw4_wa16::R](arb_rw4_wa16::R) reader structure"]
impl crate::Readable for ARB_RW4_WA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw4_wa16::W](arb_rw4_wa16::W) writer structure"]
impl crate::Writable for ARB_RW4_WA16 {}
#[doc = "Endpoint Write Address value"]
pub mod arb_rw4_wa16;
#[doc = "Endpoint Read Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw4_ra16](arb_rw4_ra16) module"]
pub type ARB_RW4_RA16 = crate::Reg<u32, _ARB_RW4_RA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW4_RA16;
#[doc = "`read()` method returns [arb_rw4_ra16::R](arb_rw4_ra16::R) reader structure"]
impl crate::Readable for ARB_RW4_RA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw4_ra16::W](arb_rw4_ra16::W) writer structure"]
impl crate::Writable for ARB_RW4_RA16 {}
#[doc = "Endpoint Read Address value"]
pub mod arb_rw4_ra16;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw4_dr16](arb_rw4_dr16) module"]
pub type ARB_RW4_DR16 = crate::Reg<u32, _ARB_RW4_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW4_DR16;
#[doc = "`read()` method returns [arb_rw4_dr16::R](arb_rw4_dr16::R) reader structure"]
impl crate::Readable for ARB_RW4_DR16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw4_dr16::W](arb_rw4_dr16::W) writer structure"]
impl crate::Writable for ARB_RW4_DR16 {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr16;
#[doc = "DMA Burst / Threshold Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_thres16](dma_thres16) module"]
pub type DMA_THRES16 = crate::Reg<u32, _DMA_THRES16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_THRES16;
#[doc = "`read()` method returns [dma_thres16::R](dma_thres16::R) reader structure"]
impl crate::Readable for DMA_THRES16 {}
#[doc = "`write(|w| ..)` method takes [dma_thres16::W](dma_thres16::W) writer structure"]
impl crate::Writable for DMA_THRES16 {}
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres16;
#[doc = "Endpoint Write Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_wa16](arb_rw5_wa16) module"]
pub type ARB_RW5_WA16 = crate::Reg<u32, _ARB_RW5_WA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW5_WA16;
#[doc = "`read()` method returns [arb_rw5_wa16::R](arb_rw5_wa16::R) reader structure"]
impl crate::Readable for ARB_RW5_WA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw5_wa16::W](arb_rw5_wa16::W) writer structure"]
impl crate::Writable for ARB_RW5_WA16 {}
#[doc = "Endpoint Write Address value"]
pub mod arb_rw5_wa16;
#[doc = "Endpoint Read Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_ra16](arb_rw5_ra16) module"]
pub type ARB_RW5_RA16 = crate::Reg<u32, _ARB_RW5_RA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW5_RA16;
#[doc = "`read()` method returns [arb_rw5_ra16::R](arb_rw5_ra16::R) reader structure"]
impl crate::Readable for ARB_RW5_RA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw5_ra16::W](arb_rw5_ra16::W) writer structure"]
impl crate::Writable for ARB_RW5_RA16 {}
#[doc = "Endpoint Read Address value"]
pub mod arb_rw5_ra16;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_dr16](arb_rw5_dr16) module"]
pub type ARB_RW5_DR16 = crate::Reg<u32, _ARB_RW5_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW5_DR16;
#[doc = "`read()` method returns [arb_rw5_dr16::R](arb_rw5_dr16::R) reader structure"]
impl crate::Readable for ARB_RW5_DR16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw5_dr16::W](arb_rw5_dr16::W) writer structure"]
impl crate::Writable for ARB_RW5_DR16 {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr16;
#[doc = "Endpoint Write Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_wa16](arb_rw6_wa16) module"]
pub type ARB_RW6_WA16 = crate::Reg<u32, _ARB_RW6_WA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW6_WA16;
#[doc = "`read()` method returns [arb_rw6_wa16::R](arb_rw6_wa16::R) reader structure"]
impl crate::Readable for ARB_RW6_WA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw6_wa16::W](arb_rw6_wa16::W) writer structure"]
impl crate::Writable for ARB_RW6_WA16 {}
#[doc = "Endpoint Write Address value"]
pub mod arb_rw6_wa16;
#[doc = "Endpoint Read Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_ra16](arb_rw6_ra16) module"]
pub type ARB_RW6_RA16 = crate::Reg<u32, _ARB_RW6_RA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW6_RA16;
#[doc = "`read()` method returns [arb_rw6_ra16::R](arb_rw6_ra16::R) reader structure"]
impl crate::Readable for ARB_RW6_RA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw6_ra16::W](arb_rw6_ra16::W) writer structure"]
impl crate::Writable for ARB_RW6_RA16 {}
#[doc = "Endpoint Read Address value"]
pub mod arb_rw6_ra16;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_dr16](arb_rw6_dr16) module"]
pub type ARB_RW6_DR16 = crate::Reg<u32, _ARB_RW6_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW6_DR16;
#[doc = "`read()` method returns [arb_rw6_dr16::R](arb_rw6_dr16::R) reader structure"]
impl crate::Readable for ARB_RW6_DR16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw6_dr16::W](arb_rw6_dr16::W) writer structure"]
impl crate::Writable for ARB_RW6_DR16 {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr16;
#[doc = "Endpoint Write Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_wa16](arb_rw7_wa16) module"]
pub type ARB_RW7_WA16 = crate::Reg<u32, _ARB_RW7_WA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW7_WA16;
#[doc = "`read()` method returns [arb_rw7_wa16::R](arb_rw7_wa16::R) reader structure"]
impl crate::Readable for ARB_RW7_WA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw7_wa16::W](arb_rw7_wa16::W) writer structure"]
impl crate::Writable for ARB_RW7_WA16 {}
#[doc = "Endpoint Write Address value"]
pub mod arb_rw7_wa16;
#[doc = "Endpoint Read Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_ra16](arb_rw7_ra16) module"]
pub type ARB_RW7_RA16 = crate::Reg<u32, _ARB_RW7_RA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW7_RA16;
#[doc = "`read()` method returns [arb_rw7_ra16::R](arb_rw7_ra16::R) reader structure"]
impl crate::Readable for ARB_RW7_RA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw7_ra16::W](arb_rw7_ra16::W) writer structure"]
impl crate::Writable for ARB_RW7_RA16 {}
#[doc = "Endpoint Read Address value"]
pub mod arb_rw7_ra16;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw7_dr16](arb_rw7_dr16) module"]
pub type ARB_RW7_DR16 = crate::Reg<u32, _ARB_RW7_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW7_DR16;
#[doc = "`read()` method returns [arb_rw7_dr16::R](arb_rw7_dr16::R) reader structure"]
impl crate::Readable for ARB_RW7_DR16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw7_dr16::W](arb_rw7_dr16::W) writer structure"]
impl crate::Writable for ARB_RW7_DR16 {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr16;
#[doc = "Endpoint Write Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw8_wa16](arb_rw8_wa16) module"]
pub type ARB_RW8_WA16 = crate::Reg<u32, _ARB_RW8_WA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW8_WA16;
#[doc = "`read()` method returns [arb_rw8_wa16::R](arb_rw8_wa16::R) reader structure"]
impl crate::Readable for ARB_RW8_WA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw8_wa16::W](arb_rw8_wa16::W) writer structure"]
impl crate::Writable for ARB_RW8_WA16 {}
#[doc = "Endpoint Write Address value"]
pub mod arb_rw8_wa16;
#[doc = "Endpoint Read Address value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw8_ra16](arb_rw8_ra16) module"]
pub type ARB_RW8_RA16 = crate::Reg<u32, _ARB_RW8_RA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW8_RA16;
#[doc = "`read()` method returns [arb_rw8_ra16::R](arb_rw8_ra16::R) reader structure"]
impl crate::Readable for ARB_RW8_RA16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw8_ra16::W](arb_rw8_ra16::W) writer structure"]
impl crate::Writable for ARB_RW8_RA16 {}
#[doc = "Endpoint Read Address value"]
pub mod arb_rw8_ra16;
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw8_dr16](arb_rw8_dr16) module"]
pub type ARB_RW8_DR16 = crate::Reg<u32, _ARB_RW8_DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARB_RW8_DR16;
#[doc = "`read()` method returns [arb_rw8_dr16::R](arb_rw8_dr16::R) reader structure"]
impl crate::Readable for ARB_RW8_DR16 {}
#[doc = "`write(|w| ..)` method takes [arb_rw8_dr16::W](arb_rw8_dr16::W) writer structure"]
impl crate::Writable for ARB_RW8_DR16 {}
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr16;

#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_ctl](power_ctl) module"]
pub type POWER_CTL = crate::Reg<u32, _POWER_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER_CTL;
#[doc = "`read()` method returns [power_ctl::R](power_ctl::R) reader structure"]
impl crate::Readable for POWER_CTL {}
#[doc = "`write(|w| ..)` method takes [power_ctl::W](power_ctl::W) writer structure"]
impl crate::Writable for POWER_CTL {}
#[doc = "Power Control Register"]
pub mod power_ctl;
#[doc = "USB IO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbio_ctl](usbio_ctl) module"]
pub type USBIO_CTL = crate::Reg<u32, _USBIO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIO_CTL;
#[doc = "`read()` method returns [usbio_ctl::R](usbio_ctl::R) reader structure"]
impl crate::Readable for USBIO_CTL {}
#[doc = "`write(|w| ..)` method takes [usbio_ctl::W](usbio_ctl::W) writer structure"]
impl crate::Writable for USBIO_CTL {}
#[doc = "USB IO Control Register"]
pub mod usbio_ctl;
#[doc = "Flow Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow_ctl](flow_ctl) module"]
pub type FLOW_CTL = crate::Reg<u32, _FLOW_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOW_CTL;
#[doc = "`read()` method returns [flow_ctl::R](flow_ctl::R) reader structure"]
impl crate::Readable for FLOW_CTL {}
#[doc = "`write(|w| ..)` method takes [flow_ctl::W](flow_ctl::W) writer structure"]
impl crate::Writable for FLOW_CTL {}
#[doc = "Flow Control Register"]
pub mod flow_ctl;
#[doc = "LPM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpm_ctl](lpm_ctl) module"]
pub type LPM_CTL = crate::Reg<u32, _LPM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPM_CTL;
#[doc = "`read()` method returns [lpm_ctl::R](lpm_ctl::R) reader structure"]
impl crate::Readable for LPM_CTL {}
#[doc = "`write(|w| ..)` method takes [lpm_ctl::W](lpm_ctl::W) writer structure"]
impl crate::Writable for LPM_CTL {}
#[doc = "LPM Control Register"]
pub mod lpm_ctl;
#[doc = "LPM Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpm_stat](lpm_stat) module"]
pub type LPM_STAT = crate::Reg<u32, _LPM_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPM_STAT;
#[doc = "`read()` method returns [lpm_stat::R](lpm_stat::R) reader structure"]
impl crate::Readable for LPM_STAT {}
#[doc = "LPM Status register"]
pub mod lpm_stat;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_sie](intr_sie) module"]
pub type INTR_SIE = crate::Reg<u32, _INTR_SIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SIE;
#[doc = "`read()` method returns [intr_sie::R](intr_sie::R) reader structure"]
impl crate::Readable for INTR_SIE {}
#[doc = "`write(|w| ..)` method takes [intr_sie::W](intr_sie::W) writer structure"]
impl crate::Writable for INTR_SIE {}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status"]
pub mod intr_sie;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_sie_set](intr_sie_set) module"]
pub type INTR_SIE_SET = crate::Reg<u32, _INTR_SIE_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SIE_SET;
#[doc = "`read()` method returns [intr_sie_set::R](intr_sie_set::R) reader structure"]
impl crate::Readable for INTR_SIE_SET {}
#[doc = "`write(|w| ..)` method takes [intr_sie_set::W](intr_sie_set::W) writer structure"]
impl crate::Writable for INTR_SIE_SET {}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set"]
pub mod intr_sie_set;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_sie_mask](intr_sie_mask) module"]
pub type INTR_SIE_MASK = crate::Reg<u32, _INTR_SIE_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SIE_MASK;
#[doc = "`read()` method returns [intr_sie_mask::R](intr_sie_mask::R) reader structure"]
impl crate::Readable for INTR_SIE_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_sie_mask::W](intr_sie_mask::W) writer structure"]
impl crate::Writable for INTR_SIE_MASK {}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask"]
pub mod intr_sie_mask;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_sie_masked](intr_sie_masked) module"]
pub type INTR_SIE_MASKED = crate::Reg<u32, _INTR_SIE_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SIE_MASKED;
#[doc = "`read()` method returns [intr_sie_masked::R](intr_sie_masked::R) reader structure"]
impl crate::Readable for INTR_SIE_MASKED {}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked"]
pub mod intr_sie_masked;
#[doc = "Select interrupt level for each interrupt source\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_lvl_sel](intr_lvl_sel) module"]
pub type INTR_LVL_SEL = crate::Reg<u32, _INTR_LVL_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_LVL_SEL;
#[doc = "`read()` method returns [intr_lvl_sel::R](intr_lvl_sel::R) reader structure"]
impl crate::Readable for INTR_LVL_SEL {}
#[doc = "`write(|w| ..)` method takes [intr_lvl_sel::W](intr_lvl_sel::W) writer structure"]
impl crate::Writable for INTR_LVL_SEL {}
#[doc = "Select interrupt level for each interrupt source"]
pub mod intr_lvl_sel;
#[doc = "High priority interrupt Cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause_hi](intr_cause_hi) module"]
pub type INTR_CAUSE_HI = crate::Reg<u32, _INTR_CAUSE_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE_HI;
#[doc = "`read()` method returns [intr_cause_hi::R](intr_cause_hi::R) reader structure"]
impl crate::Readable for INTR_CAUSE_HI {}
#[doc = "High priority interrupt Cause register"]
pub mod intr_cause_hi;
#[doc = "Medium priority interrupt Cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause_med](intr_cause_med) module"]
pub type INTR_CAUSE_MED = crate::Reg<u32, _INTR_CAUSE_MED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE_MED;
#[doc = "`read()` method returns [intr_cause_med::R](intr_cause_med::R) reader structure"]
impl crate::Readable for INTR_CAUSE_MED {}
#[doc = "Medium priority interrupt Cause register"]
pub mod intr_cause_med;
#[doc = "Low priority interrupt Cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause_lo](intr_cause_lo) module"]
pub type INTR_CAUSE_LO = crate::Reg<u32, _INTR_CAUSE_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE_LO;
#[doc = "`read()` method returns [intr_cause_lo::R](intr_cause_lo::R) reader structure"]
impl crate::Readable for INTR_CAUSE_LO {}
#[doc = "Low priority interrupt Cause register"]
pub mod intr_cause_lo;
#[doc = "DFT control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dft_ctl](dft_ctl) module"]
pub type DFT_CTL = crate::Reg<u32, _DFT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFT_CTL;
#[doc = "`read()` method returns [dft_ctl::R](dft_ctl::R) reader structure"]
impl crate::Readable for DFT_CTL {}
#[doc = "`write(|w| ..)` method takes [dft_ctl::W](dft_ctl::W) writer structure"]
impl crate::Writable for DFT_CTL {}
#[doc = "DFT control"]
pub mod dft_ctl;

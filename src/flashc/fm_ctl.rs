#[doc = "Flash macro control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_ctl](fm_ctl) module"]
pub type FM_CTL = crate::Reg<u32, _FM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_CTL;
#[doc = "`read()` method returns [fm_ctl::R](fm_ctl::R) reader structure"]
impl crate::Readable for FM_CTL {}
#[doc = "`write(|w| ..)` method takes [fm_ctl::W](fm_ctl::W) writer structure"]
impl crate::Writable for FM_CTL {}
#[doc = "Flash macro control"]
pub mod fm_ctl;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Flash macro address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_addr](fm_addr) module"]
pub type FM_ADDR = crate::Reg<u32, _FM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_ADDR;
#[doc = "`read()` method returns [fm_addr::R](fm_addr::R) reader structure"]
impl crate::Readable for FM_ADDR {}
#[doc = "`write(|w| ..)` method takes [fm_addr::W](fm_addr::W) writer structure"]
impl crate::Writable for FM_ADDR {}
#[doc = "Flash macro address"]
pub mod fm_addr;
#[doc = "Regular flash geometry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry](geometry) module"]
pub type GEOMETRY = crate::Reg<u32, _GEOMETRY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEOMETRY;
#[doc = "`read()` method returns [geometry::R](geometry::R) reader structure"]
impl crate::Readable for GEOMETRY {}
#[doc = "Regular flash geometry"]
pub mod geometry;
#[doc = "Supervisory flash geometry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry_supervisory](geometry_supervisory) module"]
pub type GEOMETRY_SUPERVISORY = crate::Reg<u32, _GEOMETRY_SUPERVISORY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEOMETRY_SUPERVISORY;
#[doc = "`read()` method returns [geometry_supervisory::R](geometry_supervisory::R) reader structure"]
impl crate::Readable for GEOMETRY_SUPERVISORY {}
#[doc = "Supervisory flash geometry"]
pub mod geometry_supervisory;
#[doc = "Timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_ctl](timer_ctl) module"]
pub type TIMER_CTL = crate::Reg<u32, _TIMER_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_CTL;
#[doc = "`read()` method returns [timer_ctl::R](timer_ctl::R) reader structure"]
impl crate::Readable for TIMER_CTL {}
#[doc = "`write(|w| ..)` method takes [timer_ctl::W](timer_ctl::W) writer structure"]
impl crate::Writable for TIMER_CTL {}
#[doc = "Timer control"]
pub mod timer_ctl;
#[doc = "Analog control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctl0](ana_ctl0) module"]
pub type ANA_CTL0 = crate::Reg<u32, _ANA_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_CTL0;
#[doc = "`read()` method returns [ana_ctl0::R](ana_ctl0::R) reader structure"]
impl crate::Readable for ANA_CTL0 {}
#[doc = "`write(|w| ..)` method takes [ana_ctl0::W](ana_ctl0::W) writer structure"]
impl crate::Writable for ANA_CTL0 {}
#[doc = "Analog control 0"]
pub mod ana_ctl0;
#[doc = "Analog control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctl1](ana_ctl1) module"]
pub type ANA_CTL1 = crate::Reg<u32, _ANA_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_CTL1;
#[doc = "`read()` method returns [ana_ctl1::R](ana_ctl1::R) reader structure"]
impl crate::Readable for ANA_CTL1 {}
#[doc = "`write(|w| ..)` method takes [ana_ctl1::W](ana_ctl1::W) writer structure"]
impl crate::Writable for ANA_CTL1 {}
#[doc = "Analog control 1"]
pub mod ana_ctl1;
#[doc = "N/A, DNU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry_gen](geometry_gen) module"]
pub type GEOMETRY_GEN = crate::Reg<u32, _GEOMETRY_GEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEOMETRY_GEN;
#[doc = "`read()` method returns [geometry_gen::R](geometry_gen::R) reader structure"]
impl crate::Readable for GEOMETRY_GEN {}
#[doc = "N/A, DNU"]
pub mod geometry_gen;
#[doc = "Test mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_ctl](test_ctl) module"]
pub type TEST_CTL = crate::Reg<u32, _TEST_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST_CTL;
#[doc = "`read()` method returns [test_ctl::R](test_ctl::R) reader structure"]
impl crate::Readable for TEST_CTL {}
#[doc = "`write(|w| ..)` method takes [test_ctl::W](test_ctl::W) writer structure"]
impl crate::Writable for TEST_CTL {}
#[doc = "Test mode control"]
pub mod test_ctl;
#[doc = "Wiat State control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wait_ctl](wait_ctl) module"]
pub type WAIT_CTL = crate::Reg<u32, _WAIT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAIT_CTL;
#[doc = "`read()` method returns [wait_ctl::R](wait_ctl::R) reader structure"]
impl crate::Readable for WAIT_CTL {}
#[doc = "`write(|w| ..)` method takes [wait_ctl::W](wait_ctl::W) writer structure"]
impl crate::Writable for WAIT_CTL {}
#[doc = "Wiat State control"]
pub mod wait_ctl;
#[doc = "Monitor Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monitor_status](monitor_status) module"]
pub type MONITOR_STATUS = crate::Reg<u32, _MONITOR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MONITOR_STATUS;
#[doc = "`read()` method returns [monitor_status::R](monitor_status::R) reader structure"]
impl crate::Readable for MONITOR_STATUS {}
#[doc = "Monitor Status"]
pub mod monitor_status;
#[doc = "Scratch Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch_ctl](scratch_ctl) module"]
pub type SCRATCH_CTL = crate::Reg<u32, _SCRATCH_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH_CTL;
#[doc = "`read()` method returns [scratch_ctl::R](scratch_ctl::R) reader structure"]
impl crate::Readable for SCRATCH_CTL {}
#[doc = "`write(|w| ..)` method takes [scratch_ctl::W](scratch_ctl::W) writer structure"]
impl crate::Writable for SCRATCH_CTL {}
#[doc = "Scratch Control"]
pub mod scratch_ctl;
#[doc = "High voltage control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hv_ctl](hv_ctl) module"]
pub type HV_CTL = crate::Reg<u32, _HV_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HV_CTL;
#[doc = "`read()` method returns [hv_ctl::R](hv_ctl::R) reader structure"]
impl crate::Readable for HV_CTL {}
#[doc = "`write(|w| ..)` method takes [hv_ctl::W](hv_ctl::W) writer structure"]
impl crate::Writable for HV_CTL {}
#[doc = "High voltage control"]
pub mod hv_ctl;
#[doc = "Aclk control\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aclk_ctl](aclk_ctl) module"]
pub type ACLK_CTL = crate::Reg<u32, _ACLK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACLK_CTL;
#[doc = "`write(|w| ..)` method takes [aclk_ctl::W](aclk_ctl::W) writer structure"]
impl crate::Writable for ACLK_CTL {}
#[doc = "Aclk control"]
pub mod aclk_ctl;
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
#[doc = "Flash macro high Voltage page latches data (for all page latches)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_hv_data_all](fm_hv_data_all) module"]
pub type FM_HV_DATA_ALL = crate::Reg<u32, _FM_HV_DATA_ALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_HV_DATA_ALL;
#[doc = "`write(|w| ..)` method takes [fm_hv_data_all::W](fm_hv_data_all::W) writer structure"]
impl crate::Writable for FM_HV_DATA_ALL {}
#[doc = "Flash macro high Voltage page latches data (for all page latches)"]
pub mod fm_hv_data_all;
#[doc = "Cal control BG LO trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl0](cal_ctl0) module"]
pub type CAL_CTL0 = crate::Reg<u32, _CAL_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL0;
#[doc = "`read()` method returns [cal_ctl0::R](cal_ctl0::R) reader structure"]
impl crate::Readable for CAL_CTL0 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl0::W](cal_ctl0::W) writer structure"]
impl crate::Writable for CAL_CTL0 {}
#[doc = "Cal control BG LO trim bits"]
pub mod cal_ctl0;
#[doc = "Cal control BG HI trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl1](cal_ctl1) module"]
pub type CAL_CTL1 = crate::Reg<u32, _CAL_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL1;
#[doc = "`read()` method returns [cal_ctl1::R](cal_ctl1::R) reader structure"]
impl crate::Readable for CAL_CTL1 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl1::W](cal_ctl1::W) writer structure"]
impl crate::Writable for CAL_CTL1 {}
#[doc = "Cal control BG HI trim bits"]
pub mod cal_ctl1;
#[doc = "Cal control BG LO&HI ipref trim, ref sel, fm_active, turbo_ext\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl2](cal_ctl2) module"]
pub type CAL_CTL2 = crate::Reg<u32, _CAL_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL2;
#[doc = "`read()` method returns [cal_ctl2::R](cal_ctl2::R) reader structure"]
impl crate::Readable for CAL_CTL2 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl2::W](cal_ctl2::W) writer structure"]
impl crate::Writable for CAL_CTL2 {}
#[doc = "Cal control BG LO&HI ipref trim, ref sel, fm_active, turbo_ext"]
pub mod cal_ctl2;
#[doc = "Cal control osc trim bits, idac, sdac, itim, bdac.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl3](cal_ctl3) module"]
pub type CAL_CTL3 = crate::Reg<u32, _CAL_CTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL3;
#[doc = "`read()` method returns [cal_ctl3::R](cal_ctl3::R) reader structure"]
impl crate::Readable for CAL_CTL3 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl3::W](cal_ctl3::W) writer structure"]
impl crate::Writable for CAL_CTL3 {}
#[doc = "Cal control osc trim bits, idac, sdac, itim, bdac."]
pub mod cal_ctl3;
#[doc = "Bookmark register - keeps the current FW HV seq\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bookmark](bookmark) module"]
pub type BOOKMARK = crate::Reg<u32, _BOOKMARK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOKMARK;
#[doc = "`write(|w| ..)` method takes [bookmark::W](bookmark::W) writer structure"]
impl crate::Writable for BOOKMARK {}
#[doc = "Bookmark register - keeps the current FW HV seq"]
pub mod bookmark;
#[doc = "Redundancy Control normal sectors 0,1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl01](red_ctl01) module"]
pub type RED_CTL01 = crate::Reg<u32, _RED_CTL01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL01;
#[doc = "`read()` method returns [red_ctl01::R](red_ctl01::R) reader structure"]
impl crate::Readable for RED_CTL01 {}
#[doc = "`write(|w| ..)` method takes [red_ctl01::W](red_ctl01::W) writer structure"]
impl crate::Writable for RED_CTL01 {}
#[doc = "Redundancy Control normal sectors 0,1"]
pub mod red_ctl01;
#[doc = "Redundancy Controll normal sectors 2,3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl23](red_ctl23) module"]
pub type RED_CTL23 = crate::Reg<u32, _RED_CTL23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL23;
#[doc = "`read()` method returns [red_ctl23::R](red_ctl23::R) reader structure"]
impl crate::Readable for RED_CTL23 {}
#[doc = "`write(|w| ..)` method takes [red_ctl23::W](red_ctl23::W) writer structure"]
impl crate::Writable for RED_CTL23 {}
#[doc = "Redundancy Controll normal sectors 2,3"]
pub mod red_ctl23;
#[doc = "Redundancy Controll normal sectors 4,5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl45](red_ctl45) module"]
pub type RED_CTL45 = crate::Reg<u32, _RED_CTL45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL45;
#[doc = "`read()` method returns [red_ctl45::R](red_ctl45::R) reader structure"]
impl crate::Readable for RED_CTL45 {}
#[doc = "`write(|w| ..)` method takes [red_ctl45::W](red_ctl45::W) writer structure"]
impl crate::Writable for RED_CTL45 {}
#[doc = "Redundancy Controll normal sectors 4,5"]
pub mod red_ctl45;
#[doc = "Redundancy Controll normal sectors 6,7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl67](red_ctl67) module"]
pub type RED_CTL67 = crate::Reg<u32, _RED_CTL67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL67;
#[doc = "`read()` method returns [red_ctl67::R](red_ctl67::R) reader structure"]
impl crate::Readable for RED_CTL67 {}
#[doc = "`write(|w| ..)` method takes [red_ctl67::W](red_ctl67::W) writer structure"]
impl crate::Writable for RED_CTL67 {}
#[doc = "Redundancy Controll normal sectors 6,7"]
pub mod red_ctl67;
#[doc = "Redundancy Controll special sectors 0,1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl_sm01](red_ctl_sm01) module"]
pub type RED_CTL_SM01 = crate::Reg<u32, _RED_CTL_SM01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL_SM01;
#[doc = "`read()` method returns [red_ctl_sm01::R](red_ctl_sm01::R) reader structure"]
impl crate::Readable for RED_CTL_SM01 {}
#[doc = "`write(|w| ..)` method takes [red_ctl_sm01::W](red_ctl_sm01::W) writer structure"]
impl crate::Writable for RED_CTL_SM01 {}
#[doc = "Redundancy Controll special sectors 0,1"]
pub mod red_ctl_sm01;
#[doc = "Do Not Use\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tm_cmpr](tm_cmpr) module"]
pub type TM_CMPR = crate::Reg<u32, _TM_CMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TM_CMPR;
#[doc = "`read()` method returns [tm_cmpr::R](tm_cmpr::R) reader structure"]
impl crate::Readable for TM_CMPR {}
#[doc = "Do Not Use"]
pub mod tm_cmpr;
#[doc = "Flash macro high Voltage page latches data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_hv_data](fm_hv_data) module"]
pub type FM_HV_DATA = crate::Reg<u32, _FM_HV_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_HV_DATA;
#[doc = "`read()` method returns [fm_hv_data::R](fm_hv_data::R) reader structure"]
impl crate::Readable for FM_HV_DATA {}
#[doc = "`write(|w| ..)` method takes [fm_hv_data::W](fm_hv_data::W) writer structure"]
impl crate::Writable for FM_HV_DATA {}
#[doc = "Flash macro high Voltage page latches data"]
pub mod fm_hv_data;
#[doc = "Flash macro memory sense amplifier and column decoder data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_mem_data](fm_mem_data) module"]
pub type FM_MEM_DATA = crate::Reg<u32, _FM_MEM_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_MEM_DATA;
#[doc = "`read()` method returns [fm_mem_data::R](fm_mem_data::R) reader structure"]
impl crate::Readable for FM_MEM_DATA {}
#[doc = "Flash macro memory sense amplifier and column decoder data"]
pub mod fm_mem_data;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - RTC Read Write register"]
    pub rtc_rw: RTC_RW,
    #[doc = "0x0c - Oscillator calibration for absolute frequency"]
    pub cal_ctl: CAL_CTL,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - Calendar Seconds, Minutes, Hours, Day of Week"]
    pub rtc_time: RTC_TIME,
    #[doc = "0x18 - Calendar Day of Month, Month, Year"]
    pub rtc_date: RTC_DATE,
    #[doc = "0x1c - Alarm 1 Seconds, Minute, Hours, Day of Week"]
    pub alm1_time: ALM1_TIME,
    #[doc = "0x20 - Alarm 1 Day of Month, Month"]
    pub alm1_date: ALM1_DATE,
    #[doc = "0x24 - Alarm 2 Seconds, Minute, Hours, Day of Week"]
    pub alm2_time: ALM2_TIME,
    #[doc = "0x28 - Alarm 2 Day of Month, Month"]
    pub alm2_date: ALM2_DATE,
    #[doc = "0x2c - Interrupt request register"]
    pub intr: INTR,
    #[doc = "0x30 - Interrupt set request register"]
    pub intr_set: INTR_SET,
    #[doc = "0x34 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x38 - Interrupt masked request register"]
    pub intr_masked: INTR_MASKED,
    #[doc = "0x3c - 32kHz oscillator counter"]
    pub osccnt: OSCCNT,
    #[doc = "0x40 - 128Hz tick counter"]
    pub ticks: TICKS,
    #[doc = "0x44 - PMIC control register"]
    pub pmic_ctl: PMIC_CTL,
    #[doc = "0x48 - Backup reset register"]
    pub reset: RESET,
    _reserved18: [u8; 4020usize],
    #[doc = "0x1000 - Backup register region"]
    pub breg: [BREG; 64],
    _reserved19: [u8; 60928usize],
    #[doc = "0xff00 - Trim Register"]
    pub trim: TRIM,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "RTC Read Write register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_rw](rtc_rw) module"]
pub type RTC_RW = crate::Reg<u32, _RTC_RW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_RW;
#[doc = "`read()` method returns [rtc_rw::R](rtc_rw::R) reader structure"]
impl crate::Readable for RTC_RW {}
#[doc = "`write(|w| ..)` method takes [rtc_rw::W](rtc_rw::W) writer structure"]
impl crate::Writable for RTC_RW {}
#[doc = "RTC Read Write register"]
pub mod rtc_rw;
#[doc = "Oscillator calibration for absolute frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl](cal_ctl) module"]
pub type CAL_CTL = crate::Reg<u32, _CAL_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL;
#[doc = "`read()` method returns [cal_ctl::R](cal_ctl::R) reader structure"]
impl crate::Readable for CAL_CTL {}
#[doc = "`write(|w| ..)` method takes [cal_ctl::W](cal_ctl::W) writer structure"]
impl crate::Writable for CAL_CTL {}
#[doc = "Oscillator calibration for absolute frequency"]
pub mod cal_ctl;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time](rtc_time) module"]
pub type RTC_TIME = crate::Reg<u32, _RTC_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TIME;
#[doc = "`read()` method returns [rtc_time::R](rtc_time::R) reader structure"]
impl crate::Readable for RTC_TIME {}
#[doc = "`write(|w| ..)` method takes [rtc_time::W](rtc_time::W) writer structure"]
impl crate::Writable for RTC_TIME {}
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week"]
pub mod rtc_time;
#[doc = "Calendar Day of Month, Month, Year\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_date](rtc_date) module"]
pub type RTC_DATE = crate::Reg<u32, _RTC_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_DATE;
#[doc = "`read()` method returns [rtc_date::R](rtc_date::R) reader structure"]
impl crate::Readable for RTC_DATE {}
#[doc = "`write(|w| ..)` method takes [rtc_date::W](rtc_date::W) writer structure"]
impl crate::Writable for RTC_DATE {}
#[doc = "Calendar Day of Month, Month, Year"]
pub mod rtc_date;
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm1_time](alm1_time) module"]
pub type ALM1_TIME = crate::Reg<u32, _ALM1_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALM1_TIME;
#[doc = "`read()` method returns [alm1_time::R](alm1_time::R) reader structure"]
impl crate::Readable for ALM1_TIME {}
#[doc = "`write(|w| ..)` method takes [alm1_time::W](alm1_time::W) writer structure"]
impl crate::Writable for ALM1_TIME {}
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week"]
pub mod alm1_time;
#[doc = "Alarm 1 Day of Month, Month\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm1_date](alm1_date) module"]
pub type ALM1_DATE = crate::Reg<u32, _ALM1_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALM1_DATE;
#[doc = "`read()` method returns [alm1_date::R](alm1_date::R) reader structure"]
impl crate::Readable for ALM1_DATE {}
#[doc = "`write(|w| ..)` method takes [alm1_date::W](alm1_date::W) writer structure"]
impl crate::Writable for ALM1_DATE {}
#[doc = "Alarm 1 Day of Month, Month"]
pub mod alm1_date;
#[doc = "Alarm 2 Seconds, Minute, Hours, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm2_time](alm2_time) module"]
pub type ALM2_TIME = crate::Reg<u32, _ALM2_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALM2_TIME;
#[doc = "`read()` method returns [alm2_time::R](alm2_time::R) reader structure"]
impl crate::Readable for ALM2_TIME {}
#[doc = "`write(|w| ..)` method takes [alm2_time::W](alm2_time::W) writer structure"]
impl crate::Writable for ALM2_TIME {}
#[doc = "Alarm 2 Seconds, Minute, Hours, Day of Week"]
pub mod alm2_time;
#[doc = "Alarm 2 Day of Month, Month\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm2_date](alm2_date) module"]
pub type ALM2_DATE = crate::Reg<u32, _ALM2_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALM2_DATE;
#[doc = "`read()` method returns [alm2_date::R](alm2_date::R) reader structure"]
impl crate::Readable for ALM2_DATE {}
#[doc = "`write(|w| ..)` method takes [alm2_date::W](alm2_date::W) writer structure"]
impl crate::Writable for ALM2_DATE {}
#[doc = "Alarm 2 Day of Month, Month"]
pub mod alm2_date;
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
#[doc = "32kHz oscillator counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccnt](osccnt) module"]
pub type OSCCNT = crate::Reg<u32, _OSCCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCNT;
#[doc = "`read()` method returns [osccnt::R](osccnt::R) reader structure"]
impl crate::Readable for OSCCNT {}
#[doc = "32kHz oscillator counter"]
pub mod osccnt;
#[doc = "128Hz tick counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ticks](ticks) module"]
pub type TICKS = crate::Reg<u32, _TICKS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TICKS;
#[doc = "`read()` method returns [ticks::R](ticks::R) reader structure"]
impl crate::Readable for TICKS {}
#[doc = "128Hz tick counter"]
pub mod ticks;
#[doc = "PMIC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmic_ctl](pmic_ctl) module"]
pub type PMIC_CTL = crate::Reg<u32, _PMIC_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMIC_CTL;
#[doc = "`read()` method returns [pmic_ctl::R](pmic_ctl::R) reader structure"]
impl crate::Readable for PMIC_CTL {}
#[doc = "`write(|w| ..)` method takes [pmic_ctl::W](pmic_ctl::W) writer structure"]
impl crate::Writable for PMIC_CTL {}
#[doc = "PMIC control register"]
pub mod pmic_ctl;
#[doc = "Backup reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](reset) module"]
pub type RESET = crate::Reg<u32, _RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET;
#[doc = "`read()` method returns [reset::R](reset::R) reader structure"]
impl crate::Readable for RESET {}
#[doc = "`write(|w| ..)` method takes [reset::W](reset::W) writer structure"]
impl crate::Writable for RESET {}
#[doc = "Backup reset register"]
pub mod reset;
#[doc = "Backup register region\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [breg](breg) module"]
pub type BREG = crate::Reg<u32, _BREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BREG;
#[doc = "`read()` method returns [breg::R](breg::R) reader structure"]
impl crate::Readable for BREG {}
#[doc = "`write(|w| ..)` method takes [breg::W](breg::W) writer structure"]
impl crate::Writable for BREG {}
#[doc = "Backup register region"]
pub mod breg;
#[doc = "Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim](trim) module"]
pub type TRIM = crate::Reg<u32, _TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM;
#[doc = "`read()` method returns [trim::R](trim::R) reader structure"]
impl crate::Readable for TRIM {}
#[doc = "`write(|w| ..)` method takes [trim::W](trim::W) writer structure"]
impl crate::Writable for TRIM {}
#[doc = "Trim Register"]
pub mod trim;

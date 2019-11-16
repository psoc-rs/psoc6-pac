#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Mode Control"]
    pub pwr_ctl: PWR_CTL,
    #[doc = "0x04 - HIBERNATE Mode Register"]
    pub pwr_hibernate: PWR_HIBERNATE,
    #[doc = "0x08 - Low Voltage Detector (LVD) Configuration Register"]
    pub pwr_lvd_ctl: PWR_LVD_CTL,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - Buck Control Register"]
    pub pwr_buck_ctl: PWR_BUCK_CTL,
    #[doc = "0x18 - Buck Control Register 2"]
    pub pwr_buck_ctl2: PWR_BUCK_CTL2,
    #[doc = "0x1c - Low Voltage Detector (LVD) Status Register"]
    pub pwr_lvd_status: PWR_LVD_STATUS,
    _reserved6: [u8; 96usize],
    #[doc = "0x80 - HIBERNATE Data Register"]
    pub pwr_hib_data: [PWR_HIB_DATA; 16],
    _reserved7: [u8; 192usize],
    #[doc = "0x180 - Watchdog Counter Control Register"]
    pub wdt_ctl: WDT_CTL,
    #[doc = "0x184 - Watchdog Counter Count Register"]
    pub wdt_cnt: WDT_CNT,
    #[doc = "0x188 - Watchdog Counter Match Register"]
    pub wdt_match: WDT_MATCH,
    _reserved10: [u8; 116usize],
    #[doc = "0x200 - Multi-Counter Watchdog Timer"]
    pub mcwdt_struct0: MCWDT_STRUCT,
    _reserved11: [u8; 20usize],
    #[doc = "0x240 - Multi-Counter Watchdog Timer"]
    pub mcwdt_struct1: MCWDT_STRUCT,
    _reserved12: [u8; 148usize],
    #[doc = "0x300 - Clock DSI Select Register"]
    pub clk_dsi_select: [CLK_DSI_SELECT; 16],
    #[doc = "0x340 - Clock Path Select Register"]
    pub clk_path_select: [CLK_PATH_SELECT; 16],
    #[doc = "0x380 - Clock Root Select Register"]
    pub clk_root_select: [CLK_ROOT_SELECT; 16],
    _reserved15: [u8; 320usize],
    #[doc = "0x500 - Clock selection register"]
    pub clk_select: CLK_SELECT,
    #[doc = "0x504 - Timer Clock Control Register"]
    pub clk_timer_ctl: CLK_TIMER_CTL,
    _reserved17: [u8; 4usize],
    #[doc = "0x50c - ILO Configuration"]
    pub clk_ilo_config: CLK_ILO_CONFIG,
    #[doc = "0x510 - IMO Configuration"]
    pub clk_imo_config: CLK_IMO_CONFIG,
    #[doc = "0x514 - Fast Clock Output Select Register"]
    pub clk_output_fast: CLK_OUTPUT_FAST,
    #[doc = "0x518 - Slow Clock Output Select Register"]
    pub clk_output_slow: CLK_OUTPUT_SLOW,
    #[doc = "0x51c - Clock Calibration Counter 1"]
    pub clk_cal_cnt1: CLK_CAL_CNT1,
    #[doc = "0x520 - Clock Calibration Counter 2"]
    pub clk_cal_cnt2: CLK_CAL_CNT2,
    _reserved23: [u8; 8usize],
    #[doc = "0x52c - ECO Configuration Register"]
    pub clk_eco_config: CLK_ECO_CONFIG,
    #[doc = "0x530 - ECO Status Register"]
    pub clk_eco_status: CLK_ECO_STATUS,
    _reserved25: [u8; 8usize],
    #[doc = "0x53c - Precision ILO Configuration Register"]
    pub clk_pilo_config: CLK_PILO_CONFIG,
    _reserved26: [u8; 64usize],
    #[doc = "0x580 - FLL Configuration Register"]
    pub clk_fll_config: CLK_FLL_CONFIG,
    #[doc = "0x584 - FLL Configuration Register 2"]
    pub clk_fll_config2: CLK_FLL_CONFIG2,
    #[doc = "0x588 - FLL Configuration Register 3"]
    pub clk_fll_config3: CLK_FLL_CONFIG3,
    #[doc = "0x58c - FLL Configuration Register 4"]
    pub clk_fll_config4: CLK_FLL_CONFIG4,
    #[doc = "0x590 - FLL Status Register"]
    pub clk_fll_status: CLK_FLL_STATUS,
    _reserved31: [u8; 108usize],
    #[doc = "0x600 - PLL Configuration Register"]
    pub clk_pll_config: [CLK_PLL_CONFIG; 15],
    _reserved32: [u8; 4usize],
    #[doc = "0x640 - PLL Status Register"]
    pub clk_pll_status: [CLK_PLL_STATUS; 15],
    _reserved33: [u8; 132usize],
    #[doc = "0x700 - SRSS Interrupt Register"]
    pub srss_intr: SRSS_INTR,
    #[doc = "0x704 - SRSS Interrupt Set Register"]
    pub srss_intr_set: SRSS_INTR_SET,
    #[doc = "0x708 - SRSS Interrupt Mask Register"]
    pub srss_intr_mask: SRSS_INTR_MASK,
    #[doc = "0x70c - SRSS Interrupt Masked Register"]
    pub srss_intr_masked: SRSS_INTR_MASKED,
    #[doc = "0x710 - SRSS Interrupt Configuration Register"]
    pub srss_intr_cfg: SRSS_INTR_CFG,
    _reserved38: [u8; 236usize],
    #[doc = "0x800 - Reset Cause Observation Register"]
    pub res_cause: RES_CAUSE,
    #[doc = "0x804 - Reset Cause Observation Register 2"]
    pub res_cause2: RES_CAUSE2,
    _reserved40: [u8; 30456usize],
    #[doc = "0x7f00 - Reference Trim Register"]
    pub pwr_trim_ref_ctl: PWR_TRIM_REF_CTL,
    #[doc = "0x7f04 - BOD/OVP Trim Register"]
    pub pwr_trim_bodovp_ctl: PWR_TRIM_BODOVP_CTL,
    #[doc = "0x7f08 - CCO Trim Register"]
    pub clk_trim_cco_ctl: CLK_TRIM_CCO_CTL,
    #[doc = "0x7f0c - CCO Trim Register 2"]
    pub clk_trim_cco_ctl2: CLK_TRIM_CCO_CTL2,
    _reserved44: [u8; 32usize],
    #[doc = "0x7f30 - Wakeup Trim Register"]
    pub pwr_trim_wake_ctl: PWR_TRIM_WAKE_CTL,
    _reserved45: [u8; 32732usize],
    #[doc = "0xff10 - LVD Trim Register"]
    pub pwr_trim_lvd_ctl: PWR_TRIM_LVD_CTL,
    _reserved46: [u8; 4usize],
    #[doc = "0xff18 - ILO Trim Register"]
    pub clk_trim_ilo_ctl: CLK_TRIM_ILO_CTL,
    #[doc = "0xff1c - Power System Trim Register"]
    pub pwr_trim_pwrsys_ctl: PWR_TRIM_PWRSYS_CTL,
    #[doc = "0xff20 - ECO Trim Register"]
    pub clk_trim_eco_ctl: CLK_TRIM_ECO_CTL,
    #[doc = "0xff24 - PILO Trim Register"]
    pub clk_trim_pilo_ctl: CLK_TRIM_PILO_CTL,
    #[doc = "0xff28 - PILO Trim Register 2"]
    pub clk_trim_pilo_ctl2: CLK_TRIM_PILO_CTL2,
    #[doc = "0xff2c - PILO Trim Register 3"]
    pub clk_trim_pilo_ctl3: CLK_TRIM_PILO_CTL3,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MCWDT_STRUCT {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Multi-Counter Watchdog Sub-counters 0/1"]
    pub mcwdt_cntlow: self::mcwdt_struct::MCWDT_CNTLOW,
    #[doc = "0x08 - Multi-Counter Watchdog Sub-counter 2"]
    pub mcwdt_cnthigh: self::mcwdt_struct::MCWDT_CNTHIGH,
    #[doc = "0x0c - Multi-Counter Watchdog Counter Match Register"]
    pub mcwdt_match: self::mcwdt_struct::MCWDT_MATCH,
    #[doc = "0x10 - Multi-Counter Watchdog Counter Configuration"]
    pub mcwdt_config: self::mcwdt_struct::MCWDT_CONFIG,
    #[doc = "0x14 - Multi-Counter Watchdog Counter Control"]
    pub mcwdt_ctl: self::mcwdt_struct::MCWDT_CTL,
    #[doc = "0x18 - Multi-Counter Watchdog Counter Interrupt Register"]
    pub mcwdt_intr: self::mcwdt_struct::MCWDT_INTR,
    #[doc = "0x1c - Multi-Counter Watchdog Counter Interrupt Set Register"]
    pub mcwdt_intr_set: self::mcwdt_struct::MCWDT_INTR_SET,
    #[doc = "0x20 - Multi-Counter Watchdog Counter Interrupt Mask Register"]
    pub mcwdt_intr_mask: self::mcwdt_struct::MCWDT_INTR_MASK,
    #[doc = "0x24 - Multi-Counter Watchdog Counter Interrupt Masked Register"]
    pub mcwdt_intr_masked: self::mcwdt_struct::MCWDT_INTR_MASKED,
    #[doc = "0x28 - Multi-Counter Watchdog Counter Lock Register"]
    pub mcwdt_lock: self::mcwdt_struct::MCWDT_LOCK,
}
#[doc = r"Register block"]
#[doc = "Multi-Counter Watchdog Timer"]
pub mod mcwdt_struct;
#[doc = "Power Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctl](pwr_ctl) module"]
pub type PWR_CTL = crate::Reg<u32, _PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_CTL;
#[doc = "`read()` method returns [pwr_ctl::R](pwr_ctl::R) reader structure"]
impl crate::Readable for PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [pwr_ctl::W](pwr_ctl::W) writer structure"]
impl crate::Writable for PWR_CTL {}
#[doc = "Power Mode Control"]
pub mod pwr_ctl;
#[doc = "HIBERNATE Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_hibernate](pwr_hibernate) module"]
pub type PWR_HIBERNATE = crate::Reg<u32, _PWR_HIBERNATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_HIBERNATE;
#[doc = "`read()` method returns [pwr_hibernate::R](pwr_hibernate::R) reader structure"]
impl crate::Readable for PWR_HIBERNATE {}
#[doc = "`write(|w| ..)` method takes [pwr_hibernate::W](pwr_hibernate::W) writer structure"]
impl crate::Writable for PWR_HIBERNATE {}
#[doc = "HIBERNATE Mode Register"]
pub mod pwr_hibernate;
#[doc = "Low Voltage Detector (LVD) Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_lvd_ctl](pwr_lvd_ctl) module"]
pub type PWR_LVD_CTL = crate::Reg<u32, _PWR_LVD_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_LVD_CTL;
#[doc = "`read()` method returns [pwr_lvd_ctl::R](pwr_lvd_ctl::R) reader structure"]
impl crate::Readable for PWR_LVD_CTL {}
#[doc = "`write(|w| ..)` method takes [pwr_lvd_ctl::W](pwr_lvd_ctl::W) writer structure"]
impl crate::Writable for PWR_LVD_CTL {}
#[doc = "Low Voltage Detector (LVD) Configuration Register"]
pub mod pwr_lvd_ctl;
#[doc = "Buck Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_buck_ctl](pwr_buck_ctl) module"]
pub type PWR_BUCK_CTL = crate::Reg<u32, _PWR_BUCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_BUCK_CTL;
#[doc = "`read()` method returns [pwr_buck_ctl::R](pwr_buck_ctl::R) reader structure"]
impl crate::Readable for PWR_BUCK_CTL {}
#[doc = "`write(|w| ..)` method takes [pwr_buck_ctl::W](pwr_buck_ctl::W) writer structure"]
impl crate::Writable for PWR_BUCK_CTL {}
#[doc = "Buck Control Register"]
pub mod pwr_buck_ctl;
#[doc = "Buck Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_buck_ctl2](pwr_buck_ctl2) module"]
pub type PWR_BUCK_CTL2 = crate::Reg<u32, _PWR_BUCK_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_BUCK_CTL2;
#[doc = "`read()` method returns [pwr_buck_ctl2::R](pwr_buck_ctl2::R) reader structure"]
impl crate::Readable for PWR_BUCK_CTL2 {}
#[doc = "`write(|w| ..)` method takes [pwr_buck_ctl2::W](pwr_buck_ctl2::W) writer structure"]
impl crate::Writable for PWR_BUCK_CTL2 {}
#[doc = "Buck Control Register 2"]
pub mod pwr_buck_ctl2;
#[doc = "Low Voltage Detector (LVD) Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_lvd_status](pwr_lvd_status) module"]
pub type PWR_LVD_STATUS = crate::Reg<u32, _PWR_LVD_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_LVD_STATUS;
#[doc = "`read()` method returns [pwr_lvd_status::R](pwr_lvd_status::R) reader structure"]
impl crate::Readable for PWR_LVD_STATUS {}
#[doc = "Low Voltage Detector (LVD) Status Register"]
pub mod pwr_lvd_status;
#[doc = "HIBERNATE Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_hib_data](pwr_hib_data) module"]
pub type PWR_HIB_DATA = crate::Reg<u32, _PWR_HIB_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_HIB_DATA;
#[doc = "`read()` method returns [pwr_hib_data::R](pwr_hib_data::R) reader structure"]
impl crate::Readable for PWR_HIB_DATA {}
#[doc = "`write(|w| ..)` method takes [pwr_hib_data::W](pwr_hib_data::W) writer structure"]
impl crate::Writable for PWR_HIB_DATA {}
#[doc = "HIBERNATE Data Register"]
pub mod pwr_hib_data;
#[doc = "Watchdog Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_ctl](wdt_ctl) module"]
pub type WDT_CTL = crate::Reg<u32, _WDT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_CTL;
#[doc = "`read()` method returns [wdt_ctl::R](wdt_ctl::R) reader structure"]
impl crate::Readable for WDT_CTL {}
#[doc = "`write(|w| ..)` method takes [wdt_ctl::W](wdt_ctl::W) writer structure"]
impl crate::Writable for WDT_CTL {}
#[doc = "Watchdog Counter Control Register"]
pub mod wdt_ctl;
#[doc = "Watchdog Counter Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_cnt](wdt_cnt) module"]
pub type WDT_CNT = crate::Reg<u32, _WDT_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_CNT;
#[doc = "`read()` method returns [wdt_cnt::R](wdt_cnt::R) reader structure"]
impl crate::Readable for WDT_CNT {}
#[doc = "`write(|w| ..)` method takes [wdt_cnt::W](wdt_cnt::W) writer structure"]
impl crate::Writable for WDT_CNT {}
#[doc = "Watchdog Counter Count Register"]
pub mod wdt_cnt;
#[doc = "Watchdog Counter Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_match](wdt_match) module"]
pub type WDT_MATCH = crate::Reg<u32, _WDT_MATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_MATCH;
#[doc = "`read()` method returns [wdt_match::R](wdt_match::R) reader structure"]
impl crate::Readable for WDT_MATCH {}
#[doc = "`write(|w| ..)` method takes [wdt_match::W](wdt_match::W) writer structure"]
impl crate::Writable for WDT_MATCH {}
#[doc = "Watchdog Counter Match Register"]
pub mod wdt_match;
#[doc = "Clock DSI Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_dsi_select](clk_dsi_select) module"]
pub type CLK_DSI_SELECT = crate::Reg<u32, _CLK_DSI_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_DSI_SELECT;
#[doc = "`read()` method returns [clk_dsi_select::R](clk_dsi_select::R) reader structure"]
impl crate::Readable for CLK_DSI_SELECT {}
#[doc = "`write(|w| ..)` method takes [clk_dsi_select::W](clk_dsi_select::W) writer structure"]
impl crate::Writable for CLK_DSI_SELECT {}
#[doc = "Clock DSI Select Register"]
pub mod clk_dsi_select;
#[doc = "Clock Path Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_path_select](clk_path_select) module"]
pub type CLK_PATH_SELECT = crate::Reg<u32, _CLK_PATH_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_PATH_SELECT;
#[doc = "`read()` method returns [clk_path_select::R](clk_path_select::R) reader structure"]
impl crate::Readable for CLK_PATH_SELECT {}
#[doc = "`write(|w| ..)` method takes [clk_path_select::W](clk_path_select::W) writer structure"]
impl crate::Writable for CLK_PATH_SELECT {}
#[doc = "Clock Path Select Register"]
pub mod clk_path_select;
#[doc = "Clock Root Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_root_select](clk_root_select) module"]
pub type CLK_ROOT_SELECT = crate::Reg<u32, _CLK_ROOT_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_ROOT_SELECT;
#[doc = "`read()` method returns [clk_root_select::R](clk_root_select::R) reader structure"]
impl crate::Readable for CLK_ROOT_SELECT {}
#[doc = "`write(|w| ..)` method takes [clk_root_select::W](clk_root_select::W) writer structure"]
impl crate::Writable for CLK_ROOT_SELECT {}
#[doc = "Clock Root Select Register"]
pub mod clk_root_select;
#[doc = "Clock selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_select](clk_select) module"]
pub type CLK_SELECT = crate::Reg<u32, _CLK_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SELECT;
#[doc = "`read()` method returns [clk_select::R](clk_select::R) reader structure"]
impl crate::Readable for CLK_SELECT {}
#[doc = "`write(|w| ..)` method takes [clk_select::W](clk_select::W) writer structure"]
impl crate::Writable for CLK_SELECT {}
#[doc = "Clock selection register"]
pub mod clk_select;
#[doc = "Timer Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_timer_ctl](clk_timer_ctl) module"]
pub type CLK_TIMER_CTL = crate::Reg<u32, _CLK_TIMER_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TIMER_CTL;
#[doc = "`read()` method returns [clk_timer_ctl::R](clk_timer_ctl::R) reader structure"]
impl crate::Readable for CLK_TIMER_CTL {}
#[doc = "`write(|w| ..)` method takes [clk_timer_ctl::W](clk_timer_ctl::W) writer structure"]
impl crate::Writable for CLK_TIMER_CTL {}
#[doc = "Timer Clock Control Register"]
pub mod clk_timer_ctl;
#[doc = "ILO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ilo_config](clk_ilo_config) module"]
pub type CLK_ILO_CONFIG = crate::Reg<u32, _CLK_ILO_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_ILO_CONFIG;
#[doc = "`read()` method returns [clk_ilo_config::R](clk_ilo_config::R) reader structure"]
impl crate::Readable for CLK_ILO_CONFIG {}
#[doc = "`write(|w| ..)` method takes [clk_ilo_config::W](clk_ilo_config::W) writer structure"]
impl crate::Writable for CLK_ILO_CONFIG {}
#[doc = "ILO Configuration"]
pub mod clk_ilo_config;
#[doc = "IMO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_imo_config](clk_imo_config) module"]
pub type CLK_IMO_CONFIG = crate::Reg<u32, _CLK_IMO_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_IMO_CONFIG;
#[doc = "`read()` method returns [clk_imo_config::R](clk_imo_config::R) reader structure"]
impl crate::Readable for CLK_IMO_CONFIG {}
#[doc = "`write(|w| ..)` method takes [clk_imo_config::W](clk_imo_config::W) writer structure"]
impl crate::Writable for CLK_IMO_CONFIG {}
#[doc = "IMO Configuration"]
pub mod clk_imo_config;
#[doc = "Fast Clock Output Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_output_fast](clk_output_fast) module"]
pub type CLK_OUTPUT_FAST = crate::Reg<u32, _CLK_OUTPUT_FAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_OUTPUT_FAST;
#[doc = "`read()` method returns [clk_output_fast::R](clk_output_fast::R) reader structure"]
impl crate::Readable for CLK_OUTPUT_FAST {}
#[doc = "`write(|w| ..)` method takes [clk_output_fast::W](clk_output_fast::W) writer structure"]
impl crate::Writable for CLK_OUTPUT_FAST {}
#[doc = "Fast Clock Output Select Register"]
pub mod clk_output_fast;
#[doc = "Slow Clock Output Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_output_slow](clk_output_slow) module"]
pub type CLK_OUTPUT_SLOW = crate::Reg<u32, _CLK_OUTPUT_SLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_OUTPUT_SLOW;
#[doc = "`read()` method returns [clk_output_slow::R](clk_output_slow::R) reader structure"]
impl crate::Readable for CLK_OUTPUT_SLOW {}
#[doc = "`write(|w| ..)` method takes [clk_output_slow::W](clk_output_slow::W) writer structure"]
impl crate::Writable for CLK_OUTPUT_SLOW {}
#[doc = "Slow Clock Output Select Register"]
pub mod clk_output_slow;
#[doc = "Clock Calibration Counter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cal_cnt1](clk_cal_cnt1) module"]
pub type CLK_CAL_CNT1 = crate::Reg<u32, _CLK_CAL_CNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CAL_CNT1;
#[doc = "`read()` method returns [clk_cal_cnt1::R](clk_cal_cnt1::R) reader structure"]
impl crate::Readable for CLK_CAL_CNT1 {}
#[doc = "`write(|w| ..)` method takes [clk_cal_cnt1::W](clk_cal_cnt1::W) writer structure"]
impl crate::Writable for CLK_CAL_CNT1 {}
#[doc = "Clock Calibration Counter 1"]
pub mod clk_cal_cnt1;
#[doc = "Clock Calibration Counter 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cal_cnt2](clk_cal_cnt2) module"]
pub type CLK_CAL_CNT2 = crate::Reg<u32, _CLK_CAL_CNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CAL_CNT2;
#[doc = "`read()` method returns [clk_cal_cnt2::R](clk_cal_cnt2::R) reader structure"]
impl crate::Readable for CLK_CAL_CNT2 {}
#[doc = "Clock Calibration Counter 2"]
pub mod clk_cal_cnt2;
#[doc = "ECO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_eco_config](clk_eco_config) module"]
pub type CLK_ECO_CONFIG = crate::Reg<u32, _CLK_ECO_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_ECO_CONFIG;
#[doc = "`read()` method returns [clk_eco_config::R](clk_eco_config::R) reader structure"]
impl crate::Readable for CLK_ECO_CONFIG {}
#[doc = "`write(|w| ..)` method takes [clk_eco_config::W](clk_eco_config::W) writer structure"]
impl crate::Writable for CLK_ECO_CONFIG {}
#[doc = "ECO Configuration Register"]
pub mod clk_eco_config;
#[doc = "ECO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_eco_status](clk_eco_status) module"]
pub type CLK_ECO_STATUS = crate::Reg<u32, _CLK_ECO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_ECO_STATUS;
#[doc = "`read()` method returns [clk_eco_status::R](clk_eco_status::R) reader structure"]
impl crate::Readable for CLK_ECO_STATUS {}
#[doc = "ECO Status Register"]
pub mod clk_eco_status;
#[doc = "Precision ILO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pilo_config](clk_pilo_config) module"]
pub type CLK_PILO_CONFIG = crate::Reg<u32, _CLK_PILO_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_PILO_CONFIG;
#[doc = "`read()` method returns [clk_pilo_config::R](clk_pilo_config::R) reader structure"]
impl crate::Readable for CLK_PILO_CONFIG {}
#[doc = "`write(|w| ..)` method takes [clk_pilo_config::W](clk_pilo_config::W) writer structure"]
impl crate::Writable for CLK_PILO_CONFIG {}
#[doc = "Precision ILO Configuration Register"]
pub mod clk_pilo_config;
#[doc = "FLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_fll_config](clk_fll_config) module"]
pub type CLK_FLL_CONFIG = crate::Reg<u32, _CLK_FLL_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_FLL_CONFIG;
#[doc = "`read()` method returns [clk_fll_config::R](clk_fll_config::R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG {}
#[doc = "`write(|w| ..)` method takes [clk_fll_config::W](clk_fll_config::W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG {}
#[doc = "FLL Configuration Register"]
pub mod clk_fll_config;
#[doc = "FLL Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_fll_config2](clk_fll_config2) module"]
pub type CLK_FLL_CONFIG2 = crate::Reg<u32, _CLK_FLL_CONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_FLL_CONFIG2;
#[doc = "`read()` method returns [clk_fll_config2::R](clk_fll_config2::R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG2 {}
#[doc = "`write(|w| ..)` method takes [clk_fll_config2::W](clk_fll_config2::W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG2 {}
#[doc = "FLL Configuration Register 2"]
pub mod clk_fll_config2;
#[doc = "FLL Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_fll_config3](clk_fll_config3) module"]
pub type CLK_FLL_CONFIG3 = crate::Reg<u32, _CLK_FLL_CONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_FLL_CONFIG3;
#[doc = "`read()` method returns [clk_fll_config3::R](clk_fll_config3::R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG3 {}
#[doc = "`write(|w| ..)` method takes [clk_fll_config3::W](clk_fll_config3::W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG3 {}
#[doc = "FLL Configuration Register 3"]
pub mod clk_fll_config3;
#[doc = "FLL Configuration Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_fll_config4](clk_fll_config4) module"]
pub type CLK_FLL_CONFIG4 = crate::Reg<u32, _CLK_FLL_CONFIG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_FLL_CONFIG4;
#[doc = "`read()` method returns [clk_fll_config4::R](clk_fll_config4::R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG4 {}
#[doc = "`write(|w| ..)` method takes [clk_fll_config4::W](clk_fll_config4::W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG4 {}
#[doc = "FLL Configuration Register 4"]
pub mod clk_fll_config4;
#[doc = "FLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_fll_status](clk_fll_status) module"]
pub type CLK_FLL_STATUS = crate::Reg<u32, _CLK_FLL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_FLL_STATUS;
#[doc = "`read()` method returns [clk_fll_status::R](clk_fll_status::R) reader structure"]
impl crate::Readable for CLK_FLL_STATUS {}
#[doc = "`write(|w| ..)` method takes [clk_fll_status::W](clk_fll_status::W) writer structure"]
impl crate::Writable for CLK_FLL_STATUS {}
#[doc = "FLL Status Register"]
pub mod clk_fll_status;
#[doc = "PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pll_config](clk_pll_config) module"]
pub type CLK_PLL_CONFIG = crate::Reg<u32, _CLK_PLL_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_PLL_CONFIG;
#[doc = "`read()` method returns [clk_pll_config::R](clk_pll_config::R) reader structure"]
impl crate::Readable for CLK_PLL_CONFIG {}
#[doc = "`write(|w| ..)` method takes [clk_pll_config::W](clk_pll_config::W) writer structure"]
impl crate::Writable for CLK_PLL_CONFIG {}
#[doc = "PLL Configuration Register"]
pub mod clk_pll_config;
#[doc = "PLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pll_status](clk_pll_status) module"]
pub type CLK_PLL_STATUS = crate::Reg<u32, _CLK_PLL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_PLL_STATUS;
#[doc = "`read()` method returns [clk_pll_status::R](clk_pll_status::R) reader structure"]
impl crate::Readable for CLK_PLL_STATUS {}
#[doc = "`write(|w| ..)` method takes [clk_pll_status::W](clk_pll_status::W) writer structure"]
impl crate::Writable for CLK_PLL_STATUS {}
#[doc = "PLL Status Register"]
pub mod clk_pll_status;
#[doc = "SRSS Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr](srss_intr) module"]
pub type SRSS_INTR = crate::Reg<u32, _SRSS_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSS_INTR;
#[doc = "`read()` method returns [srss_intr::R](srss_intr::R) reader structure"]
impl crate::Readable for SRSS_INTR {}
#[doc = "`write(|w| ..)` method takes [srss_intr::W](srss_intr::W) writer structure"]
impl crate::Writable for SRSS_INTR {}
#[doc = "SRSS Interrupt Register"]
pub mod srss_intr;
#[doc = "SRSS Interrupt Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr_set](srss_intr_set) module"]
pub type SRSS_INTR_SET = crate::Reg<u32, _SRSS_INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSS_INTR_SET;
#[doc = "`read()` method returns [srss_intr_set::R](srss_intr_set::R) reader structure"]
impl crate::Readable for SRSS_INTR_SET {}
#[doc = "`write(|w| ..)` method takes [srss_intr_set::W](srss_intr_set::W) writer structure"]
impl crate::Writable for SRSS_INTR_SET {}
#[doc = "SRSS Interrupt Set Register"]
pub mod srss_intr_set;
#[doc = "SRSS Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr_mask](srss_intr_mask) module"]
pub type SRSS_INTR_MASK = crate::Reg<u32, _SRSS_INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSS_INTR_MASK;
#[doc = "`read()` method returns [srss_intr_mask::R](srss_intr_mask::R) reader structure"]
impl crate::Readable for SRSS_INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [srss_intr_mask::W](srss_intr_mask::W) writer structure"]
impl crate::Writable for SRSS_INTR_MASK {}
#[doc = "SRSS Interrupt Mask Register"]
pub mod srss_intr_mask;
#[doc = "SRSS Interrupt Masked Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr_masked](srss_intr_masked) module"]
pub type SRSS_INTR_MASKED = crate::Reg<u32, _SRSS_INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSS_INTR_MASKED;
#[doc = "`read()` method returns [srss_intr_masked::R](srss_intr_masked::R) reader structure"]
impl crate::Readable for SRSS_INTR_MASKED {}
#[doc = "SRSS Interrupt Masked Register"]
pub mod srss_intr_masked;
#[doc = "SRSS Interrupt Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr_cfg](srss_intr_cfg) module"]
pub type SRSS_INTR_CFG = crate::Reg<u32, _SRSS_INTR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSS_INTR_CFG;
#[doc = "`read()` method returns [srss_intr_cfg::R](srss_intr_cfg::R) reader structure"]
impl crate::Readable for SRSS_INTR_CFG {}
#[doc = "`write(|w| ..)` method takes [srss_intr_cfg::W](srss_intr_cfg::W) writer structure"]
impl crate::Writable for SRSS_INTR_CFG {}
#[doc = "SRSS Interrupt Configuration Register"]
pub mod srss_intr_cfg;
#[doc = "Reset Cause Observation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_cause](res_cause) module"]
pub type RES_CAUSE = crate::Reg<u32, _RES_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES_CAUSE;
#[doc = "`read()` method returns [res_cause::R](res_cause::R) reader structure"]
impl crate::Readable for RES_CAUSE {}
#[doc = "`write(|w| ..)` method takes [res_cause::W](res_cause::W) writer structure"]
impl crate::Writable for RES_CAUSE {}
#[doc = "Reset Cause Observation Register"]
pub mod res_cause;
#[doc = "Reset Cause Observation Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_cause2](res_cause2) module"]
pub type RES_CAUSE2 = crate::Reg<u32, _RES_CAUSE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES_CAUSE2;
#[doc = "`read()` method returns [res_cause2::R](res_cause2::R) reader structure"]
impl crate::Readable for RES_CAUSE2 {}
#[doc = "`write(|w| ..)` method takes [res_cause2::W](res_cause2::W) writer structure"]
impl crate::Writable for RES_CAUSE2 {}
#[doc = "Reset Cause Observation Register 2"]
pub mod res_cause2;
#[doc = "Reference Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_ref_ctl](pwr_trim_ref_ctl) module"]
pub type PWR_TRIM_REF_CTL = crate::Reg<u32, _PWR_TRIM_REF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_TRIM_REF_CTL;
#[doc = "`read()` method returns [pwr_trim_ref_ctl::R](pwr_trim_ref_ctl::R) reader structure"]
impl crate::Readable for PWR_TRIM_REF_CTL {}
#[doc = "`write(|w| ..)` method takes [pwr_trim_ref_ctl::W](pwr_trim_ref_ctl::W) writer structure"]
impl crate::Writable for PWR_TRIM_REF_CTL {}
#[doc = "Reference Trim Register"]
pub mod pwr_trim_ref_ctl;
#[doc = "BOD/OVP Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_bodovp_ctl](pwr_trim_bodovp_ctl) module"]
pub type PWR_TRIM_BODOVP_CTL = crate::Reg<u32, _PWR_TRIM_BODOVP_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_TRIM_BODOVP_CTL;
#[doc = "`read()` method returns [pwr_trim_bodovp_ctl::R](pwr_trim_bodovp_ctl::R) reader structure"]
impl crate::Readable for PWR_TRIM_BODOVP_CTL {}
#[doc = "`write(|w| ..)` method takes [pwr_trim_bodovp_ctl::W](pwr_trim_bodovp_ctl::W) writer structure"]
impl crate::Writable for PWR_TRIM_BODOVP_CTL {}
#[doc = "BOD/OVP Trim Register"]
pub mod pwr_trim_bodovp_ctl;
#[doc = "CCO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_cco_ctl](clk_trim_cco_ctl) module"]
pub type CLK_TRIM_CCO_CTL = crate::Reg<u32, _CLK_TRIM_CCO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TRIM_CCO_CTL;
#[doc = "`read()` method returns [clk_trim_cco_ctl::R](clk_trim_cco_ctl::R) reader structure"]
impl crate::Readable for CLK_TRIM_CCO_CTL {}
#[doc = "`write(|w| ..)` method takes [clk_trim_cco_ctl::W](clk_trim_cco_ctl::W) writer structure"]
impl crate::Writable for CLK_TRIM_CCO_CTL {}
#[doc = "CCO Trim Register"]
pub mod clk_trim_cco_ctl;
#[doc = "CCO Trim Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_cco_ctl2](clk_trim_cco_ctl2) module"]
pub type CLK_TRIM_CCO_CTL2 = crate::Reg<u32, _CLK_TRIM_CCO_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TRIM_CCO_CTL2;
#[doc = "`read()` method returns [clk_trim_cco_ctl2::R](clk_trim_cco_ctl2::R) reader structure"]
impl crate::Readable for CLK_TRIM_CCO_CTL2 {}
#[doc = "`write(|w| ..)` method takes [clk_trim_cco_ctl2::W](clk_trim_cco_ctl2::W) writer structure"]
impl crate::Writable for CLK_TRIM_CCO_CTL2 {}
#[doc = "CCO Trim Register 2"]
pub mod clk_trim_cco_ctl2;
#[doc = "Wakeup Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_wake_ctl](pwr_trim_wake_ctl) module"]
pub type PWR_TRIM_WAKE_CTL = crate::Reg<u32, _PWR_TRIM_WAKE_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_TRIM_WAKE_CTL;
#[doc = "`read()` method returns [pwr_trim_wake_ctl::R](pwr_trim_wake_ctl::R) reader structure"]
impl crate::Readable for PWR_TRIM_WAKE_CTL {}
#[doc = "`write(|w| ..)` method takes [pwr_trim_wake_ctl::W](pwr_trim_wake_ctl::W) writer structure"]
impl crate::Writable for PWR_TRIM_WAKE_CTL {}
#[doc = "Wakeup Trim Register"]
pub mod pwr_trim_wake_ctl;
#[doc = "LVD Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_lvd_ctl](pwr_trim_lvd_ctl) module"]
pub type PWR_TRIM_LVD_CTL = crate::Reg<u32, _PWR_TRIM_LVD_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_TRIM_LVD_CTL;
#[doc = "`read()` method returns [pwr_trim_lvd_ctl::R](pwr_trim_lvd_ctl::R) reader structure"]
impl crate::Readable for PWR_TRIM_LVD_CTL {}
#[doc = "`write(|w| ..)` method takes [pwr_trim_lvd_ctl::W](pwr_trim_lvd_ctl::W) writer structure"]
impl crate::Writable for PWR_TRIM_LVD_CTL {}
#[doc = "LVD Trim Register"]
pub mod pwr_trim_lvd_ctl;
#[doc = "ILO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_ilo_ctl](clk_trim_ilo_ctl) module"]
pub type CLK_TRIM_ILO_CTL = crate::Reg<u32, _CLK_TRIM_ILO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TRIM_ILO_CTL;
#[doc = "`read()` method returns [clk_trim_ilo_ctl::R](clk_trim_ilo_ctl::R) reader structure"]
impl crate::Readable for CLK_TRIM_ILO_CTL {}
#[doc = "`write(|w| ..)` method takes [clk_trim_ilo_ctl::W](clk_trim_ilo_ctl::W) writer structure"]
impl crate::Writable for CLK_TRIM_ILO_CTL {}
#[doc = "ILO Trim Register"]
pub mod clk_trim_ilo_ctl;
#[doc = "Power System Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_pwrsys_ctl](pwr_trim_pwrsys_ctl) module"]
pub type PWR_TRIM_PWRSYS_CTL = crate::Reg<u32, _PWR_TRIM_PWRSYS_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_TRIM_PWRSYS_CTL;
#[doc = "`read()` method returns [pwr_trim_pwrsys_ctl::R](pwr_trim_pwrsys_ctl::R) reader structure"]
impl crate::Readable for PWR_TRIM_PWRSYS_CTL {}
#[doc = "`write(|w| ..)` method takes [pwr_trim_pwrsys_ctl::W](pwr_trim_pwrsys_ctl::W) writer structure"]
impl crate::Writable for PWR_TRIM_PWRSYS_CTL {}
#[doc = "Power System Trim Register"]
pub mod pwr_trim_pwrsys_ctl;
#[doc = "ECO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_eco_ctl](clk_trim_eco_ctl) module"]
pub type CLK_TRIM_ECO_CTL = crate::Reg<u32, _CLK_TRIM_ECO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TRIM_ECO_CTL;
#[doc = "`read()` method returns [clk_trim_eco_ctl::R](clk_trim_eco_ctl::R) reader structure"]
impl crate::Readable for CLK_TRIM_ECO_CTL {}
#[doc = "`write(|w| ..)` method takes [clk_trim_eco_ctl::W](clk_trim_eco_ctl::W) writer structure"]
impl crate::Writable for CLK_TRIM_ECO_CTL {}
#[doc = "ECO Trim Register"]
pub mod clk_trim_eco_ctl;
#[doc = "PILO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_pilo_ctl](clk_trim_pilo_ctl) module"]
pub type CLK_TRIM_PILO_CTL = crate::Reg<u32, _CLK_TRIM_PILO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TRIM_PILO_CTL;
#[doc = "`read()` method returns [clk_trim_pilo_ctl::R](clk_trim_pilo_ctl::R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL {}
#[doc = "`write(|w| ..)` method takes [clk_trim_pilo_ctl::W](clk_trim_pilo_ctl::W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL {}
#[doc = "PILO Trim Register"]
pub mod clk_trim_pilo_ctl;
#[doc = "PILO Trim Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_pilo_ctl2](clk_trim_pilo_ctl2) module"]
pub type CLK_TRIM_PILO_CTL2 = crate::Reg<u32, _CLK_TRIM_PILO_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TRIM_PILO_CTL2;
#[doc = "`read()` method returns [clk_trim_pilo_ctl2::R](clk_trim_pilo_ctl2::R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL2 {}
#[doc = "`write(|w| ..)` method takes [clk_trim_pilo_ctl2::W](clk_trim_pilo_ctl2::W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL2 {}
#[doc = "PILO Trim Register 2"]
pub mod clk_trim_pilo_ctl2;
#[doc = "PILO Trim Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_pilo_ctl3](clk_trim_pilo_ctl3) module"]
pub type CLK_TRIM_PILO_CTL3 = crate::Reg<u32, _CLK_TRIM_PILO_CTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TRIM_PILO_CTL3;
#[doc = "`read()` method returns [clk_trim_pilo_ctl3::R](clk_trim_pilo_ctl3::R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL3 {}
#[doc = "`write(|w| ..)` method takes [clk_trim_pilo_ctl3::W](clk_trim_pilo_ctl3::W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL3 {}
#[doc = "PILO Trim Register 3"]
pub mod clk_trim_pilo_ctl3;

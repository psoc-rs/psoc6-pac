#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Mode Control"]
    pub pwr_ctl: crate::Reg<pwr_ctl::PWR_CTL_SPEC>,
    #[doc = "0x04 - HIBERNATE Mode Register"]
    pub pwr_hibernate: crate::Reg<pwr_hibernate::PWR_HIBERNATE_SPEC>,
    #[doc = "0x08 - Low Voltage Detector (LVD) Configuration Register"]
    pub pwr_lvd_ctl: crate::Reg<pwr_lvd_ctl::PWR_LVD_CTL_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x14 - Buck Control Register"]
    pub pwr_buck_ctl: crate::Reg<pwr_buck_ctl::PWR_BUCK_CTL_SPEC>,
    #[doc = "0x18 - Buck Control Register 2"]
    pub pwr_buck_ctl2: crate::Reg<pwr_buck_ctl2::PWR_BUCK_CTL2_SPEC>,
    #[doc = "0x1c - Low Voltage Detector (LVD) Status Register"]
    pub pwr_lvd_status: crate::Reg<pwr_lvd_status::PWR_LVD_STATUS_SPEC>,
    _reserved6: [u8; 0x60],
    #[doc = "0x80..0xc0 - HIBERNATE Data Register"]
    pub pwr_hib_data: [crate::Reg<pwr_hib_data::PWR_HIB_DATA_SPEC>; 16],
    _reserved7: [u8; 0xc0],
    #[doc = "0x180 - Watchdog Counter Control Register"]
    pub wdt_ctl: crate::Reg<wdt_ctl::WDT_CTL_SPEC>,
    #[doc = "0x184 - Watchdog Counter Count Register"]
    pub wdt_cnt: crate::Reg<wdt_cnt::WDT_CNT_SPEC>,
    #[doc = "0x188 - Watchdog Counter Match Register"]
    pub wdt_match: crate::Reg<wdt_match::WDT_MATCH_SPEC>,
    _reserved10: [u8; 0x74],
    #[doc = "0x200..0x22c - Multi-Counter Watchdog Timer"]
    pub mcwdt_struct0: MCWDT_STRUCT,
    _reserved11: [u8; 0x14],
    #[doc = "0x240..0x26c - Multi-Counter Watchdog Timer"]
    pub mcwdt_struct1: MCWDT_STRUCT,
    _reserved12: [u8; 0x94],
    #[doc = "0x300..0x340 - Clock DSI Select Register"]
    pub clk_dsi_select: [crate::Reg<clk_dsi_select::CLK_DSI_SELECT_SPEC>; 16],
    #[doc = "0x340..0x380 - Clock Path Select Register"]
    pub clk_path_select: [crate::Reg<clk_path_select::CLK_PATH_SELECT_SPEC>; 16],
    #[doc = "0x380..0x3c0 - Clock Root Select Register"]
    pub clk_root_select: [crate::Reg<clk_root_select::CLK_ROOT_SELECT_SPEC>; 16],
    _reserved15: [u8; 0x0140],
    #[doc = "0x500 - Clock selection register"]
    pub clk_select: crate::Reg<clk_select::CLK_SELECT_SPEC>,
    #[doc = "0x504 - Timer Clock Control Register"]
    pub clk_timer_ctl: crate::Reg<clk_timer_ctl::CLK_TIMER_CTL_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x50c - ILO Configuration"]
    pub clk_ilo_config: crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>,
    #[doc = "0x510 - IMO Configuration"]
    pub clk_imo_config: crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>,
    #[doc = "0x514 - Fast Clock Output Select Register"]
    pub clk_output_fast: crate::Reg<clk_output_fast::CLK_OUTPUT_FAST_SPEC>,
    #[doc = "0x518 - Slow Clock Output Select Register"]
    pub clk_output_slow: crate::Reg<clk_output_slow::CLK_OUTPUT_SLOW_SPEC>,
    #[doc = "0x51c - Clock Calibration Counter 1"]
    pub clk_cal_cnt1: crate::Reg<clk_cal_cnt1::CLK_CAL_CNT1_SPEC>,
    #[doc = "0x520 - Clock Calibration Counter 2"]
    pub clk_cal_cnt2: crate::Reg<clk_cal_cnt2::CLK_CAL_CNT2_SPEC>,
    _reserved23: [u8; 0x08],
    #[doc = "0x52c - ECO Configuration Register"]
    pub clk_eco_config: crate::Reg<clk_eco_config::CLK_ECO_CONFIG_SPEC>,
    #[doc = "0x530 - ECO Status Register"]
    pub clk_eco_status: crate::Reg<clk_eco_status::CLK_ECO_STATUS_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x53c - Precision ILO Configuration Register"]
    pub clk_pilo_config: crate::Reg<clk_pilo_config::CLK_PILO_CONFIG_SPEC>,
    _reserved26: [u8; 0x40],
    #[doc = "0x580 - FLL Configuration Register"]
    pub clk_fll_config: crate::Reg<clk_fll_config::CLK_FLL_CONFIG_SPEC>,
    #[doc = "0x584 - FLL Configuration Register 2"]
    pub clk_fll_config2: crate::Reg<clk_fll_config2::CLK_FLL_CONFIG2_SPEC>,
    #[doc = "0x588 - FLL Configuration Register 3"]
    pub clk_fll_config3: crate::Reg<clk_fll_config3::CLK_FLL_CONFIG3_SPEC>,
    #[doc = "0x58c - FLL Configuration Register 4"]
    pub clk_fll_config4: crate::Reg<clk_fll_config4::CLK_FLL_CONFIG4_SPEC>,
    #[doc = "0x590 - FLL Status Register"]
    pub clk_fll_status: crate::Reg<clk_fll_status::CLK_FLL_STATUS_SPEC>,
    _reserved31: [u8; 0x6c],
    #[doc = "0x600..0x63c - PLL Configuration Register"]
    pub clk_pll_config: [crate::Reg<clk_pll_config::CLK_PLL_CONFIG_SPEC>; 15],
    _reserved32: [u8; 0x04],
    #[doc = "0x640..0x67c - PLL Status Register"]
    pub clk_pll_status: [crate::Reg<clk_pll_status::CLK_PLL_STATUS_SPEC>; 15],
    _reserved33: [u8; 0x84],
    #[doc = "0x700 - SRSS Interrupt Register"]
    pub srss_intr: crate::Reg<srss_intr::SRSS_INTR_SPEC>,
    #[doc = "0x704 - SRSS Interrupt Set Register"]
    pub srss_intr_set: crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>,
    #[doc = "0x708 - SRSS Interrupt Mask Register"]
    pub srss_intr_mask: crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>,
    #[doc = "0x70c - SRSS Interrupt Masked Register"]
    pub srss_intr_masked: crate::Reg<srss_intr_masked::SRSS_INTR_MASKED_SPEC>,
    #[doc = "0x710 - SRSS Interrupt Configuration Register"]
    pub srss_intr_cfg: crate::Reg<srss_intr_cfg::SRSS_INTR_CFG_SPEC>,
    _reserved38: [u8; 0xec],
    #[doc = "0x800 - Reset Cause Observation Register"]
    pub res_cause: crate::Reg<res_cause::RES_CAUSE_SPEC>,
    #[doc = "0x804 - Reset Cause Observation Register 2"]
    pub res_cause2: crate::Reg<res_cause2::RES_CAUSE2_SPEC>,
    _reserved40: [u8; 0x76f8],
    #[doc = "0x7f00 - Reference Trim Register"]
    pub pwr_trim_ref_ctl: crate::Reg<pwr_trim_ref_ctl::PWR_TRIM_REF_CTL_SPEC>,
    #[doc = "0x7f04 - BOD/OVP Trim Register"]
    pub pwr_trim_bodovp_ctl: crate::Reg<pwr_trim_bodovp_ctl::PWR_TRIM_BODOVP_CTL_SPEC>,
    #[doc = "0x7f08 - CCO Trim Register"]
    pub clk_trim_cco_ctl: crate::Reg<clk_trim_cco_ctl::CLK_TRIM_CCO_CTL_SPEC>,
    #[doc = "0x7f0c - CCO Trim Register 2"]
    pub clk_trim_cco_ctl2: crate::Reg<clk_trim_cco_ctl2::CLK_TRIM_CCO_CTL2_SPEC>,
    _reserved44: [u8; 0x20],
    #[doc = "0x7f30 - Wakeup Trim Register"]
    pub pwr_trim_wake_ctl: crate::Reg<pwr_trim_wake_ctl::PWR_TRIM_WAKE_CTL_SPEC>,
    _reserved45: [u8; 0x7fdc],
    #[doc = "0xff10 - LVD Trim Register"]
    pub pwr_trim_lvd_ctl: crate::Reg<pwr_trim_lvd_ctl::PWR_TRIM_LVD_CTL_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0xff18 - ILO Trim Register"]
    pub clk_trim_ilo_ctl: crate::Reg<clk_trim_ilo_ctl::CLK_TRIM_ILO_CTL_SPEC>,
    #[doc = "0xff1c - Power System Trim Register"]
    pub pwr_trim_pwrsys_ctl: crate::Reg<pwr_trim_pwrsys_ctl::PWR_TRIM_PWRSYS_CTL_SPEC>,
    #[doc = "0xff20 - ECO Trim Register"]
    pub clk_trim_eco_ctl: crate::Reg<clk_trim_eco_ctl::CLK_TRIM_ECO_CTL_SPEC>,
    #[doc = "0xff24 - PILO Trim Register"]
    pub clk_trim_pilo_ctl: crate::Reg<clk_trim_pilo_ctl::CLK_TRIM_PILO_CTL_SPEC>,
    #[doc = "0xff28 - PILO Trim Register 2"]
    pub clk_trim_pilo_ctl2: crate::Reg<clk_trim_pilo_ctl2::CLK_TRIM_PILO_CTL2_SPEC>,
    #[doc = "0xff2c - PILO Trim Register 3"]
    pub clk_trim_pilo_ctl3: crate::Reg<clk_trim_pilo_ctl3::CLK_TRIM_PILO_CTL3_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MCWDT_STRUCT {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Multi-Counter Watchdog Sub-counters 0/1"]
    pub mcwdt_cntlow: crate::Reg<self::mcwdt_struct::mcwdt_cntlow::MCWDT_CNTLOW_SPEC>,
    #[doc = "0x08 - Multi-Counter Watchdog Sub-counter 2"]
    pub mcwdt_cnthigh: crate::Reg<self::mcwdt_struct::mcwdt_cnthigh::MCWDT_CNTHIGH_SPEC>,
    #[doc = "0x0c - Multi-Counter Watchdog Counter Match Register"]
    pub mcwdt_match: crate::Reg<self::mcwdt_struct::mcwdt_match::MCWDT_MATCH_SPEC>,
    #[doc = "0x10 - Multi-Counter Watchdog Counter Configuration"]
    pub mcwdt_config: crate::Reg<self::mcwdt_struct::mcwdt_config::MCWDT_CONFIG_SPEC>,
    #[doc = "0x14 - Multi-Counter Watchdog Counter Control"]
    pub mcwdt_ctl: crate::Reg<self::mcwdt_struct::mcwdt_ctl::MCWDT_CTL_SPEC>,
    #[doc = "0x18 - Multi-Counter Watchdog Counter Interrupt Register"]
    pub mcwdt_intr: crate::Reg<self::mcwdt_struct::mcwdt_intr::MCWDT_INTR_SPEC>,
    #[doc = "0x1c - Multi-Counter Watchdog Counter Interrupt Set Register"]
    pub mcwdt_intr_set: crate::Reg<self::mcwdt_struct::mcwdt_intr_set::MCWDT_INTR_SET_SPEC>,
    #[doc = "0x20 - Multi-Counter Watchdog Counter Interrupt Mask Register"]
    pub mcwdt_intr_mask: crate::Reg<self::mcwdt_struct::mcwdt_intr_mask::MCWDT_INTR_MASK_SPEC>,
    #[doc = "0x24 - Multi-Counter Watchdog Counter Interrupt Masked Register"]
    pub mcwdt_intr_masked:
        crate::Reg<self::mcwdt_struct::mcwdt_intr_masked::MCWDT_INTR_MASKED_SPEC>,
    #[doc = "0x28 - Multi-Counter Watchdog Counter Lock Register"]
    pub mcwdt_lock: crate::Reg<self::mcwdt_struct::mcwdt_lock::MCWDT_LOCK_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Multi-Counter Watchdog Timer"]
pub mod mcwdt_struct;
#[doc = "PWR_CTL register accessor: an alias for `Reg<PWR_CTL_SPEC>`"]
pub type PWR_CTL = crate::Reg<pwr_ctl::PWR_CTL_SPEC>;
#[doc = "Power Mode Control"]
pub mod pwr_ctl;
#[doc = "PWR_HIBERNATE register accessor: an alias for `Reg<PWR_HIBERNATE_SPEC>`"]
pub type PWR_HIBERNATE = crate::Reg<pwr_hibernate::PWR_HIBERNATE_SPEC>;
#[doc = "HIBERNATE Mode Register"]
pub mod pwr_hibernate;
#[doc = "PWR_LVD_CTL register accessor: an alias for `Reg<PWR_LVD_CTL_SPEC>`"]
pub type PWR_LVD_CTL = crate::Reg<pwr_lvd_ctl::PWR_LVD_CTL_SPEC>;
#[doc = "Low Voltage Detector (LVD) Configuration Register"]
pub mod pwr_lvd_ctl;
#[doc = "PWR_BUCK_CTL register accessor: an alias for `Reg<PWR_BUCK_CTL_SPEC>`"]
pub type PWR_BUCK_CTL = crate::Reg<pwr_buck_ctl::PWR_BUCK_CTL_SPEC>;
#[doc = "Buck Control Register"]
pub mod pwr_buck_ctl;
#[doc = "PWR_BUCK_CTL2 register accessor: an alias for `Reg<PWR_BUCK_CTL2_SPEC>`"]
pub type PWR_BUCK_CTL2 = crate::Reg<pwr_buck_ctl2::PWR_BUCK_CTL2_SPEC>;
#[doc = "Buck Control Register 2"]
pub mod pwr_buck_ctl2;
#[doc = "PWR_LVD_STATUS register accessor: an alias for `Reg<PWR_LVD_STATUS_SPEC>`"]
pub type PWR_LVD_STATUS = crate::Reg<pwr_lvd_status::PWR_LVD_STATUS_SPEC>;
#[doc = "Low Voltage Detector (LVD) Status Register"]
pub mod pwr_lvd_status;
#[doc = "PWR_HIB_DATA register accessor: an alias for `Reg<PWR_HIB_DATA_SPEC>`"]
pub type PWR_HIB_DATA = crate::Reg<pwr_hib_data::PWR_HIB_DATA_SPEC>;
#[doc = "HIBERNATE Data Register"]
pub mod pwr_hib_data;
#[doc = "WDT_CTL register accessor: an alias for `Reg<WDT_CTL_SPEC>`"]
pub type WDT_CTL = crate::Reg<wdt_ctl::WDT_CTL_SPEC>;
#[doc = "Watchdog Counter Control Register"]
pub mod wdt_ctl;
#[doc = "WDT_CNT register accessor: an alias for `Reg<WDT_CNT_SPEC>`"]
pub type WDT_CNT = crate::Reg<wdt_cnt::WDT_CNT_SPEC>;
#[doc = "Watchdog Counter Count Register"]
pub mod wdt_cnt;
#[doc = "WDT_MATCH register accessor: an alias for `Reg<WDT_MATCH_SPEC>`"]
pub type WDT_MATCH = crate::Reg<wdt_match::WDT_MATCH_SPEC>;
#[doc = "Watchdog Counter Match Register"]
pub mod wdt_match;
#[doc = "CLK_DSI_SELECT register accessor: an alias for `Reg<CLK_DSI_SELECT_SPEC>`"]
pub type CLK_DSI_SELECT = crate::Reg<clk_dsi_select::CLK_DSI_SELECT_SPEC>;
#[doc = "Clock DSI Select Register"]
pub mod clk_dsi_select;
#[doc = "CLK_PATH_SELECT register accessor: an alias for `Reg<CLK_PATH_SELECT_SPEC>`"]
pub type CLK_PATH_SELECT = crate::Reg<clk_path_select::CLK_PATH_SELECT_SPEC>;
#[doc = "Clock Path Select Register"]
pub mod clk_path_select;
#[doc = "CLK_ROOT_SELECT register accessor: an alias for `Reg<CLK_ROOT_SELECT_SPEC>`"]
pub type CLK_ROOT_SELECT = crate::Reg<clk_root_select::CLK_ROOT_SELECT_SPEC>;
#[doc = "Clock Root Select Register"]
pub mod clk_root_select;
#[doc = "CLK_SELECT register accessor: an alias for `Reg<CLK_SELECT_SPEC>`"]
pub type CLK_SELECT = crate::Reg<clk_select::CLK_SELECT_SPEC>;
#[doc = "Clock selection register"]
pub mod clk_select;
#[doc = "CLK_TIMER_CTL register accessor: an alias for `Reg<CLK_TIMER_CTL_SPEC>`"]
pub type CLK_TIMER_CTL = crate::Reg<clk_timer_ctl::CLK_TIMER_CTL_SPEC>;
#[doc = "Timer Clock Control Register"]
pub mod clk_timer_ctl;
#[doc = "CLK_ILO_CONFIG register accessor: an alias for `Reg<CLK_ILO_CONFIG_SPEC>`"]
pub type CLK_ILO_CONFIG = crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>;
#[doc = "ILO Configuration"]
pub mod clk_ilo_config;
#[doc = "CLK_IMO_CONFIG register accessor: an alias for `Reg<CLK_IMO_CONFIG_SPEC>`"]
pub type CLK_IMO_CONFIG = crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>;
#[doc = "IMO Configuration"]
pub mod clk_imo_config;
#[doc = "CLK_OUTPUT_FAST register accessor: an alias for `Reg<CLK_OUTPUT_FAST_SPEC>`"]
pub type CLK_OUTPUT_FAST = crate::Reg<clk_output_fast::CLK_OUTPUT_FAST_SPEC>;
#[doc = "Fast Clock Output Select Register"]
pub mod clk_output_fast;
#[doc = "CLK_OUTPUT_SLOW register accessor: an alias for `Reg<CLK_OUTPUT_SLOW_SPEC>`"]
pub type CLK_OUTPUT_SLOW = crate::Reg<clk_output_slow::CLK_OUTPUT_SLOW_SPEC>;
#[doc = "Slow Clock Output Select Register"]
pub mod clk_output_slow;
#[doc = "CLK_CAL_CNT1 register accessor: an alias for `Reg<CLK_CAL_CNT1_SPEC>`"]
pub type CLK_CAL_CNT1 = crate::Reg<clk_cal_cnt1::CLK_CAL_CNT1_SPEC>;
#[doc = "Clock Calibration Counter 1"]
pub mod clk_cal_cnt1;
#[doc = "CLK_CAL_CNT2 register accessor: an alias for `Reg<CLK_CAL_CNT2_SPEC>`"]
pub type CLK_CAL_CNT2 = crate::Reg<clk_cal_cnt2::CLK_CAL_CNT2_SPEC>;
#[doc = "Clock Calibration Counter 2"]
pub mod clk_cal_cnt2;
#[doc = "CLK_ECO_CONFIG register accessor: an alias for `Reg<CLK_ECO_CONFIG_SPEC>`"]
pub type CLK_ECO_CONFIG = crate::Reg<clk_eco_config::CLK_ECO_CONFIG_SPEC>;
#[doc = "ECO Configuration Register"]
pub mod clk_eco_config;
#[doc = "CLK_ECO_STATUS register accessor: an alias for `Reg<CLK_ECO_STATUS_SPEC>`"]
pub type CLK_ECO_STATUS = crate::Reg<clk_eco_status::CLK_ECO_STATUS_SPEC>;
#[doc = "ECO Status Register"]
pub mod clk_eco_status;
#[doc = "CLK_PILO_CONFIG register accessor: an alias for `Reg<CLK_PILO_CONFIG_SPEC>`"]
pub type CLK_PILO_CONFIG = crate::Reg<clk_pilo_config::CLK_PILO_CONFIG_SPEC>;
#[doc = "Precision ILO Configuration Register"]
pub mod clk_pilo_config;
#[doc = "CLK_FLL_CONFIG register accessor: an alias for `Reg<CLK_FLL_CONFIG_SPEC>`"]
pub type CLK_FLL_CONFIG = crate::Reg<clk_fll_config::CLK_FLL_CONFIG_SPEC>;
#[doc = "FLL Configuration Register"]
pub mod clk_fll_config;
#[doc = "CLK_FLL_CONFIG2 register accessor: an alias for `Reg<CLK_FLL_CONFIG2_SPEC>`"]
pub type CLK_FLL_CONFIG2 = crate::Reg<clk_fll_config2::CLK_FLL_CONFIG2_SPEC>;
#[doc = "FLL Configuration Register 2"]
pub mod clk_fll_config2;
#[doc = "CLK_FLL_CONFIG3 register accessor: an alias for `Reg<CLK_FLL_CONFIG3_SPEC>`"]
pub type CLK_FLL_CONFIG3 = crate::Reg<clk_fll_config3::CLK_FLL_CONFIG3_SPEC>;
#[doc = "FLL Configuration Register 3"]
pub mod clk_fll_config3;
#[doc = "CLK_FLL_CONFIG4 register accessor: an alias for `Reg<CLK_FLL_CONFIG4_SPEC>`"]
pub type CLK_FLL_CONFIG4 = crate::Reg<clk_fll_config4::CLK_FLL_CONFIG4_SPEC>;
#[doc = "FLL Configuration Register 4"]
pub mod clk_fll_config4;
#[doc = "CLK_FLL_STATUS register accessor: an alias for `Reg<CLK_FLL_STATUS_SPEC>`"]
pub type CLK_FLL_STATUS = crate::Reg<clk_fll_status::CLK_FLL_STATUS_SPEC>;
#[doc = "FLL Status Register"]
pub mod clk_fll_status;
#[doc = "CLK_PLL_CONFIG register accessor: an alias for `Reg<CLK_PLL_CONFIG_SPEC>`"]
pub type CLK_PLL_CONFIG = crate::Reg<clk_pll_config::CLK_PLL_CONFIG_SPEC>;
#[doc = "PLL Configuration Register"]
pub mod clk_pll_config;
#[doc = "CLK_PLL_STATUS register accessor: an alias for `Reg<CLK_PLL_STATUS_SPEC>`"]
pub type CLK_PLL_STATUS = crate::Reg<clk_pll_status::CLK_PLL_STATUS_SPEC>;
#[doc = "PLL Status Register"]
pub mod clk_pll_status;
#[doc = "SRSS_INTR register accessor: an alias for `Reg<SRSS_INTR_SPEC>`"]
pub type SRSS_INTR = crate::Reg<srss_intr::SRSS_INTR_SPEC>;
#[doc = "SRSS Interrupt Register"]
pub mod srss_intr;
#[doc = "SRSS_INTR_SET register accessor: an alias for `Reg<SRSS_INTR_SET_SPEC>`"]
pub type SRSS_INTR_SET = crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>;
#[doc = "SRSS Interrupt Set Register"]
pub mod srss_intr_set;
#[doc = "SRSS_INTR_MASK register accessor: an alias for `Reg<SRSS_INTR_MASK_SPEC>`"]
pub type SRSS_INTR_MASK = crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>;
#[doc = "SRSS Interrupt Mask Register"]
pub mod srss_intr_mask;
#[doc = "SRSS_INTR_MASKED register accessor: an alias for `Reg<SRSS_INTR_MASKED_SPEC>`"]
pub type SRSS_INTR_MASKED = crate::Reg<srss_intr_masked::SRSS_INTR_MASKED_SPEC>;
#[doc = "SRSS Interrupt Masked Register"]
pub mod srss_intr_masked;
#[doc = "SRSS_INTR_CFG register accessor: an alias for `Reg<SRSS_INTR_CFG_SPEC>`"]
pub type SRSS_INTR_CFG = crate::Reg<srss_intr_cfg::SRSS_INTR_CFG_SPEC>;
#[doc = "SRSS Interrupt Configuration Register"]
pub mod srss_intr_cfg;
#[doc = "RES_CAUSE register accessor: an alias for `Reg<RES_CAUSE_SPEC>`"]
pub type RES_CAUSE = crate::Reg<res_cause::RES_CAUSE_SPEC>;
#[doc = "Reset Cause Observation Register"]
pub mod res_cause;
#[doc = "RES_CAUSE2 register accessor: an alias for `Reg<RES_CAUSE2_SPEC>`"]
pub type RES_CAUSE2 = crate::Reg<res_cause2::RES_CAUSE2_SPEC>;
#[doc = "Reset Cause Observation Register 2"]
pub mod res_cause2;
#[doc = "PWR_TRIM_REF_CTL register accessor: an alias for `Reg<PWR_TRIM_REF_CTL_SPEC>`"]
pub type PWR_TRIM_REF_CTL = crate::Reg<pwr_trim_ref_ctl::PWR_TRIM_REF_CTL_SPEC>;
#[doc = "Reference Trim Register"]
pub mod pwr_trim_ref_ctl;
#[doc = "PWR_TRIM_BODOVP_CTL register accessor: an alias for `Reg<PWR_TRIM_BODOVP_CTL_SPEC>`"]
pub type PWR_TRIM_BODOVP_CTL = crate::Reg<pwr_trim_bodovp_ctl::PWR_TRIM_BODOVP_CTL_SPEC>;
#[doc = "BOD/OVP Trim Register"]
pub mod pwr_trim_bodovp_ctl;
#[doc = "CLK_TRIM_CCO_CTL register accessor: an alias for `Reg<CLK_TRIM_CCO_CTL_SPEC>`"]
pub type CLK_TRIM_CCO_CTL = crate::Reg<clk_trim_cco_ctl::CLK_TRIM_CCO_CTL_SPEC>;
#[doc = "CCO Trim Register"]
pub mod clk_trim_cco_ctl;
#[doc = "CLK_TRIM_CCO_CTL2 register accessor: an alias for `Reg<CLK_TRIM_CCO_CTL2_SPEC>`"]
pub type CLK_TRIM_CCO_CTL2 = crate::Reg<clk_trim_cco_ctl2::CLK_TRIM_CCO_CTL2_SPEC>;
#[doc = "CCO Trim Register 2"]
pub mod clk_trim_cco_ctl2;
#[doc = "PWR_TRIM_WAKE_CTL register accessor: an alias for `Reg<PWR_TRIM_WAKE_CTL_SPEC>`"]
pub type PWR_TRIM_WAKE_CTL = crate::Reg<pwr_trim_wake_ctl::PWR_TRIM_WAKE_CTL_SPEC>;
#[doc = "Wakeup Trim Register"]
pub mod pwr_trim_wake_ctl;
#[doc = "PWR_TRIM_LVD_CTL register accessor: an alias for `Reg<PWR_TRIM_LVD_CTL_SPEC>`"]
pub type PWR_TRIM_LVD_CTL = crate::Reg<pwr_trim_lvd_ctl::PWR_TRIM_LVD_CTL_SPEC>;
#[doc = "LVD Trim Register"]
pub mod pwr_trim_lvd_ctl;
#[doc = "CLK_TRIM_ILO_CTL register accessor: an alias for `Reg<CLK_TRIM_ILO_CTL_SPEC>`"]
pub type CLK_TRIM_ILO_CTL = crate::Reg<clk_trim_ilo_ctl::CLK_TRIM_ILO_CTL_SPEC>;
#[doc = "ILO Trim Register"]
pub mod clk_trim_ilo_ctl;
#[doc = "PWR_TRIM_PWRSYS_CTL register accessor: an alias for `Reg<PWR_TRIM_PWRSYS_CTL_SPEC>`"]
pub type PWR_TRIM_PWRSYS_CTL = crate::Reg<pwr_trim_pwrsys_ctl::PWR_TRIM_PWRSYS_CTL_SPEC>;
#[doc = "Power System Trim Register"]
pub mod pwr_trim_pwrsys_ctl;
#[doc = "CLK_TRIM_ECO_CTL register accessor: an alias for `Reg<CLK_TRIM_ECO_CTL_SPEC>`"]
pub type CLK_TRIM_ECO_CTL = crate::Reg<clk_trim_eco_ctl::CLK_TRIM_ECO_CTL_SPEC>;
#[doc = "ECO Trim Register"]
pub mod clk_trim_eco_ctl;
#[doc = "CLK_TRIM_PILO_CTL register accessor: an alias for `Reg<CLK_TRIM_PILO_CTL_SPEC>`"]
pub type CLK_TRIM_PILO_CTL = crate::Reg<clk_trim_pilo_ctl::CLK_TRIM_PILO_CTL_SPEC>;
#[doc = "PILO Trim Register"]
pub mod clk_trim_pilo_ctl;
#[doc = "CLK_TRIM_PILO_CTL2 register accessor: an alias for `Reg<CLK_TRIM_PILO_CTL2_SPEC>`"]
pub type CLK_TRIM_PILO_CTL2 = crate::Reg<clk_trim_pilo_ctl2::CLK_TRIM_PILO_CTL2_SPEC>;
#[doc = "PILO Trim Register 2"]
pub mod clk_trim_pilo_ctl2;
#[doc = "CLK_TRIM_PILO_CTL3 register accessor: an alias for `Reg<CLK_TRIM_PILO_CTL3_SPEC>`"]
pub type CLK_TRIM_PILO_CTL3 = crate::Reg<clk_trim_pilo_ctl3::CLK_TRIM_PILO_CTL3_SPEC>;
#[doc = "PILO Trim Register 3"]
pub mod clk_trim_pilo_ctl3;

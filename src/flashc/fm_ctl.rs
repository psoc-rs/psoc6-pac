#[doc = "FM_CTL register accessor: an alias for `Reg<FM_CTL_SPEC>`"]
pub type FM_CTL = crate::Reg<fm_ctl::FM_CTL_SPEC>;
#[doc = "Flash macro control"]
pub mod fm_ctl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "FM_ADDR register accessor: an alias for `Reg<FM_ADDR_SPEC>`"]
pub type FM_ADDR = crate::Reg<fm_addr::FM_ADDR_SPEC>;
#[doc = "Flash macro address"]
pub mod fm_addr;
#[doc = "GEOMETRY register accessor: an alias for `Reg<GEOMETRY_SPEC>`"]
pub type GEOMETRY = crate::Reg<geometry::GEOMETRY_SPEC>;
#[doc = "Regular flash geometry"]
pub mod geometry;
#[doc = "GEOMETRY_SUPERVISORY register accessor: an alias for `Reg<GEOMETRY_SUPERVISORY_SPEC>`"]
pub type GEOMETRY_SUPERVISORY = crate::Reg<geometry_supervisory::GEOMETRY_SUPERVISORY_SPEC>;
#[doc = "Supervisory flash geometry"]
pub mod geometry_supervisory;
#[doc = "TIMER_CTL register accessor: an alias for `Reg<TIMER_CTL_SPEC>`"]
pub type TIMER_CTL = crate::Reg<timer_ctl::TIMER_CTL_SPEC>;
#[doc = "Timer control"]
pub mod timer_ctl;
#[doc = "ANA_CTL0 register accessor: an alias for `Reg<ANA_CTL0_SPEC>`"]
pub type ANA_CTL0 = crate::Reg<ana_ctl0::ANA_CTL0_SPEC>;
#[doc = "Analog control 0"]
pub mod ana_ctl0;
#[doc = "ANA_CTL1 register accessor: an alias for `Reg<ANA_CTL1_SPEC>`"]
pub type ANA_CTL1 = crate::Reg<ana_ctl1::ANA_CTL1_SPEC>;
#[doc = "Analog control 1"]
pub mod ana_ctl1;
#[doc = "GEOMETRY_GEN register accessor: an alias for `Reg<GEOMETRY_GEN_SPEC>`"]
pub type GEOMETRY_GEN = crate::Reg<geometry_gen::GEOMETRY_GEN_SPEC>;
#[doc = "N/A, DNU"]
pub mod geometry_gen;
#[doc = "TEST_CTL register accessor: an alias for `Reg<TEST_CTL_SPEC>`"]
pub type TEST_CTL = crate::Reg<test_ctl::TEST_CTL_SPEC>;
#[doc = "Test mode control"]
pub mod test_ctl;
#[doc = "WAIT_CTL register accessor: an alias for `Reg<WAIT_CTL_SPEC>`"]
pub type WAIT_CTL = crate::Reg<wait_ctl::WAIT_CTL_SPEC>;
#[doc = "Wiat State control"]
pub mod wait_ctl;
#[doc = "MONITOR_STATUS register accessor: an alias for `Reg<MONITOR_STATUS_SPEC>`"]
pub type MONITOR_STATUS = crate::Reg<monitor_status::MONITOR_STATUS_SPEC>;
#[doc = "Monitor Status"]
pub mod monitor_status;
#[doc = "SCRATCH_CTL register accessor: an alias for `Reg<SCRATCH_CTL_SPEC>`"]
pub type SCRATCH_CTL = crate::Reg<scratch_ctl::SCRATCH_CTL_SPEC>;
#[doc = "Scratch Control"]
pub mod scratch_ctl;
#[doc = "HV_CTL register accessor: an alias for `Reg<HV_CTL_SPEC>`"]
pub type HV_CTL = crate::Reg<hv_ctl::HV_CTL_SPEC>;
#[doc = "High voltage control"]
pub mod hv_ctl;
#[doc = "ACLK_CTL register accessor: an alias for `Reg<ACLK_CTL_SPEC>`"]
pub type ACLK_CTL = crate::Reg<aclk_ctl::ACLK_CTL_SPEC>;
#[doc = "Aclk control"]
pub mod aclk_ctl;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_masked;
#[doc = "FM_HV_DATA_ALL register accessor: an alias for `Reg<FM_HV_DATA_ALL_SPEC>`"]
pub type FM_HV_DATA_ALL = crate::Reg<fm_hv_data_all::FM_HV_DATA_ALL_SPEC>;
#[doc = "Flash macro high Voltage page latches data (for all page latches)"]
pub mod fm_hv_data_all;
#[doc = "CAL_CTL0 register accessor: an alias for `Reg<CAL_CTL0_SPEC>`"]
pub type CAL_CTL0 = crate::Reg<cal_ctl0::CAL_CTL0_SPEC>;
#[doc = "Cal control BG LO trim bits"]
pub mod cal_ctl0;
#[doc = "CAL_CTL1 register accessor: an alias for `Reg<CAL_CTL1_SPEC>`"]
pub type CAL_CTL1 = crate::Reg<cal_ctl1::CAL_CTL1_SPEC>;
#[doc = "Cal control BG HI trim bits"]
pub mod cal_ctl1;
#[doc = "CAL_CTL2 register accessor: an alias for `Reg<CAL_CTL2_SPEC>`"]
pub type CAL_CTL2 = crate::Reg<cal_ctl2::CAL_CTL2_SPEC>;
#[doc = "Cal control BG LO&HI ipref trim, ref sel, fm_active, turbo_ext"]
pub mod cal_ctl2;
#[doc = "CAL_CTL3 register accessor: an alias for `Reg<CAL_CTL3_SPEC>`"]
pub type CAL_CTL3 = crate::Reg<cal_ctl3::CAL_CTL3_SPEC>;
#[doc = "Cal control osc trim bits, idac, sdac, itim, bdac."]
pub mod cal_ctl3;
#[doc = "BOOKMARK register accessor: an alias for `Reg<BOOKMARK_SPEC>`"]
pub type BOOKMARK = crate::Reg<bookmark::BOOKMARK_SPEC>;
#[doc = "Bookmark register - keeps the current FW HV seq"]
pub mod bookmark;
#[doc = "RED_CTL01 register accessor: an alias for `Reg<RED_CTL01_SPEC>`"]
pub type RED_CTL01 = crate::Reg<red_ctl01::RED_CTL01_SPEC>;
#[doc = "Redundancy Control normal sectors 0,1"]
pub mod red_ctl01;
#[doc = "RED_CTL23 register accessor: an alias for `Reg<RED_CTL23_SPEC>`"]
pub type RED_CTL23 = crate::Reg<red_ctl23::RED_CTL23_SPEC>;
#[doc = "Redundancy Controll normal sectors 2,3"]
pub mod red_ctl23;
#[doc = "RED_CTL45 register accessor: an alias for `Reg<RED_CTL45_SPEC>`"]
pub type RED_CTL45 = crate::Reg<red_ctl45::RED_CTL45_SPEC>;
#[doc = "Redundancy Controll normal sectors 4,5"]
pub mod red_ctl45;
#[doc = "RED_CTL67 register accessor: an alias for `Reg<RED_CTL67_SPEC>`"]
pub type RED_CTL67 = crate::Reg<red_ctl67::RED_CTL67_SPEC>;
#[doc = "Redundancy Controll normal sectors 6,7"]
pub mod red_ctl67;
#[doc = "RED_CTL_SM01 register accessor: an alias for `Reg<RED_CTL_SM01_SPEC>`"]
pub type RED_CTL_SM01 = crate::Reg<red_ctl_sm01::RED_CTL_SM01_SPEC>;
#[doc = "Redundancy Controll special sectors 0,1"]
pub mod red_ctl_sm01;
#[doc = "TM_CMPR register accessor: an alias for `Reg<TM_CMPR_SPEC>`"]
pub type TM_CMPR = crate::Reg<tm_cmpr::TM_CMPR_SPEC>;
#[doc = "Do Not Use"]
pub mod tm_cmpr;
#[doc = "FM_HV_DATA register accessor: an alias for `Reg<FM_HV_DATA_SPEC>`"]
pub type FM_HV_DATA = crate::Reg<fm_hv_data::FM_HV_DATA_SPEC>;
#[doc = "Flash macro high Voltage page latches data"]
pub mod fm_hv_data;
#[doc = "FM_MEM_DATA register accessor: an alias for `Reg<FM_MEM_DATA_SPEC>`"]
pub type FM_MEM_DATA = crate::Reg<fm_mem_data::FM_MEM_DATA_SPEC>;
#[doc = "Flash macro memory sense amplifier and column decoder data"]
pub mod fm_mem_data;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub flash_ctl: crate::Reg<flash_ctl::FLASH_CTL_SPEC>,
    #[doc = "0x04 - Flash power control"]
    pub flash_pwr_ctl: crate::Reg<flash_pwr_ctl::FLASH_PWR_CTL_SPEC>,
    #[doc = "0x08 - Command"]
    pub flash_cmd: crate::Reg<flash_cmd::FLASH_CMD_SPEC>,
    _reserved3: [u8; 0xf4],
    #[doc = "0x100 - BIST control"]
    pub bist_ctl: crate::Reg<bist_ctl::BIST_CTL_SPEC>,
    #[doc = "0x104 - BIST command"]
    pub bist_cmd: crate::Reg<bist_cmd::BIST_CMD_SPEC>,
    #[doc = "0x108 - BIST address start register"]
    pub bist_addr_start: crate::Reg<bist_addr_start::BIST_ADDR_START_SPEC>,
    #[doc = "0x10c..0x12c - BIST data register(s)"]
    pub bist_data: [crate::Reg<bist_data::BIST_DATA_SPEC>; 8],
    #[doc = "0x12c..0x14c - BIST data actual register(s)"]
    pub bist_data_act: [crate::Reg<bist_data_act::BIST_DATA_ACT_SPEC>; 8],
    #[doc = "0x14c..0x16c - BIST data expected register(s)"]
    pub bist_data_exp: [crate::Reg<bist_data_exp::BIST_DATA_EXP_SPEC>; 8],
    #[doc = "0x16c - BIST address register"]
    pub bist_addr: crate::Reg<bist_addr::BIST_ADDR_SPEC>,
    #[doc = "0x170 - BIST status register"]
    pub bist_status: crate::Reg<bist_status::BIST_STATUS_SPEC>,
    _reserved11: [u8; 0x028c],
    #[doc = "0x400 - CM0+ cache control"]
    pub cm0_ca_ctl0: crate::Reg<cm0_ca_ctl0::CM0_CA_CTL0_SPEC>,
    #[doc = "0x404 - CM0+ cache control"]
    pub cm0_ca_ctl1: crate::Reg<cm0_ca_ctl1::CM0_CA_CTL1_SPEC>,
    #[doc = "0x408 - CM0+ cache control"]
    pub cm0_ca_ctl2: crate::Reg<cm0_ca_ctl2::CM0_CA_CTL2_SPEC>,
    #[doc = "0x40c - CM0+ cache command"]
    pub cm0_ca_cmd: crate::Reg<cm0_ca_cmd::CM0_CA_CMD_SPEC>,
    _reserved15: [u8; 0x30],
    #[doc = "0x440 - CM0+ cache status 0"]
    pub cm0_ca_status0: crate::Reg<cm0_ca_status0::CM0_CA_STATUS0_SPEC>,
    #[doc = "0x444 - CM0+ cache status 1"]
    pub cm0_ca_status1: crate::Reg<cm0_ca_status1::CM0_CA_STATUS1_SPEC>,
    #[doc = "0x448 - CM0+ cache status 2"]
    pub cm0_ca_status2: crate::Reg<cm0_ca_status2::CM0_CA_STATUS2_SPEC>,
    _reserved18: [u8; 0x34],
    #[doc = "0x480 - CM4 cache control"]
    pub cm4_ca_ctl0: crate::Reg<cm4_ca_ctl0::CM4_CA_CTL0_SPEC>,
    #[doc = "0x484 - CM4 cache control"]
    pub cm4_ca_ctl1: crate::Reg<cm4_ca_ctl1::CM4_CA_CTL1_SPEC>,
    #[doc = "0x488 - CM4 cache control"]
    pub cm4_ca_ctl2: crate::Reg<cm4_ca_ctl2::CM4_CA_CTL2_SPEC>,
    #[doc = "0x48c - CM4 cache command"]
    pub cm4_ca_cmd: crate::Reg<cm4_ca_cmd::CM4_CA_CMD_SPEC>,
    _reserved22: [u8; 0x30],
    #[doc = "0x4c0 - CM4 cache status 0"]
    pub cm4_ca_status0: crate::Reg<cm4_ca_status0::CM4_CA_STATUS0_SPEC>,
    #[doc = "0x4c4 - CM4 cache status 1"]
    pub cm4_ca_status1: crate::Reg<cm4_ca_status1::CM4_CA_STATUS1_SPEC>,
    #[doc = "0x4c8 - CM4 cache status 2"]
    pub cm4_ca_status2: crate::Reg<cm4_ca_status2::CM4_CA_STATUS2_SPEC>,
    _reserved25: [u8; 0x34],
    #[doc = "0x500 - Cryptography buffer control"]
    pub crypto_buff_ctl: crate::Reg<crypto_buff_ctl::CRYPTO_BUFF_CTL_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x508 - Cryptography buffer command"]
    pub crypto_buff_cmd: crate::Reg<crypto_buff_cmd::CRYPTO_BUFF_CMD_SPEC>,
    _reserved27: [u8; 0x74],
    #[doc = "0x580 - Datawire 0 buffer control"]
    pub dw0_buff_ctl: crate::Reg<dw0_buff_ctl::DW0_BUFF_CTL_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x588 - Datawire 0 buffer command"]
    pub dw0_buff_cmd: crate::Reg<dw0_buff_cmd::DW0_BUFF_CMD_SPEC>,
    _reserved29: [u8; 0x74],
    #[doc = "0x600 - Datawire 1 buffer control"]
    pub dw1_buff_ctl: crate::Reg<dw1_buff_ctl::DW1_BUFF_CTL_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x608 - Datawire 1 buffer command"]
    pub dw1_buff_cmd: crate::Reg<dw1_buff_cmd::DW1_BUFF_CMD_SPEC>,
    _reserved31: [u8; 0x74],
    #[doc = "0x680 - Debug access port buffer control"]
    pub dap_buff_ctl: crate::Reg<dap_buff_ctl::DAP_BUFF_CTL_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x688 - Debug access port buffer command"]
    pub dap_buff_cmd: crate::Reg<dap_buff_cmd::DAP_BUFF_CMD_SPEC>,
    _reserved33: [u8; 0x74],
    #[doc = "0x700 - External master 0 buffer control"]
    pub ext_ms0_buff_ctl: crate::Reg<ext_ms0_buff_ctl::EXT_MS0_BUFF_CTL_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0x708 - External master 0 buffer command"]
    pub ext_ms0_buff_cmd: crate::Reg<ext_ms0_buff_cmd::EXT_MS0_BUFF_CMD_SPEC>,
    _reserved35: [u8; 0x74],
    #[doc = "0x780 - External master 1 buffer control"]
    pub ext_ms1_buff_ctl: crate::Reg<ext_ms1_buff_ctl::EXT_MS1_BUFF_CTL_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x788 - External master 1 buffer command"]
    pub ext_ms1_buff_cmd: crate::Reg<ext_ms1_buff_cmd::EXT_MS1_BUFF_CMD_SPEC>,
    _reserved37: [u8; 0xe874],
    #[doc = "0xf000..0x10000 - Flash Macro Registers"]
    pub fm_ctl: FM_CTL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct FM_CTL {
    #[doc = "0x00 - Flash macro control"]
    pub fm_ctl: crate::Reg<self::fm_ctl::fm_ctl::FM_CTL_SPEC>,
    #[doc = "0x04 - Status"]
    pub status: crate::Reg<self::fm_ctl::status::STATUS_SPEC>,
    #[doc = "0x08 - Flash macro address"]
    pub fm_addr: crate::Reg<self::fm_ctl::fm_addr::FM_ADDR_SPEC>,
    #[doc = "0x0c - Regular flash geometry"]
    pub geometry: crate::Reg<self::fm_ctl::geometry::GEOMETRY_SPEC>,
    #[doc = "0x10 - Supervisory flash geometry"]
    pub geometry_supervisory:
        crate::Reg<self::fm_ctl::geometry_supervisory::GEOMETRY_SUPERVISORY_SPEC>,
    #[doc = "0x14 - Timer control"]
    pub timer_ctl: crate::Reg<self::fm_ctl::timer_ctl::TIMER_CTL_SPEC>,
    #[doc = "0x18 - Analog control 0"]
    pub ana_ctl0: crate::Reg<self::fm_ctl::ana_ctl0::ANA_CTL0_SPEC>,
    #[doc = "0x1c - Analog control 1"]
    pub ana_ctl1: crate::Reg<self::fm_ctl::ana_ctl1::ANA_CTL1_SPEC>,
    #[doc = "0x20 - N/A, DNU"]
    pub geometry_gen: crate::Reg<self::fm_ctl::geometry_gen::GEOMETRY_GEN_SPEC>,
    #[doc = "0x24 - Test mode control"]
    pub test_ctl: crate::Reg<self::fm_ctl::test_ctl::TEST_CTL_SPEC>,
    #[doc = "0x28 - Wiat State control"]
    pub wait_ctl: crate::Reg<self::fm_ctl::wait_ctl::WAIT_CTL_SPEC>,
    #[doc = "0x2c - Monitor Status"]
    pub monitor_status: crate::Reg<self::fm_ctl::monitor_status::MONITOR_STATUS_SPEC>,
    #[doc = "0x30 - Scratch Control"]
    pub scratch_ctl: crate::Reg<self::fm_ctl::scratch_ctl::SCRATCH_CTL_SPEC>,
    #[doc = "0x34 - High voltage control"]
    pub hv_ctl: crate::Reg<self::fm_ctl::hv_ctl::HV_CTL_SPEC>,
    #[doc = "0x38 - Aclk control"]
    pub aclk_ctl: crate::Reg<self::fm_ctl::aclk_ctl::ACLK_CTL_SPEC>,
    #[doc = "0x3c - Interrupt"]
    pub intr: crate::Reg<self::fm_ctl::intr::INTR_SPEC>,
    #[doc = "0x40 - Interrupt set"]
    pub intr_set: crate::Reg<self::fm_ctl::intr_set::INTR_SET_SPEC>,
    #[doc = "0x44 - Interrupt mask"]
    pub intr_mask: crate::Reg<self::fm_ctl::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x48 - Interrupt masked"]
    pub intr_masked: crate::Reg<self::fm_ctl::intr_masked::INTR_MASKED_SPEC>,
    #[doc = "0x4c - Flash macro high Voltage page latches data (for all page latches)"]
    pub fm_hv_data_all: crate::Reg<self::fm_ctl::fm_hv_data_all::FM_HV_DATA_ALL_SPEC>,
    #[doc = "0x50 - Cal control BG LO trim bits"]
    pub cal_ctl0: crate::Reg<self::fm_ctl::cal_ctl0::CAL_CTL0_SPEC>,
    #[doc = "0x54 - Cal control BG HI trim bits"]
    pub cal_ctl1: crate::Reg<self::fm_ctl::cal_ctl1::CAL_CTL1_SPEC>,
    #[doc = "0x58 - Cal control BG LO&HI ipref trim, ref sel, fm_active, turbo_ext"]
    pub cal_ctl2: crate::Reg<self::fm_ctl::cal_ctl2::CAL_CTL2_SPEC>,
    #[doc = "0x5c - Cal control osc trim bits, idac, sdac, itim, bdac."]
    pub cal_ctl3: crate::Reg<self::fm_ctl::cal_ctl3::CAL_CTL3_SPEC>,
    #[doc = "0x60 - Bookmark register - keeps the current FW HV seq"]
    pub bookmark: crate::Reg<self::fm_ctl::bookmark::BOOKMARK_SPEC>,
    _reserved25: [u8; 0x1c],
    #[doc = "0x80 - Redundancy Control normal sectors 0,1"]
    pub red_ctl01: crate::Reg<self::fm_ctl::red_ctl01::RED_CTL01_SPEC>,
    #[doc = "0x84 - Redundancy Controll normal sectors 2,3"]
    pub red_ctl23: crate::Reg<self::fm_ctl::red_ctl23::RED_CTL23_SPEC>,
    #[doc = "0x88 - Redundancy Controll normal sectors 4,5"]
    pub red_ctl45: crate::Reg<self::fm_ctl::red_ctl45::RED_CTL45_SPEC>,
    #[doc = "0x8c - Redundancy Controll normal sectors 6,7"]
    pub red_ctl67: crate::Reg<self::fm_ctl::red_ctl67::RED_CTL67_SPEC>,
    #[doc = "0x90 - Redundancy Controll special sectors 0,1"]
    pub red_ctl_sm01: crate::Reg<self::fm_ctl::red_ctl_sm01::RED_CTL_SM01_SPEC>,
    _reserved30: [u8; 0x6c],
    #[doc = "0x100..0x180 - Do Not Use"]
    pub tm_cmpr: [crate::Reg<self::fm_ctl::tm_cmpr::TM_CMPR_SPEC>; 32],
    _reserved31: [u8; 0x0680],
    #[doc = "0x800..0xc00 - Flash macro high Voltage page latches data"]
    pub fm_hv_data: [crate::Reg<self::fm_ctl::fm_hv_data::FM_HV_DATA_SPEC>; 256],
    #[doc = "0xc00..0x1000 - Flash macro memory sense amplifier and column decoder data"]
    pub fm_mem_data: [crate::Reg<self::fm_ctl::fm_mem_data::FM_MEM_DATA_SPEC>; 256],
}
#[doc = r"Register block"]
#[doc = "Flash Macro Registers"]
pub mod fm_ctl;
#[doc = "FLASH_CTL register accessor: an alias for `Reg<FLASH_CTL_SPEC>`"]
pub type FLASH_CTL = crate::Reg<flash_ctl::FLASH_CTL_SPEC>;
#[doc = "Control"]
pub mod flash_ctl;
#[doc = "FLASH_PWR_CTL register accessor: an alias for `Reg<FLASH_PWR_CTL_SPEC>`"]
pub type FLASH_PWR_CTL = crate::Reg<flash_pwr_ctl::FLASH_PWR_CTL_SPEC>;
#[doc = "Flash power control"]
pub mod flash_pwr_ctl;
#[doc = "FLASH_CMD register accessor: an alias for `Reg<FLASH_CMD_SPEC>`"]
pub type FLASH_CMD = crate::Reg<flash_cmd::FLASH_CMD_SPEC>;
#[doc = "Command"]
pub mod flash_cmd;
#[doc = "BIST_CTL register accessor: an alias for `Reg<BIST_CTL_SPEC>`"]
pub type BIST_CTL = crate::Reg<bist_ctl::BIST_CTL_SPEC>;
#[doc = "BIST control"]
pub mod bist_ctl;
#[doc = "BIST_CMD register accessor: an alias for `Reg<BIST_CMD_SPEC>`"]
pub type BIST_CMD = crate::Reg<bist_cmd::BIST_CMD_SPEC>;
#[doc = "BIST command"]
pub mod bist_cmd;
#[doc = "BIST_ADDR_START register accessor: an alias for `Reg<BIST_ADDR_START_SPEC>`"]
pub type BIST_ADDR_START = crate::Reg<bist_addr_start::BIST_ADDR_START_SPEC>;
#[doc = "BIST address start register"]
pub mod bist_addr_start;
#[doc = "BIST_DATA register accessor: an alias for `Reg<BIST_DATA_SPEC>`"]
pub type BIST_DATA = crate::Reg<bist_data::BIST_DATA_SPEC>;
#[doc = "BIST data register(s)"]
pub mod bist_data;
#[doc = "BIST_DATA_ACT register accessor: an alias for `Reg<BIST_DATA_ACT_SPEC>`"]
pub type BIST_DATA_ACT = crate::Reg<bist_data_act::BIST_DATA_ACT_SPEC>;
#[doc = "BIST data actual register(s)"]
pub mod bist_data_act;
#[doc = "BIST_DATA_EXP register accessor: an alias for `Reg<BIST_DATA_EXP_SPEC>`"]
pub type BIST_DATA_EXP = crate::Reg<bist_data_exp::BIST_DATA_EXP_SPEC>;
#[doc = "BIST data expected register(s)"]
pub mod bist_data_exp;
#[doc = "BIST_ADDR register accessor: an alias for `Reg<BIST_ADDR_SPEC>`"]
pub type BIST_ADDR = crate::Reg<bist_addr::BIST_ADDR_SPEC>;
#[doc = "BIST address register"]
pub mod bist_addr;
#[doc = "BIST_STATUS register accessor: an alias for `Reg<BIST_STATUS_SPEC>`"]
pub type BIST_STATUS = crate::Reg<bist_status::BIST_STATUS_SPEC>;
#[doc = "BIST status register"]
pub mod bist_status;
#[doc = "CM0_CA_CTL0 register accessor: an alias for `Reg<CM0_CA_CTL0_SPEC>`"]
pub type CM0_CA_CTL0 = crate::Reg<cm0_ca_ctl0::CM0_CA_CTL0_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl0;
#[doc = "CM0_CA_CTL1 register accessor: an alias for `Reg<CM0_CA_CTL1_SPEC>`"]
pub type CM0_CA_CTL1 = crate::Reg<cm0_ca_ctl1::CM0_CA_CTL1_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl1;
#[doc = "CM0_CA_CTL2 register accessor: an alias for `Reg<CM0_CA_CTL2_SPEC>`"]
pub type CM0_CA_CTL2 = crate::Reg<cm0_ca_ctl2::CM0_CA_CTL2_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl2;
#[doc = "CM0_CA_CMD register accessor: an alias for `Reg<CM0_CA_CMD_SPEC>`"]
pub type CM0_CA_CMD = crate::Reg<cm0_ca_cmd::CM0_CA_CMD_SPEC>;
#[doc = "CM0+ cache command"]
pub mod cm0_ca_cmd;
#[doc = "CM0_CA_STATUS0 register accessor: an alias for `Reg<CM0_CA_STATUS0_SPEC>`"]
pub type CM0_CA_STATUS0 = crate::Reg<cm0_ca_status0::CM0_CA_STATUS0_SPEC>;
#[doc = "CM0+ cache status 0"]
pub mod cm0_ca_status0;
#[doc = "CM0_CA_STATUS1 register accessor: an alias for `Reg<CM0_CA_STATUS1_SPEC>`"]
pub type CM0_CA_STATUS1 = crate::Reg<cm0_ca_status1::CM0_CA_STATUS1_SPEC>;
#[doc = "CM0+ cache status 1"]
pub mod cm0_ca_status1;
#[doc = "CM0_CA_STATUS2 register accessor: an alias for `Reg<CM0_CA_STATUS2_SPEC>`"]
pub type CM0_CA_STATUS2 = crate::Reg<cm0_ca_status2::CM0_CA_STATUS2_SPEC>;
#[doc = "CM0+ cache status 2"]
pub mod cm0_ca_status2;
#[doc = "CM4_CA_CTL0 register accessor: an alias for `Reg<CM4_CA_CTL0_SPEC>`"]
pub type CM4_CA_CTL0 = crate::Reg<cm4_ca_ctl0::CM4_CA_CTL0_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl0;
#[doc = "CM4_CA_CTL1 register accessor: an alias for `Reg<CM4_CA_CTL1_SPEC>`"]
pub type CM4_CA_CTL1 = crate::Reg<cm4_ca_ctl1::CM4_CA_CTL1_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl1;
#[doc = "CM4_CA_CTL2 register accessor: an alias for `Reg<CM4_CA_CTL2_SPEC>`"]
pub type CM4_CA_CTL2 = crate::Reg<cm4_ca_ctl2::CM4_CA_CTL2_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl2;
#[doc = "CM4_CA_CMD register accessor: an alias for `Reg<CM4_CA_CMD_SPEC>`"]
pub type CM4_CA_CMD = crate::Reg<cm4_ca_cmd::CM4_CA_CMD_SPEC>;
#[doc = "CM4 cache command"]
pub mod cm4_ca_cmd;
#[doc = "CM4_CA_STATUS0 register accessor: an alias for `Reg<CM4_CA_STATUS0_SPEC>`"]
pub type CM4_CA_STATUS0 = crate::Reg<cm4_ca_status0::CM4_CA_STATUS0_SPEC>;
#[doc = "CM4 cache status 0"]
pub mod cm4_ca_status0;
#[doc = "CM4_CA_STATUS1 register accessor: an alias for `Reg<CM4_CA_STATUS1_SPEC>`"]
pub type CM4_CA_STATUS1 = crate::Reg<cm4_ca_status1::CM4_CA_STATUS1_SPEC>;
#[doc = "CM4 cache status 1"]
pub mod cm4_ca_status1;
#[doc = "CM4_CA_STATUS2 register accessor: an alias for `Reg<CM4_CA_STATUS2_SPEC>`"]
pub type CM4_CA_STATUS2 = crate::Reg<cm4_ca_status2::CM4_CA_STATUS2_SPEC>;
#[doc = "CM4 cache status 2"]
pub mod cm4_ca_status2;
#[doc = "CRYPTO_BUFF_CTL register accessor: an alias for `Reg<CRYPTO_BUFF_CTL_SPEC>`"]
pub type CRYPTO_BUFF_CTL = crate::Reg<crypto_buff_ctl::CRYPTO_BUFF_CTL_SPEC>;
#[doc = "Cryptography buffer control"]
pub mod crypto_buff_ctl;
#[doc = "CRYPTO_BUFF_CMD register accessor: an alias for `Reg<CRYPTO_BUFF_CMD_SPEC>`"]
pub type CRYPTO_BUFF_CMD = crate::Reg<crypto_buff_cmd::CRYPTO_BUFF_CMD_SPEC>;
#[doc = "Cryptography buffer command"]
pub mod crypto_buff_cmd;
#[doc = "DW0_BUFF_CTL register accessor: an alias for `Reg<DW0_BUFF_CTL_SPEC>`"]
pub type DW0_BUFF_CTL = crate::Reg<dw0_buff_ctl::DW0_BUFF_CTL_SPEC>;
#[doc = "Datawire 0 buffer control"]
pub mod dw0_buff_ctl;
#[doc = "DW0_BUFF_CMD register accessor: an alias for `Reg<DW0_BUFF_CMD_SPEC>`"]
pub type DW0_BUFF_CMD = crate::Reg<dw0_buff_cmd::DW0_BUFF_CMD_SPEC>;
#[doc = "Datawire 0 buffer command"]
pub mod dw0_buff_cmd;
#[doc = "DW1_BUFF_CTL register accessor: an alias for `Reg<DW1_BUFF_CTL_SPEC>`"]
pub type DW1_BUFF_CTL = crate::Reg<dw1_buff_ctl::DW1_BUFF_CTL_SPEC>;
#[doc = "Datawire 1 buffer control"]
pub mod dw1_buff_ctl;
#[doc = "DW1_BUFF_CMD register accessor: an alias for `Reg<DW1_BUFF_CMD_SPEC>`"]
pub type DW1_BUFF_CMD = crate::Reg<dw1_buff_cmd::DW1_BUFF_CMD_SPEC>;
#[doc = "Datawire 1 buffer command"]
pub mod dw1_buff_cmd;
#[doc = "DAP_BUFF_CTL register accessor: an alias for `Reg<DAP_BUFF_CTL_SPEC>`"]
pub type DAP_BUFF_CTL = crate::Reg<dap_buff_ctl::DAP_BUFF_CTL_SPEC>;
#[doc = "Debug access port buffer control"]
pub mod dap_buff_ctl;
#[doc = "DAP_BUFF_CMD register accessor: an alias for `Reg<DAP_BUFF_CMD_SPEC>`"]
pub type DAP_BUFF_CMD = crate::Reg<dap_buff_cmd::DAP_BUFF_CMD_SPEC>;
#[doc = "Debug access port buffer command"]
pub mod dap_buff_cmd;
#[doc = "EXT_MS0_BUFF_CTL register accessor: an alias for `Reg<EXT_MS0_BUFF_CTL_SPEC>`"]
pub type EXT_MS0_BUFF_CTL = crate::Reg<ext_ms0_buff_ctl::EXT_MS0_BUFF_CTL_SPEC>;
#[doc = "External master 0 buffer control"]
pub mod ext_ms0_buff_ctl;
#[doc = "EXT_MS0_BUFF_CMD register accessor: an alias for `Reg<EXT_MS0_BUFF_CMD_SPEC>`"]
pub type EXT_MS0_BUFF_CMD = crate::Reg<ext_ms0_buff_cmd::EXT_MS0_BUFF_CMD_SPEC>;
#[doc = "External master 0 buffer command"]
pub mod ext_ms0_buff_cmd;
#[doc = "EXT_MS1_BUFF_CTL register accessor: an alias for `Reg<EXT_MS1_BUFF_CTL_SPEC>`"]
pub type EXT_MS1_BUFF_CTL = crate::Reg<ext_ms1_buff_ctl::EXT_MS1_BUFF_CTL_SPEC>;
#[doc = "External master 1 buffer control"]
pub mod ext_ms1_buff_ctl;
#[doc = "EXT_MS1_BUFF_CMD register accessor: an alias for `Reg<EXT_MS1_BUFF_CMD_SPEC>`"]
pub type EXT_MS1_BUFF_CMD = crate::Reg<ext_ms1_buff_cmd::EXT_MS1_BUFF_CMD_SPEC>;
#[doc = "External master 1 buffer command"]
pub mod ext_ms1_buff_cmd;

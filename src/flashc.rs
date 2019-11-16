#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub flash_ctl: FLASH_CTL,
    #[doc = "0x04 - Flash power control"]
    pub flash_pwr_ctl: FLASH_PWR_CTL,
    #[doc = "0x08 - Command"]
    pub flash_cmd: FLASH_CMD,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - BIST control"]
    pub bist_ctl: BIST_CTL,
    #[doc = "0x104 - BIST command"]
    pub bist_cmd: BIST_CMD,
    #[doc = "0x108 - BIST address start register"]
    pub bist_addr_start: BIST_ADDR_START,
    #[doc = "0x10c - BIST data register(s)"]
    pub bist_data: [BIST_DATA; 8],
    #[doc = "0x12c - BIST data actual register(s)"]
    pub bist_data_act: [BIST_DATA_ACT; 8],
    #[doc = "0x14c - BIST data expected register(s)"]
    pub bist_data_exp: [BIST_DATA_EXP; 8],
    #[doc = "0x16c - BIST address register"]
    pub bist_addr: BIST_ADDR,
    #[doc = "0x170 - BIST status register"]
    pub bist_status: BIST_STATUS,
    _reserved11: [u8; 652usize],
    #[doc = "0x400 - CM0+ cache control"]
    pub cm0_ca_ctl0: CM0_CA_CTL0,
    #[doc = "0x404 - CM0+ cache control"]
    pub cm0_ca_ctl1: CM0_CA_CTL1,
    #[doc = "0x408 - CM0+ cache control"]
    pub cm0_ca_ctl2: CM0_CA_CTL2,
    #[doc = "0x40c - CM0+ cache command"]
    pub cm0_ca_cmd: CM0_CA_CMD,
    _reserved15: [u8; 48usize],
    #[doc = "0x440 - CM0+ cache status 0"]
    pub cm0_ca_status0: CM0_CA_STATUS0,
    #[doc = "0x444 - CM0+ cache status 1"]
    pub cm0_ca_status1: CM0_CA_STATUS1,
    #[doc = "0x448 - CM0+ cache status 2"]
    pub cm0_ca_status2: CM0_CA_STATUS2,
    _reserved18: [u8; 52usize],
    #[doc = "0x480 - CM4 cache control"]
    pub cm4_ca_ctl0: CM4_CA_CTL0,
    #[doc = "0x484 - CM4 cache control"]
    pub cm4_ca_ctl1: CM4_CA_CTL1,
    #[doc = "0x488 - CM4 cache control"]
    pub cm4_ca_ctl2: CM4_CA_CTL2,
    #[doc = "0x48c - CM4 cache command"]
    pub cm4_ca_cmd: CM4_CA_CMD,
    _reserved22: [u8; 48usize],
    #[doc = "0x4c0 - CM4 cache status 0"]
    pub cm4_ca_status0: CM4_CA_STATUS0,
    #[doc = "0x4c4 - CM4 cache status 1"]
    pub cm4_ca_status1: CM4_CA_STATUS1,
    #[doc = "0x4c8 - CM4 cache status 2"]
    pub cm4_ca_status2: CM4_CA_STATUS2,
    _reserved25: [u8; 52usize],
    #[doc = "0x500 - Cryptography buffer control"]
    pub crypto_buff_ctl: CRYPTO_BUFF_CTL,
    _reserved26: [u8; 4usize],
    #[doc = "0x508 - Cryptography buffer command"]
    pub crypto_buff_cmd: CRYPTO_BUFF_CMD,
    _reserved27: [u8; 116usize],
    #[doc = "0x580 - Datawire 0 buffer control"]
    pub dw0_buff_ctl: DW0_BUFF_CTL,
    _reserved28: [u8; 4usize],
    #[doc = "0x588 - Datawire 0 buffer command"]
    pub dw0_buff_cmd: DW0_BUFF_CMD,
    _reserved29: [u8; 116usize],
    #[doc = "0x600 - Datawire 1 buffer control"]
    pub dw1_buff_ctl: DW1_BUFF_CTL,
    _reserved30: [u8; 4usize],
    #[doc = "0x608 - Datawire 1 buffer command"]
    pub dw1_buff_cmd: DW1_BUFF_CMD,
    _reserved31: [u8; 116usize],
    #[doc = "0x680 - Debug access port buffer control"]
    pub dap_buff_ctl: DAP_BUFF_CTL,
    _reserved32: [u8; 4usize],
    #[doc = "0x688 - Debug access port buffer command"]
    pub dap_buff_cmd: DAP_BUFF_CMD,
    _reserved33: [u8; 116usize],
    #[doc = "0x700 - External master 0 buffer control"]
    pub ext_ms0_buff_ctl: EXT_MS0_BUFF_CTL,
    _reserved34: [u8; 4usize],
    #[doc = "0x708 - External master 0 buffer command"]
    pub ext_ms0_buff_cmd: EXT_MS0_BUFF_CMD,
    _reserved35: [u8; 116usize],
    #[doc = "0x780 - External master 1 buffer control"]
    pub ext_ms1_buff_ctl: EXT_MS1_BUFF_CTL,
    _reserved36: [u8; 4usize],
    #[doc = "0x788 - External master 1 buffer command"]
    pub ext_ms1_buff_cmd: EXT_MS1_BUFF_CMD,
    _reserved37: [u8; 59508usize],
    #[doc = "0xf000 - Flash Macro Registers"]
    pub fm_ctl: FM_CTL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct FM_CTL {
    #[doc = "0x00 - Flash macro control"]
    pub fm_ctl: self::fm_ctl::FM_CTL,
    #[doc = "0x04 - Status"]
    pub status: self::fm_ctl::STATUS,
    #[doc = "0x08 - Flash macro address"]
    pub fm_addr: self::fm_ctl::FM_ADDR,
    #[doc = "0x0c - Regular flash geometry"]
    pub geometry: self::fm_ctl::GEOMETRY,
    #[doc = "0x10 - Supervisory flash geometry"]
    pub geometry_supervisory: self::fm_ctl::GEOMETRY_SUPERVISORY,
    #[doc = "0x14 - Timer control"]
    pub timer_ctl: self::fm_ctl::TIMER_CTL,
    #[doc = "0x18 - Analog control 0"]
    pub ana_ctl0: self::fm_ctl::ANA_CTL0,
    #[doc = "0x1c - Analog control 1"]
    pub ana_ctl1: self::fm_ctl::ANA_CTL1,
    #[doc = "0x20 - N/A, DNU"]
    pub geometry_gen: self::fm_ctl::GEOMETRY_GEN,
    #[doc = "0x24 - Test mode control"]
    pub test_ctl: self::fm_ctl::TEST_CTL,
    #[doc = "0x28 - Wiat State control"]
    pub wait_ctl: self::fm_ctl::WAIT_CTL,
    #[doc = "0x2c - Monitor Status"]
    pub monitor_status: self::fm_ctl::MONITOR_STATUS,
    #[doc = "0x30 - Scratch Control"]
    pub scratch_ctl: self::fm_ctl::SCRATCH_CTL,
    #[doc = "0x34 - High voltage control"]
    pub hv_ctl: self::fm_ctl::HV_CTL,
    #[doc = "0x38 - Aclk control"]
    pub aclk_ctl: self::fm_ctl::ACLK_CTL,
    #[doc = "0x3c - Interrupt"]
    pub intr: self::fm_ctl::INTR,
    #[doc = "0x40 - Interrupt set"]
    pub intr_set: self::fm_ctl::INTR_SET,
    #[doc = "0x44 - Interrupt mask"]
    pub intr_mask: self::fm_ctl::INTR_MASK,
    #[doc = "0x48 - Interrupt masked"]
    pub intr_masked: self::fm_ctl::INTR_MASKED,
    #[doc = "0x4c - Flash macro high Voltage page latches data (for all page latches)"]
    pub fm_hv_data_all: self::fm_ctl::FM_HV_DATA_ALL,
    #[doc = "0x50 - Cal control BG LO trim bits"]
    pub cal_ctl0: self::fm_ctl::CAL_CTL0,
    #[doc = "0x54 - Cal control BG HI trim bits"]
    pub cal_ctl1: self::fm_ctl::CAL_CTL1,
    #[doc = "0x58 - Cal control BG LO&HI ipref trim, ref sel, fm_active, turbo_ext"]
    pub cal_ctl2: self::fm_ctl::CAL_CTL2,
    #[doc = "0x5c - Cal control osc trim bits, idac, sdac, itim, bdac."]
    pub cal_ctl3: self::fm_ctl::CAL_CTL3,
    #[doc = "0x60 - Bookmark register - keeps the current FW HV seq"]
    pub bookmark: self::fm_ctl::BOOKMARK,
    _reserved25: [u8; 28usize],
    #[doc = "0x80 - Redundancy Control normal sectors 0,1"]
    pub red_ctl01: self::fm_ctl::RED_CTL01,
    #[doc = "0x84 - Redundancy Controll normal sectors 2,3"]
    pub red_ctl23: self::fm_ctl::RED_CTL23,
    #[doc = "0x88 - Redundancy Controll normal sectors 4,5"]
    pub red_ctl45: self::fm_ctl::RED_CTL45,
    #[doc = "0x8c - Redundancy Controll normal sectors 6,7"]
    pub red_ctl67: self::fm_ctl::RED_CTL67,
    #[doc = "0x90 - Redundancy Controll special sectors 0,1"]
    pub red_ctl_sm01: self::fm_ctl::RED_CTL_SM01,
    _reserved30: [u8; 108usize],
    #[doc = "0x100 - Do Not Use"]
    pub tm_cmpr: [self::fm_ctl::TM_CMPR; 32],
    _reserved31: [u8; 1664usize],
    #[doc = "0x800 - Flash macro high Voltage page latches data"]
    pub fm_hv_data: [self::fm_ctl::FM_HV_DATA; 256],
    #[doc = "0xc00 - Flash macro memory sense amplifier and column decoder data"]
    pub fm_mem_data: [self::fm_ctl::FM_MEM_DATA; 256],
}
#[doc = r"Register block"]
#[doc = "Flash Macro Registers"]
pub mod fm_ctl;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctl](flash_ctl) module"]
pub type FLASH_CTL = crate::Reg<u32, _FLASH_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_CTL;
#[doc = "`read()` method returns [flash_ctl::R](flash_ctl::R) reader structure"]
impl crate::Readable for FLASH_CTL {}
#[doc = "`write(|w| ..)` method takes [flash_ctl::W](flash_ctl::W) writer structure"]
impl crate::Writable for FLASH_CTL {}
#[doc = "Control"]
pub mod flash_ctl;
#[doc = "Flash power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_pwr_ctl](flash_pwr_ctl) module"]
pub type FLASH_PWR_CTL = crate::Reg<u32, _FLASH_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_PWR_CTL;
#[doc = "`read()` method returns [flash_pwr_ctl::R](flash_pwr_ctl::R) reader structure"]
impl crate::Readable for FLASH_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [flash_pwr_ctl::W](flash_pwr_ctl::W) writer structure"]
impl crate::Writable for FLASH_PWR_CTL {}
#[doc = "Flash power control"]
pub mod flash_pwr_ctl;
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_cmd](flash_cmd) module"]
pub type FLASH_CMD = crate::Reg<u32, _FLASH_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_CMD;
#[doc = "`read()` method returns [flash_cmd::R](flash_cmd::R) reader structure"]
impl crate::Readable for FLASH_CMD {}
#[doc = "`write(|w| ..)` method takes [flash_cmd::W](flash_cmd::W) writer structure"]
impl crate::Writable for FLASH_CMD {}
#[doc = "Command"]
pub mod flash_cmd;
#[doc = "BIST control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_ctl](bist_ctl) module"]
pub type BIST_CTL = crate::Reg<u32, _BIST_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIST_CTL;
#[doc = "`read()` method returns [bist_ctl::R](bist_ctl::R) reader structure"]
impl crate::Readable for BIST_CTL {}
#[doc = "`write(|w| ..)` method takes [bist_ctl::W](bist_ctl::W) writer structure"]
impl crate::Writable for BIST_CTL {}
#[doc = "BIST control"]
pub mod bist_ctl;
#[doc = "BIST command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_cmd](bist_cmd) module"]
pub type BIST_CMD = crate::Reg<u32, _BIST_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIST_CMD;
#[doc = "`read()` method returns [bist_cmd::R](bist_cmd::R) reader structure"]
impl crate::Readable for BIST_CMD {}
#[doc = "`write(|w| ..)` method takes [bist_cmd::W](bist_cmd::W) writer structure"]
impl crate::Writable for BIST_CMD {}
#[doc = "BIST command"]
pub mod bist_cmd;
#[doc = "BIST address start register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_addr_start](bist_addr_start) module"]
pub type BIST_ADDR_START = crate::Reg<u32, _BIST_ADDR_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIST_ADDR_START;
#[doc = "`read()` method returns [bist_addr_start::R](bist_addr_start::R) reader structure"]
impl crate::Readable for BIST_ADDR_START {}
#[doc = "`write(|w| ..)` method takes [bist_addr_start::W](bist_addr_start::W) writer structure"]
impl crate::Writable for BIST_ADDR_START {}
#[doc = "BIST address start register"]
pub mod bist_addr_start;
#[doc = "BIST data register(s)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_data](bist_data) module"]
pub type BIST_DATA = crate::Reg<u32, _BIST_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIST_DATA;
#[doc = "`read()` method returns [bist_data::R](bist_data::R) reader structure"]
impl crate::Readable for BIST_DATA {}
#[doc = "`write(|w| ..)` method takes [bist_data::W](bist_data::W) writer structure"]
impl crate::Writable for BIST_DATA {}
#[doc = "BIST data register(s)"]
pub mod bist_data;
#[doc = "BIST data actual register(s)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_data_act](bist_data_act) module"]
pub type BIST_DATA_ACT = crate::Reg<u32, _BIST_DATA_ACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIST_DATA_ACT;
#[doc = "`read()` method returns [bist_data_act::R](bist_data_act::R) reader structure"]
impl crate::Readable for BIST_DATA_ACT {}
#[doc = "BIST data actual register(s)"]
pub mod bist_data_act;
#[doc = "BIST data expected register(s)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_data_exp](bist_data_exp) module"]
pub type BIST_DATA_EXP = crate::Reg<u32, _BIST_DATA_EXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIST_DATA_EXP;
#[doc = "`read()` method returns [bist_data_exp::R](bist_data_exp::R) reader structure"]
impl crate::Readable for BIST_DATA_EXP {}
#[doc = "BIST data expected register(s)"]
pub mod bist_data_exp;
#[doc = "BIST address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_addr](bist_addr) module"]
pub type BIST_ADDR = crate::Reg<u32, _BIST_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIST_ADDR;
#[doc = "`read()` method returns [bist_addr::R](bist_addr::R) reader structure"]
impl crate::Readable for BIST_ADDR {}
#[doc = "BIST address register"]
pub mod bist_addr;
#[doc = "BIST status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_status](bist_status) module"]
pub type BIST_STATUS = crate::Reg<u32, _BIST_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIST_STATUS;
#[doc = "`read()` method returns [bist_status::R](bist_status::R) reader structure"]
impl crate::Readable for BIST_STATUS {}
#[doc = "`write(|w| ..)` method takes [bist_status::W](bist_status::W) writer structure"]
impl crate::Writable for BIST_STATUS {}
#[doc = "BIST status register"]
pub mod bist_status;
#[doc = "CM0+ cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_ctl0](cm0_ca_ctl0) module"]
pub type CM0_CA_CTL0 = crate::Reg<u32, _CM0_CA_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_CTL0;
#[doc = "`read()` method returns [cm0_ca_ctl0::R](cm0_ca_ctl0::R) reader structure"]
impl crate::Readable for CM0_CA_CTL0 {}
#[doc = "`write(|w| ..)` method takes [cm0_ca_ctl0::W](cm0_ca_ctl0::W) writer structure"]
impl crate::Writable for CM0_CA_CTL0 {}
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl0;
#[doc = "CM0+ cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_ctl1](cm0_ca_ctl1) module"]
pub type CM0_CA_CTL1 = crate::Reg<u32, _CM0_CA_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_CTL1;
#[doc = "`read()` method returns [cm0_ca_ctl1::R](cm0_ca_ctl1::R) reader structure"]
impl crate::Readable for CM0_CA_CTL1 {}
#[doc = "`write(|w| ..)` method takes [cm0_ca_ctl1::W](cm0_ca_ctl1::W) writer structure"]
impl crate::Writable for CM0_CA_CTL1 {}
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl1;
#[doc = "CM0+ cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_ctl2](cm0_ca_ctl2) module"]
pub type CM0_CA_CTL2 = crate::Reg<u32, _CM0_CA_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_CTL2;
#[doc = "`read()` method returns [cm0_ca_ctl2::R](cm0_ca_ctl2::R) reader structure"]
impl crate::Readable for CM0_CA_CTL2 {}
#[doc = "`write(|w| ..)` method takes [cm0_ca_ctl2::W](cm0_ca_ctl2::W) writer structure"]
impl crate::Writable for CM0_CA_CTL2 {}
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl2;
#[doc = "CM0+ cache command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_cmd](cm0_ca_cmd) module"]
pub type CM0_CA_CMD = crate::Reg<u32, _CM0_CA_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_CMD;
#[doc = "`read()` method returns [cm0_ca_cmd::R](cm0_ca_cmd::R) reader structure"]
impl crate::Readable for CM0_CA_CMD {}
#[doc = "`write(|w| ..)` method takes [cm0_ca_cmd::W](cm0_ca_cmd::W) writer structure"]
impl crate::Writable for CM0_CA_CMD {}
#[doc = "CM0+ cache command"]
pub mod cm0_ca_cmd;
#[doc = "CM0+ cache status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_status0](cm0_ca_status0) module"]
pub type CM0_CA_STATUS0 = crate::Reg<u32, _CM0_CA_STATUS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_STATUS0;
#[doc = "`read()` method returns [cm0_ca_status0::R](cm0_ca_status0::R) reader structure"]
impl crate::Readable for CM0_CA_STATUS0 {}
#[doc = "CM0+ cache status 0"]
pub mod cm0_ca_status0;
#[doc = "CM0+ cache status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_status1](cm0_ca_status1) module"]
pub type CM0_CA_STATUS1 = crate::Reg<u32, _CM0_CA_STATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_STATUS1;
#[doc = "`read()` method returns [cm0_ca_status1::R](cm0_ca_status1::R) reader structure"]
impl crate::Readable for CM0_CA_STATUS1 {}
#[doc = "CM0+ cache status 1"]
pub mod cm0_ca_status1;
#[doc = "CM0+ cache status 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_status2](cm0_ca_status2) module"]
pub type CM0_CA_STATUS2 = crate::Reg<u32, _CM0_CA_STATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_STATUS2;
#[doc = "`read()` method returns [cm0_ca_status2::R](cm0_ca_status2::R) reader structure"]
impl crate::Readable for CM0_CA_STATUS2 {}
#[doc = "CM0+ cache status 2"]
pub mod cm0_ca_status2;
#[doc = "CM4 cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_ctl0](cm4_ca_ctl0) module"]
pub type CM4_CA_CTL0 = crate::Reg<u32, _CM4_CA_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_CTL0;
#[doc = "`read()` method returns [cm4_ca_ctl0::R](cm4_ca_ctl0::R) reader structure"]
impl crate::Readable for CM4_CA_CTL0 {}
#[doc = "`write(|w| ..)` method takes [cm4_ca_ctl0::W](cm4_ca_ctl0::W) writer structure"]
impl crate::Writable for CM4_CA_CTL0 {}
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl0;
#[doc = "CM4 cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_ctl1](cm4_ca_ctl1) module"]
pub type CM4_CA_CTL1 = crate::Reg<u32, _CM4_CA_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_CTL1;
#[doc = "`read()` method returns [cm4_ca_ctl1::R](cm4_ca_ctl1::R) reader structure"]
impl crate::Readable for CM4_CA_CTL1 {}
#[doc = "`write(|w| ..)` method takes [cm4_ca_ctl1::W](cm4_ca_ctl1::W) writer structure"]
impl crate::Writable for CM4_CA_CTL1 {}
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl1;
#[doc = "CM4 cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_ctl2](cm4_ca_ctl2) module"]
pub type CM4_CA_CTL2 = crate::Reg<u32, _CM4_CA_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_CTL2;
#[doc = "`read()` method returns [cm4_ca_ctl2::R](cm4_ca_ctl2::R) reader structure"]
impl crate::Readable for CM4_CA_CTL2 {}
#[doc = "`write(|w| ..)` method takes [cm4_ca_ctl2::W](cm4_ca_ctl2::W) writer structure"]
impl crate::Writable for CM4_CA_CTL2 {}
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl2;
#[doc = "CM4 cache command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_cmd](cm4_ca_cmd) module"]
pub type CM4_CA_CMD = crate::Reg<u32, _CM4_CA_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_CMD;
#[doc = "`read()` method returns [cm4_ca_cmd::R](cm4_ca_cmd::R) reader structure"]
impl crate::Readable for CM4_CA_CMD {}
#[doc = "`write(|w| ..)` method takes [cm4_ca_cmd::W](cm4_ca_cmd::W) writer structure"]
impl crate::Writable for CM4_CA_CMD {}
#[doc = "CM4 cache command"]
pub mod cm4_ca_cmd;
#[doc = "CM4 cache status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status0](cm4_ca_status0) module"]
pub type CM4_CA_STATUS0 = crate::Reg<u32, _CM4_CA_STATUS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_STATUS0;
#[doc = "`read()` method returns [cm4_ca_status0::R](cm4_ca_status0::R) reader structure"]
impl crate::Readable for CM4_CA_STATUS0 {}
#[doc = "CM4 cache status 0"]
pub mod cm4_ca_status0;
#[doc = "CM4 cache status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status1](cm4_ca_status1) module"]
pub type CM4_CA_STATUS1 = crate::Reg<u32, _CM4_CA_STATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_STATUS1;
#[doc = "`read()` method returns [cm4_ca_status1::R](cm4_ca_status1::R) reader structure"]
impl crate::Readable for CM4_CA_STATUS1 {}
#[doc = "CM4 cache status 1"]
pub mod cm4_ca_status1;
#[doc = "CM4 cache status 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status2](cm4_ca_status2) module"]
pub type CM4_CA_STATUS2 = crate::Reg<u32, _CM4_CA_STATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_STATUS2;
#[doc = "`read()` method returns [cm4_ca_status2::R](cm4_ca_status2::R) reader structure"]
impl crate::Readable for CM4_CA_STATUS2 {}
#[doc = "CM4 cache status 2"]
pub mod cm4_ca_status2;
#[doc = "Cryptography buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_buff_ctl](crypto_buff_ctl) module"]
pub type CRYPTO_BUFF_CTL = crate::Reg<u32, _CRYPTO_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_BUFF_CTL;
#[doc = "`read()` method returns [crypto_buff_ctl::R](crypto_buff_ctl::R) reader structure"]
impl crate::Readable for CRYPTO_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [crypto_buff_ctl::W](crypto_buff_ctl::W) writer structure"]
impl crate::Writable for CRYPTO_BUFF_CTL {}
#[doc = "Cryptography buffer control"]
pub mod crypto_buff_ctl;
#[doc = "Cryptography buffer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_buff_cmd](crypto_buff_cmd) module"]
pub type CRYPTO_BUFF_CMD = crate::Reg<u32, _CRYPTO_BUFF_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_BUFF_CMD;
#[doc = "`read()` method returns [crypto_buff_cmd::R](crypto_buff_cmd::R) reader structure"]
impl crate::Readable for CRYPTO_BUFF_CMD {}
#[doc = "`write(|w| ..)` method takes [crypto_buff_cmd::W](crypto_buff_cmd::W) writer structure"]
impl crate::Writable for CRYPTO_BUFF_CMD {}
#[doc = "Cryptography buffer command"]
pub mod crypto_buff_cmd;
#[doc = "Datawire 0 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dw0_buff_ctl](dw0_buff_ctl) module"]
pub type DW0_BUFF_CTL = crate::Reg<u32, _DW0_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DW0_BUFF_CTL;
#[doc = "`read()` method returns [dw0_buff_ctl::R](dw0_buff_ctl::R) reader structure"]
impl crate::Readable for DW0_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [dw0_buff_ctl::W](dw0_buff_ctl::W) writer structure"]
impl crate::Writable for DW0_BUFF_CTL {}
#[doc = "Datawire 0 buffer control"]
pub mod dw0_buff_ctl;
#[doc = "Datawire 0 buffer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dw0_buff_cmd](dw0_buff_cmd) module"]
pub type DW0_BUFF_CMD = crate::Reg<u32, _DW0_BUFF_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DW0_BUFF_CMD;
#[doc = "`read()` method returns [dw0_buff_cmd::R](dw0_buff_cmd::R) reader structure"]
impl crate::Readable for DW0_BUFF_CMD {}
#[doc = "`write(|w| ..)` method takes [dw0_buff_cmd::W](dw0_buff_cmd::W) writer structure"]
impl crate::Writable for DW0_BUFF_CMD {}
#[doc = "Datawire 0 buffer command"]
pub mod dw0_buff_cmd;
#[doc = "Datawire 1 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dw1_buff_ctl](dw1_buff_ctl) module"]
pub type DW1_BUFF_CTL = crate::Reg<u32, _DW1_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DW1_BUFF_CTL;
#[doc = "`read()` method returns [dw1_buff_ctl::R](dw1_buff_ctl::R) reader structure"]
impl crate::Readable for DW1_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [dw1_buff_ctl::W](dw1_buff_ctl::W) writer structure"]
impl crate::Writable for DW1_BUFF_CTL {}
#[doc = "Datawire 1 buffer control"]
pub mod dw1_buff_ctl;
#[doc = "Datawire 1 buffer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dw1_buff_cmd](dw1_buff_cmd) module"]
pub type DW1_BUFF_CMD = crate::Reg<u32, _DW1_BUFF_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DW1_BUFF_CMD;
#[doc = "`read()` method returns [dw1_buff_cmd::R](dw1_buff_cmd::R) reader structure"]
impl crate::Readable for DW1_BUFF_CMD {}
#[doc = "`write(|w| ..)` method takes [dw1_buff_cmd::W](dw1_buff_cmd::W) writer structure"]
impl crate::Writable for DW1_BUFF_CMD {}
#[doc = "Datawire 1 buffer command"]
pub mod dw1_buff_cmd;
#[doc = "Debug access port buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dap_buff_ctl](dap_buff_ctl) module"]
pub type DAP_BUFF_CTL = crate::Reg<u32, _DAP_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAP_BUFF_CTL;
#[doc = "`read()` method returns [dap_buff_ctl::R](dap_buff_ctl::R) reader structure"]
impl crate::Readable for DAP_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [dap_buff_ctl::W](dap_buff_ctl::W) writer structure"]
impl crate::Writable for DAP_BUFF_CTL {}
#[doc = "Debug access port buffer control"]
pub mod dap_buff_ctl;
#[doc = "Debug access port buffer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dap_buff_cmd](dap_buff_cmd) module"]
pub type DAP_BUFF_CMD = crate::Reg<u32, _DAP_BUFF_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAP_BUFF_CMD;
#[doc = "`read()` method returns [dap_buff_cmd::R](dap_buff_cmd::R) reader structure"]
impl crate::Readable for DAP_BUFF_CMD {}
#[doc = "`write(|w| ..)` method takes [dap_buff_cmd::W](dap_buff_cmd::W) writer structure"]
impl crate::Writable for DAP_BUFF_CMD {}
#[doc = "Debug access port buffer command"]
pub mod dap_buff_cmd;
#[doc = "External master 0 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_ms0_buff_ctl](ext_ms0_buff_ctl) module"]
pub type EXT_MS0_BUFF_CTL = crate::Reg<u32, _EXT_MS0_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_MS0_BUFF_CTL;
#[doc = "`read()` method returns [ext_ms0_buff_ctl::R](ext_ms0_buff_ctl::R) reader structure"]
impl crate::Readable for EXT_MS0_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [ext_ms0_buff_ctl::W](ext_ms0_buff_ctl::W) writer structure"]
impl crate::Writable for EXT_MS0_BUFF_CTL {}
#[doc = "External master 0 buffer control"]
pub mod ext_ms0_buff_ctl;
#[doc = "External master 0 buffer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_ms0_buff_cmd](ext_ms0_buff_cmd) module"]
pub type EXT_MS0_BUFF_CMD = crate::Reg<u32, _EXT_MS0_BUFF_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_MS0_BUFF_CMD;
#[doc = "`read()` method returns [ext_ms0_buff_cmd::R](ext_ms0_buff_cmd::R) reader structure"]
impl crate::Readable for EXT_MS0_BUFF_CMD {}
#[doc = "`write(|w| ..)` method takes [ext_ms0_buff_cmd::W](ext_ms0_buff_cmd::W) writer structure"]
impl crate::Writable for EXT_MS0_BUFF_CMD {}
#[doc = "External master 0 buffer command"]
pub mod ext_ms0_buff_cmd;
#[doc = "External master 1 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_ms1_buff_ctl](ext_ms1_buff_ctl) module"]
pub type EXT_MS1_BUFF_CTL = crate::Reg<u32, _EXT_MS1_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_MS1_BUFF_CTL;
#[doc = "`read()` method returns [ext_ms1_buff_ctl::R](ext_ms1_buff_ctl::R) reader structure"]
impl crate::Readable for EXT_MS1_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [ext_ms1_buff_ctl::W](ext_ms1_buff_ctl::W) writer structure"]
impl crate::Writable for EXT_MS1_BUFF_CTL {}
#[doc = "External master 1 buffer control"]
pub mod ext_ms1_buff_ctl;
#[doc = "External master 1 buffer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_ms1_buff_cmd](ext_ms1_buff_cmd) module"]
pub type EXT_MS1_BUFF_CMD = crate::Reg<u32, _EXT_MS1_BUFF_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_MS1_BUFF_CMD;
#[doc = "`read()` method returns [ext_ms1_buff_cmd::R](ext_ms1_buff_cmd::R) reader structure"]
impl crate::Readable for EXT_MS1_BUFF_CMD {}
#[doc = "`write(|w| ..)` method takes [ext_ms1_buff_cmd::W](ext_ms1_buff_cmd::W) writer structure"]
impl crate::Writable for EXT_MS1_BUFF_CMD {}
#[doc = "External master 1 buffer command"]
pub mod ext_ms1_buff_cmd;

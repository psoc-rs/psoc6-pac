#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - GPIO port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 0x40],
    #[doc = "0x80..0xc0 - GPIO port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 0x40],
    #[doc = "0x100..0x140 - GPIO port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 0x40],
    #[doc = "0x180..0x1c0 - GPIO port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 0x40],
    #[doc = "0x200..0x240 - GPIO port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 0x40],
    #[doc = "0x280..0x2c0 - GPIO port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 0x40],
    #[doc = "0x300..0x340 - GPIO port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 0x40],
    #[doc = "0x380..0x3c0 - GPIO port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 0x40],
    #[doc = "0x400..0x440 - GPIO port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 0x40],
    #[doc = "0x480..0x4c0 - GPIO port registers"]
    pub prt9: PRT,
    _reserved10: [u8; 0x40],
    #[doc = "0x500..0x540 - GPIO port registers"]
    pub prt10: PRT,
    _reserved11: [u8; 0x40],
    #[doc = "0x580..0x5c0 - GPIO port registers"]
    pub prt11: PRT,
    _reserved12: [u8; 0x40],
    #[doc = "0x600..0x640 - GPIO port registers"]
    pub prt12: PRT,
    _reserved13: [u8; 0x40],
    #[doc = "0x680..0x6c0 - GPIO port registers"]
    pub prt13: PRT,
    _reserved14: [u8; 0x40],
    #[doc = "0x700..0x740 - GPIO port registers"]
    pub prt14: PRT,
    _reserved15: [u8; 0x38c0],
    #[doc = "0x4000 - Interrupt port cause register 0"]
    pub intr_cause0: crate::Reg<intr_cause0::INTR_CAUSE0_SPEC>,
    #[doc = "0x4004 - Interrupt port cause register 1"]
    pub intr_cause1: crate::Reg<intr_cause1::INTR_CAUSE1_SPEC>,
    #[doc = "0x4008 - Interrupt port cause register 2"]
    pub intr_cause2: crate::Reg<intr_cause2::INTR_CAUSE2_SPEC>,
    #[doc = "0x400c - Interrupt port cause register 3"]
    pub intr_cause3: crate::Reg<intr_cause3::INTR_CAUSE3_SPEC>,
    #[doc = "0x4010 - Extern power supply detection register"]
    pub vdd_active: crate::Reg<vdd_active::VDD_ACTIVE_SPEC>,
    #[doc = "0x4014 - Supply detection interrupt register"]
    pub vdd_intr: crate::Reg<vdd_intr::VDD_INTR_SPEC>,
    #[doc = "0x4018 - Supply detection interrupt mask register"]
    pub vdd_intr_mask: crate::Reg<vdd_intr_mask::VDD_INTR_MASK_SPEC>,
    #[doc = "0x401c - Supply detection interrupt masked register"]
    pub vdd_intr_masked: crate::Reg<vdd_intr_masked::VDD_INTR_MASKED_SPEC>,
    #[doc = "0x4020 - Supply detection interrupt set register"]
    pub vdd_intr_set: crate::Reg<vdd_intr_set::VDD_INTR_SET_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Port output data register"]
    pub out: crate::Reg<self::prt::out::OUT_SPEC>,
    #[doc = "0x04 - Port output data clear register"]
    pub out_clr: crate::Reg<self::prt::out_clr::OUT_CLR_SPEC>,
    #[doc = "0x08 - Port output data set register"]
    pub out_set: crate::Reg<self::prt::out_set::OUT_SET_SPEC>,
    #[doc = "0x0c - Port output data invert register"]
    pub out_inv: crate::Reg<self::prt::out_inv::OUT_INV_SPEC>,
    #[doc = "0x10 - Port input state register"]
    pub in_: crate::Reg<self::prt::in_::IN_SPEC>,
    #[doc = "0x14 - Port interrupt status register"]
    pub intr: crate::Reg<self::prt::intr::INTR_SPEC>,
    #[doc = "0x18 - Port interrupt mask register"]
    pub intr_mask: crate::Reg<self::prt::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x1c - Port interrupt masked status register"]
    pub intr_masked: crate::Reg<self::prt::intr_masked::INTR_MASKED_SPEC>,
    #[doc = "0x20 - Port interrupt set register"]
    pub intr_set: crate::Reg<self::prt::intr_set::INTR_SET_SPEC>,
    #[doc = "0x24 - Port interrupt configuration register"]
    pub intr_cfg: crate::Reg<self::prt::intr_cfg::INTR_CFG_SPEC>,
    #[doc = "0x28 - Port configuration register"]
    pub cfg: crate::Reg<self::prt::cfg::CFG_SPEC>,
    #[doc = "0x2c - Port input buffer configuration register"]
    pub cfg_in: crate::Reg<self::prt::cfg_in::CFG_IN_SPEC>,
    #[doc = "0x30 - Port output buffer configuration register"]
    pub cfg_out: crate::Reg<self::prt::cfg_out::CFG_OUT_SPEC>,
    #[doc = "0x34 - Port SIO configuration register"]
    pub cfg_sio: crate::Reg<self::prt::cfg_sio::CFG_SIO_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x3c - Port GPIO5V input buffer configuration register"]
    pub cfg_in_gpio5v: crate::Reg<self::prt::cfg_in_gpio5v::CFG_IN_GPIO5V_SPEC>,
}
#[doc = r"Register block"]
#[doc = "GPIO port registers"]
pub mod prt;
#[doc = "INTR_CAUSE0 register accessor: an alias for `Reg<INTR_CAUSE0_SPEC>`"]
pub type INTR_CAUSE0 = crate::Reg<intr_cause0::INTR_CAUSE0_SPEC>;
#[doc = "Interrupt port cause register 0"]
pub mod intr_cause0;
#[doc = "INTR_CAUSE1 register accessor: an alias for `Reg<INTR_CAUSE1_SPEC>`"]
pub type INTR_CAUSE1 = crate::Reg<intr_cause1::INTR_CAUSE1_SPEC>;
#[doc = "Interrupt port cause register 1"]
pub mod intr_cause1;
#[doc = "INTR_CAUSE2 register accessor: an alias for `Reg<INTR_CAUSE2_SPEC>`"]
pub type INTR_CAUSE2 = crate::Reg<intr_cause2::INTR_CAUSE2_SPEC>;
#[doc = "Interrupt port cause register 2"]
pub mod intr_cause2;
#[doc = "INTR_CAUSE3 register accessor: an alias for `Reg<INTR_CAUSE3_SPEC>`"]
pub type INTR_CAUSE3 = crate::Reg<intr_cause3::INTR_CAUSE3_SPEC>;
#[doc = "Interrupt port cause register 3"]
pub mod intr_cause3;
#[doc = "VDD_ACTIVE register accessor: an alias for `Reg<VDD_ACTIVE_SPEC>`"]
pub type VDD_ACTIVE = crate::Reg<vdd_active::VDD_ACTIVE_SPEC>;
#[doc = "Extern power supply detection register"]
pub mod vdd_active;
#[doc = "VDD_INTR register accessor: an alias for `Reg<VDD_INTR_SPEC>`"]
pub type VDD_INTR = crate::Reg<vdd_intr::VDD_INTR_SPEC>;
#[doc = "Supply detection interrupt register"]
pub mod vdd_intr;
#[doc = "VDD_INTR_MASK register accessor: an alias for `Reg<VDD_INTR_MASK_SPEC>`"]
pub type VDD_INTR_MASK = crate::Reg<vdd_intr_mask::VDD_INTR_MASK_SPEC>;
#[doc = "Supply detection interrupt mask register"]
pub mod vdd_intr_mask;
#[doc = "VDD_INTR_MASKED register accessor: an alias for `Reg<VDD_INTR_MASKED_SPEC>`"]
pub type VDD_INTR_MASKED = crate::Reg<vdd_intr_masked::VDD_INTR_MASKED_SPEC>;
#[doc = "Supply detection interrupt masked register"]
pub mod vdd_intr_masked;
#[doc = "VDD_INTR_SET register accessor: an alias for `Reg<VDD_INTR_SET_SPEC>`"]
pub type VDD_INTR_SET = crate::Reg<vdd_intr_set::VDD_INTR_SET_SPEC>;
#[doc = "Supply detection interrupt set register"]
pub mod vdd_intr_set;

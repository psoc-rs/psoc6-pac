#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x28 - Peripheral group structure"]
    pub gr0: GR,
    _reserved1: [u8; 0x18],
    #[doc = "0x40..0x68 - Peripheral group structure"]
    pub gr1: GR,
    _reserved2: [u8; 0x18],
    #[doc = "0x80..0xa8 - Peripheral group structure"]
    pub gr2: GR,
    _reserved3: [u8; 0x18],
    #[doc = "0xc0..0xe8 - Peripheral group structure"]
    pub gr3: GR,
    _reserved4: [u8; 0x18],
    #[doc = "0x100..0x128 - Peripheral group structure"]
    pub gr4: GR,
    _reserved5: [u8; 0x18],
    #[doc = "0x140..0x168 - Peripheral group structure"]
    pub gr5: GR,
    _reserved6: [u8; 0x18],
    #[doc = "0x180..0x1a8 - Peripheral group structure"]
    pub gr6: GR,
    _reserved7: [u8; 0x18],
    #[doc = "0x1c0..0x1e8 - Peripheral group structure"]
    pub gr7: GR,
    _reserved8: [u8; 0x18],
    #[doc = "0x200..0x228 - Peripheral group structure"]
    pub gr8: GR,
    _reserved9: [u8; 0x18],
    #[doc = "0x240..0x268 - Peripheral group structure"]
    pub gr9: GR,
    _reserved10: [u8; 0x18],
    #[doc = "0x280..0x2a8 - Peripheral group structure"]
    pub gr10: GR,
    _reserved11: [u8; 0x0158],
    #[doc = "0x400 - Divider command register"]
    pub div_cmd: crate::Reg<div_cmd::DIV_CMD_SPEC>,
    _reserved12: [u8; 0x03fc],
    #[doc = "0x800..0x900 - Divider control register (for 8.0 divider)"]
    pub div_8_ctl: [crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>; 64],
    #[doc = "0x900..0xa00 - Divider control register (for 16.0 divider)"]
    pub div_16_ctl: [crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>; 64],
    #[doc = "0xa00..0xb00 - Divider control register (for 16.5 divider)"]
    pub div_16_5_ctl: [crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>; 64],
    #[doc = "0xb00..0xbfc - Divider control register (for 24.5 divider)"]
    pub div_24_5_ctl: [crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>; 63],
    _reserved16: [u8; 0x04],
    #[doc = "0xc00..0xe00 - Clock control register"]
    pub clock_ctl: [crate::Reg<clock_ctl::CLOCK_CTL_SPEC>; 128],
    _reserved17: [u8; 0x0200],
    #[doc = "0x1000 - Trigger command register"]
    pub tr_cmd: crate::Reg<tr_cmd::TR_CMD_SPEC>,
    _reserved18: [u8; 0x0ffc],
    #[doc = "0x2000..0x3e00 - Trigger group"]
    pub tr_gr: [TR_GR; 15],
    _reserved19: [u8; 0x0200],
    #[doc = "0x4000..0x4028 - PPU structure with programmable address"]
    pub ppu_pr0: PPU_PR,
    _reserved20: [u8; 0x18],
    #[doc = "0x4040..0x4068 - PPU structure with programmable address"]
    pub ppu_pr1: PPU_PR,
    _reserved21: [u8; 0x18],
    #[doc = "0x4080..0x40a8 - PPU structure with programmable address"]
    pub ppu_pr2: PPU_PR,
    _reserved22: [u8; 0x18],
    #[doc = "0x40c0..0x40e8 - PPU structure with programmable address"]
    pub ppu_pr3: PPU_PR,
    _reserved23: [u8; 0x18],
    #[doc = "0x4100..0x4128 - PPU structure with programmable address"]
    pub ppu_pr4: PPU_PR,
    _reserved24: [u8; 0x18],
    #[doc = "0x4140..0x4168 - PPU structure with programmable address"]
    pub ppu_pr5: PPU_PR,
    _reserved25: [u8; 0x18],
    #[doc = "0x4180..0x41a8 - PPU structure with programmable address"]
    pub ppu_pr6: PPU_PR,
    _reserved26: [u8; 0x18],
    #[doc = "0x41c0..0x41e8 - PPU structure with programmable address"]
    pub ppu_pr7: PPU_PR,
    _reserved27: [u8; 0x18],
    #[doc = "0x4200..0x4228 - PPU structure with programmable address"]
    pub ppu_pr8: PPU_PR,
    _reserved28: [u8; 0x18],
    #[doc = "0x4240..0x4268 - PPU structure with programmable address"]
    pub ppu_pr9: PPU_PR,
    _reserved29: [u8; 0x18],
    #[doc = "0x4280..0x42a8 - PPU structure with programmable address"]
    pub ppu_pr10: PPU_PR,
    _reserved30: [u8; 0x18],
    #[doc = "0x42c0..0x42e8 - PPU structure with programmable address"]
    pub ppu_pr11: PPU_PR,
    _reserved31: [u8; 0x18],
    #[doc = "0x4300..0x4328 - PPU structure with programmable address"]
    pub ppu_pr12: PPU_PR,
    _reserved32: [u8; 0x18],
    #[doc = "0x4340..0x4368 - PPU structure with programmable address"]
    pub ppu_pr13: PPU_PR,
    _reserved33: [u8; 0x18],
    #[doc = "0x4380..0x43a8 - PPU structure with programmable address"]
    pub ppu_pr14: PPU_PR,
    _reserved34: [u8; 0x18],
    #[doc = "0x43c0..0x43e8 - PPU structure with programmable address"]
    pub ppu_pr15: PPU_PR,
    _reserved35: [u8; 0x0c18],
    #[doc = "0x5000..0x5028 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr0: PPU_GR,
    _reserved36: [u8; 0x18],
    #[doc = "0x5040..0x5068 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr1: PPU_GR,
    _reserved37: [u8; 0x18],
    #[doc = "0x5080..0x50a8 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr2: PPU_GR,
    _reserved38: [u8; 0x18],
    #[doc = "0x50c0..0x50e8 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr3: PPU_GR,
    _reserved39: [u8; 0x18],
    #[doc = "0x5100..0x5128 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr4: PPU_GR,
    _reserved40: [u8; 0x18],
    #[doc = "0x5140..0x5168 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr5: PPU_GR,
    _reserved41: [u8; 0x18],
    #[doc = "0x5180..0x51a8 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr6: PPU_GR,
    _reserved42: [u8; 0x18],
    #[doc = "0x51c0..0x51e8 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr7: PPU_GR,
    _reserved43: [u8; 0x18],
    #[doc = "0x5200..0x5228 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr8: PPU_GR,
    _reserved44: [u8; 0x18],
    #[doc = "0x5240..0x5268 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr9: PPU_GR,
    _reserved45: [u8; 0x18],
    #[doc = "0x5280..0x52a8 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr10: PPU_GR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GR {
    #[doc = "0x00 - Clock control"]
    pub clock_ctl: crate::Reg<self::gr::clock_ctl::CLOCK_CTL_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Slave control"]
    pub sl_ctl: crate::Reg<self::gr::sl_ctl::SL_CTL_SPEC>,
    #[doc = "0x24 - Timeout control"]
    pub timeout_ctl: crate::Reg<self::gr::timeout_ctl::TIMEOUT_CTL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Peripheral group structure"]
pub mod gr;
#[doc = r"Register block"]
#[repr(C)]
pub struct TR_GR {
    #[doc = "0x00..0x200 - Trigger control register"]
    pub tr_out_ctl: [crate::Reg<self::tr_gr::tr_out_ctl::TR_OUT_CTL_SPEC>; 128],
}
#[doc = r"Register block"]
#[doc = "Trigger group"]
pub mod tr_gr;
#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_PR {
    #[doc = "0x00 - PPU region address 0 (slave structure)"]
    pub addr0: crate::Reg<self::ppu_pr::addr0::ADDR0_SPEC>,
    #[doc = "0x04 - PPU region attributes 0 (slave structure)"]
    pub att0: crate::Reg<self::ppu_pr::att0::ATT0_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - PPU region address 1 (master structure)"]
    pub addr1: crate::Reg<self::ppu_pr::addr1::ADDR1_SPEC>,
    #[doc = "0x24 - PPU region attributes 1 (master structure)"]
    pub att1: crate::Reg<self::ppu_pr::att1::ATT1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PPU structure with programmable address"]
pub mod ppu_pr;
#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_GR {
    #[doc = "0x00 - PPU region address 0 (slave structure)"]
    pub addr0: crate::Reg<self::ppu_gr::addr0::ADDR0_SPEC>,
    #[doc = "0x04 - PPU region attributes 0 (slave structure)"]
    pub att0: crate::Reg<self::ppu_gr::att0::ATT0_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - PPU region address 1 (master structure)"]
    pub addr1: crate::Reg<self::ppu_gr::addr1::ADDR1_SPEC>,
    #[doc = "0x24 - PPU region attributes 1 (master structure)"]
    pub att1: crate::Reg<self::ppu_gr::att1::ATT1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PPU structure with fixed/constant address for a peripheral group"]
pub mod ppu_gr;
#[doc = "DIV_CMD register accessor: an alias for `Reg<DIV_CMD_SPEC>`"]
pub type DIV_CMD = crate::Reg<div_cmd::DIV_CMD_SPEC>;
#[doc = "Divider command register"]
pub mod div_cmd;
#[doc = "DIV_8_CTL register accessor: an alias for `Reg<DIV_8_CTL_SPEC>`"]
pub type DIV_8_CTL = crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>;
#[doc = "Divider control register (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "DIV_16_CTL register accessor: an alias for `Reg<DIV_16_CTL_SPEC>`"]
pub type DIV_16_CTL = crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>;
#[doc = "Divider control register (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "DIV_16_5_CTL register accessor: an alias for `Reg<DIV_16_5_CTL_SPEC>`"]
pub type DIV_16_5_CTL = crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>;
#[doc = "Divider control register (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "DIV_24_5_CTL register accessor: an alias for `Reg<DIV_24_5_CTL_SPEC>`"]
pub type DIV_24_5_CTL = crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>;
#[doc = "Divider control register (for 24.5 divider)"]
pub mod div_24_5_ctl;
#[doc = "CLOCK_CTL register accessor: an alias for `Reg<CLOCK_CTL_SPEC>`"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control register"]
pub mod clock_ctl;
#[doc = "TR_CMD register accessor: an alias for `Reg<TR_CMD_SPEC>`"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Trigger command register"]
pub mod tr_cmd;

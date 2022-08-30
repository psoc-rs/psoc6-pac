#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xf4 - Programmable IO port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 0x0c],
    #[doc = "0x100..0x1f4 - Programmable IO port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 0x0c],
    #[doc = "0x200..0x2f4 - Programmable IO port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 0x0c],
    #[doc = "0x300..0x3f4 - Programmable IO port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 0x0c],
    #[doc = "0x400..0x4f4 - Programmable IO port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 0x0c],
    #[doc = "0x500..0x5f4 - Programmable IO port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 0x0c],
    #[doc = "0x600..0x6f4 - Programmable IO port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 0x0c],
    #[doc = "0x700..0x7f4 - Programmable IO port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 0x0c],
    #[doc = "0x800..0x8f4 - Programmable IO port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 0x0c],
    #[doc = "0x900..0x9f4 - Programmable IO port registers"]
    pub prt9: PRT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Control register"]
    pub ctl: crate::Reg<self::prt::ctl::CTL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Synchronization control register"]
    pub sync_ctl: crate::Reg<self::prt::sync_ctl::SYNC_CTL_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20..0x40 - LUT component input selection"]
    pub lut_sel: [crate::Reg<self::prt::lut_sel::LUT_SEL_SPEC>; 8],
    #[doc = "0x40..0x60 - LUT component control register"]
    pub lut_ctl: [crate::Reg<self::prt::lut_ctl::LUT_CTL_SPEC>; 8],
    _reserved4: [u8; 0x60],
    #[doc = "0xc0 - Data unit component input selection"]
    pub du_sel: crate::Reg<self::prt::du_sel::DU_SEL_SPEC>,
    #[doc = "0xc4 - Data unit component control register"]
    pub du_ctl: crate::Reg<self::prt::du_ctl::DU_CTL_SPEC>,
    _reserved6: [u8; 0x28],
    #[doc = "0xf0 - Data register"]
    pub data: crate::Reg<self::prt::data::DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Programmable IO port registers"]
pub mod prt;

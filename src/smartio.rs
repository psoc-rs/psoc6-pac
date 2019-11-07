#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Programmable IO port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 12usize],
    #[doc = "0x100 - Programmable IO port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 12usize],
    #[doc = "0x200 - Programmable IO port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 12usize],
    #[doc = "0x300 - Programmable IO port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 12usize],
    #[doc = "0x400 - Programmable IO port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 12usize],
    #[doc = "0x500 - Programmable IO port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 12usize],
    #[doc = "0x600 - Programmable IO port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 12usize],
    #[doc = "0x700 - Programmable IO port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 12usize],
    #[doc = "0x800 - Programmable IO port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 12usize],
    #[doc = "0x900 - Programmable IO port registers"]
    pub prt9: PRT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Control register"]
    pub ctl: self::prt::CTL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Synchronization control register"]
    pub sync_ctl: self::prt::SYNC_CTL,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - LUT component input selection"]
    pub lut_sel: [self::prt::LUT_SEL; 8],
    #[doc = "0x40 - LUT component control register"]
    pub lut_ctl: [self::prt::LUT_CTL; 8],
    _reserved4: [u8; 96usize],
    #[doc = "0xc0 - Data unit component input selection"]
    pub du_sel: self::prt::DU_SEL,
    #[doc = "0xc4 - Data unit component control register"]
    pub du_ctl: self::prt::DU_CTL,
    _reserved6: [u8; 40usize],
    #[doc = "0xf0 - Data register"]
    pub data: self::prt::DATA,
}
#[doc = r"Register block"]
#[doc = "Programmable IO port registers"]
pub mod prt;

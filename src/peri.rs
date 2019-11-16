#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral group structure"]
    pub gr0: GR,
    _reserved1: [u8; 24usize],
    #[doc = "0x40 - Peripheral group structure"]
    pub gr1: GR,
    _reserved2: [u8; 24usize],
    #[doc = "0x80 - Peripheral group structure"]
    pub gr2: GR,
    _reserved3: [u8; 24usize],
    #[doc = "0xc0 - Peripheral group structure"]
    pub gr3: GR,
    _reserved4: [u8; 24usize],
    #[doc = "0x100 - Peripheral group structure"]
    pub gr4: GR,
    _reserved5: [u8; 24usize],
    #[doc = "0x140 - Peripheral group structure"]
    pub gr5: GR,
    _reserved6: [u8; 24usize],
    #[doc = "0x180 - Peripheral group structure"]
    pub gr6: GR,
    _reserved7: [u8; 24usize],
    #[doc = "0x1c0 - Peripheral group structure"]
    pub gr7: GR,
    _reserved8: [u8; 24usize],
    #[doc = "0x200 - Peripheral group structure"]
    pub gr8: GR,
    _reserved9: [u8; 24usize],
    #[doc = "0x240 - Peripheral group structure"]
    pub gr9: GR,
    _reserved10: [u8; 24usize],
    #[doc = "0x280 - Peripheral group structure"]
    pub gr10: GR,
    _reserved11: [u8; 344usize],
    #[doc = "0x400 - Divider command register"]
    pub div_cmd: DIV_CMD,
    _reserved12: [u8; 1020usize],
    #[doc = "0x800 - Divider control register (for 8.0 divider)"]
    pub div_8_ctl: [DIV_8_CTL; 64],
    #[doc = "0x900 - Divider control register (for 16.0 divider)"]
    pub div_16_ctl: [DIV_16_CTL; 64],
    #[doc = "0xa00 - Divider control register (for 16.5 divider)"]
    pub div_16_5_ctl: [DIV_16_5_CTL; 64],
    #[doc = "0xb00 - Divider control register (for 24.5 divider)"]
    pub div_24_5_ctl: [DIV_24_5_CTL; 63],
    _reserved16: [u8; 4usize],
    #[doc = "0xc00 - Clock control register"]
    pub clock_ctl: [CLOCK_CTL; 128],
    _reserved17: [u8; 512usize],
    #[doc = "0x1000 - Trigger command register"]
    pub tr_cmd: TR_CMD,
    _reserved18: [u8; 4092usize],
    #[doc = "0x2000 - Trigger group"]
    pub tr_gr: [TR_GR; 15],
    _reserved19: [u8; 512usize],
    #[doc = "0x4000 - PPU structure with programmable address"]
    pub ppu_pr0: PPU_PR,
    _reserved20: [u8; 24usize],
    #[doc = "0x4040 - PPU structure with programmable address"]
    pub ppu_pr1: PPU_PR,
    _reserved21: [u8; 24usize],
    #[doc = "0x4080 - PPU structure with programmable address"]
    pub ppu_pr2: PPU_PR,
    _reserved22: [u8; 24usize],
    #[doc = "0x40c0 - PPU structure with programmable address"]
    pub ppu_pr3: PPU_PR,
    _reserved23: [u8; 24usize],
    #[doc = "0x4100 - PPU structure with programmable address"]
    pub ppu_pr4: PPU_PR,
    _reserved24: [u8; 24usize],
    #[doc = "0x4140 - PPU structure with programmable address"]
    pub ppu_pr5: PPU_PR,
    _reserved25: [u8; 24usize],
    #[doc = "0x4180 - PPU structure with programmable address"]
    pub ppu_pr6: PPU_PR,
    _reserved26: [u8; 24usize],
    #[doc = "0x41c0 - PPU structure with programmable address"]
    pub ppu_pr7: PPU_PR,
    _reserved27: [u8; 24usize],
    #[doc = "0x4200 - PPU structure with programmable address"]
    pub ppu_pr8: PPU_PR,
    _reserved28: [u8; 24usize],
    #[doc = "0x4240 - PPU structure with programmable address"]
    pub ppu_pr9: PPU_PR,
    _reserved29: [u8; 24usize],
    #[doc = "0x4280 - PPU structure with programmable address"]
    pub ppu_pr10: PPU_PR,
    _reserved30: [u8; 24usize],
    #[doc = "0x42c0 - PPU structure with programmable address"]
    pub ppu_pr11: PPU_PR,
    _reserved31: [u8; 24usize],
    #[doc = "0x4300 - PPU structure with programmable address"]
    pub ppu_pr12: PPU_PR,
    _reserved32: [u8; 24usize],
    #[doc = "0x4340 - PPU structure with programmable address"]
    pub ppu_pr13: PPU_PR,
    _reserved33: [u8; 24usize],
    #[doc = "0x4380 - PPU structure with programmable address"]
    pub ppu_pr14: PPU_PR,
    _reserved34: [u8; 24usize],
    #[doc = "0x43c0 - PPU structure with programmable address"]
    pub ppu_pr15: PPU_PR,
    _reserved35: [u8; 3096usize],
    #[doc = "0x5000 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr0: PPU_GR,
    _reserved36: [u8; 24usize],
    #[doc = "0x5040 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr1: PPU_GR,
    _reserved37: [u8; 24usize],
    #[doc = "0x5080 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr2: PPU_GR,
    _reserved38: [u8; 24usize],
    #[doc = "0x50c0 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr3: PPU_GR,
    _reserved39: [u8; 24usize],
    #[doc = "0x5100 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr4: PPU_GR,
    _reserved40: [u8; 24usize],
    #[doc = "0x5140 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr5: PPU_GR,
    _reserved41: [u8; 24usize],
    #[doc = "0x5180 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr6: PPU_GR,
    _reserved42: [u8; 24usize],
    #[doc = "0x51c0 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr7: PPU_GR,
    _reserved43: [u8; 24usize],
    #[doc = "0x5200 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr8: PPU_GR,
    _reserved44: [u8; 24usize],
    #[doc = "0x5240 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr9: PPU_GR,
    _reserved45: [u8; 24usize],
    #[doc = "0x5280 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr10: PPU_GR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GR {
    #[doc = "0x00 - Clock control"]
    pub clock_ctl: self::gr::CLOCK_CTL,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - Slave control"]
    pub sl_ctl: self::gr::SL_CTL,
    #[doc = "0x24 - Timeout control"]
    pub timeout_ctl: self::gr::TIMEOUT_CTL,
}
#[doc = r"Register block"]
#[doc = "Peripheral group structure"]
pub mod gr;
#[doc = r"Register block"]
#[repr(C)]
pub struct TR_GR {
    #[doc = "0x00 - Trigger control register"]
    pub tr_out_ctl: [self::tr_gr::TR_OUT_CTL; 128],
}
#[doc = r"Register block"]
#[doc = "Trigger group"]
pub mod tr_gr;
#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_PR {
    #[doc = "0x00 - PPU region address 0 (slave structure)"]
    pub addr0: self::ppu_pr::ADDR0,
    #[doc = "0x04 - PPU region attributes 0 (slave structure)"]
    pub att0: self::ppu_pr::ATT0,
    _reserved2: [u8; 24usize],
    #[doc = "0x20 - PPU region address 1 (master structure)"]
    pub addr1: self::ppu_pr::ADDR1,
    #[doc = "0x24 - PPU region attributes 1 (master structure)"]
    pub att1: self::ppu_pr::ATT1,
}
#[doc = r"Register block"]
#[doc = "PPU structure with programmable address"]
pub mod ppu_pr;
#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_GR {
    #[doc = "0x00 - PPU region address 0 (slave structure)"]
    pub addr0: self::ppu_gr::ADDR0,
    #[doc = "0x04 - PPU region attributes 0 (slave structure)"]
    pub att0: self::ppu_gr::ATT0,
    _reserved2: [u8; 24usize],
    #[doc = "0x20 - PPU region address 1 (master structure)"]
    pub addr1: self::ppu_gr::ADDR1,
    #[doc = "0x24 - PPU region attributes 1 (master structure)"]
    pub att1: self::ppu_gr::ATT1,
}
#[doc = r"Register block"]
#[doc = "PPU structure with fixed/constant address for a peripheral group"]
pub mod ppu_gr;
#[doc = "Divider command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_cmd](div_cmd) module"]
pub type DIV_CMD = crate::Reg<u32, _DIV_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_CMD;
#[doc = "`read()` method returns [div_cmd::R](div_cmd::R) reader structure"]
impl crate::Readable for DIV_CMD {}
#[doc = "`write(|w| ..)` method takes [div_cmd::W](div_cmd::W) writer structure"]
impl crate::Writable for DIV_CMD {}
#[doc = "Divider command register"]
pub mod div_cmd;
#[doc = "Divider control register (for 8.0 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_8_ctl](div_8_ctl) module"]
pub type DIV_8_CTL = crate::Reg<u32, _DIV_8_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_8_CTL;
#[doc = "`read()` method returns [div_8_ctl::R](div_8_ctl::R) reader structure"]
impl crate::Readable for DIV_8_CTL {}
#[doc = "`write(|w| ..)` method takes [div_8_ctl::W](div_8_ctl::W) writer structure"]
impl crate::Writable for DIV_8_CTL {}
#[doc = "Divider control register (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "Divider control register (for 16.0 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_16_ctl](div_16_ctl) module"]
pub type DIV_16_CTL = crate::Reg<u32, _DIV_16_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_16_CTL;
#[doc = "`read()` method returns [div_16_ctl::R](div_16_ctl::R) reader structure"]
impl crate::Readable for DIV_16_CTL {}
#[doc = "`write(|w| ..)` method takes [div_16_ctl::W](div_16_ctl::W) writer structure"]
impl crate::Writable for DIV_16_CTL {}
#[doc = "Divider control register (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "Divider control register (for 16.5 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_16_5_ctl](div_16_5_ctl) module"]
pub type DIV_16_5_CTL = crate::Reg<u32, _DIV_16_5_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_16_5_CTL;
#[doc = "`read()` method returns [div_16_5_ctl::R](div_16_5_ctl::R) reader structure"]
impl crate::Readable for DIV_16_5_CTL {}
#[doc = "`write(|w| ..)` method takes [div_16_5_ctl::W](div_16_5_ctl::W) writer structure"]
impl crate::Writable for DIV_16_5_CTL {}
#[doc = "Divider control register (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "Divider control register (for 24.5 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_24_5_ctl](div_24_5_ctl) module"]
pub type DIV_24_5_CTL = crate::Reg<u32, _DIV_24_5_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_24_5_CTL;
#[doc = "`read()` method returns [div_24_5_ctl::R](div_24_5_ctl::R) reader structure"]
impl crate::Readable for DIV_24_5_CTL {}
#[doc = "`write(|w| ..)` method takes [div_24_5_ctl::W](div_24_5_ctl::W) writer structure"]
impl crate::Writable for DIV_24_5_CTL {}
#[doc = "Divider control register (for 24.5 divider)"]
pub mod div_24_5_ctl;
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](clock_ctl) module"]
pub type CLOCK_CTL = crate::Reg<u32, _CLOCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_CTL;
#[doc = "`read()` method returns [clock_ctl::R](clock_ctl::R) reader structure"]
impl crate::Readable for CLOCK_CTL {}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](clock_ctl::W) writer structure"]
impl crate::Writable for CLOCK_CTL {}
#[doc = "Clock control register"]
pub mod clock_ctl;
#[doc = "Trigger command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_cmd](tr_cmd) module"]
pub type TR_CMD = crate::Reg<u32, _TR_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CMD;
#[doc = "`read()` method returns [tr_cmd::R](tr_cmd::R) reader structure"]
impl crate::Readable for TR_CMD {}
#[doc = "`write(|w| ..)` method takes [tr_cmd::W](tr_cmd::W) writer structure"]
impl crate::Writable for TR_CMD {}
#[doc = "Trigger command register"]
pub mod tr_cmd;

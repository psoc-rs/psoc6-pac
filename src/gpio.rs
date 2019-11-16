#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 64usize],
    #[doc = "0x80 - GPIO port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 64usize],
    #[doc = "0x100 - GPIO port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 64usize],
    #[doc = "0x180 - GPIO port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 64usize],
    #[doc = "0x200 - GPIO port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 64usize],
    #[doc = "0x280 - GPIO port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 64usize],
    #[doc = "0x300 - GPIO port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 64usize],
    #[doc = "0x380 - GPIO port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 64usize],
    #[doc = "0x400 - GPIO port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 64usize],
    #[doc = "0x480 - GPIO port registers"]
    pub prt9: PRT,
    _reserved10: [u8; 64usize],
    #[doc = "0x500 - GPIO port registers"]
    pub prt10: PRT,
    _reserved11: [u8; 64usize],
    #[doc = "0x580 - GPIO port registers"]
    pub prt11: PRT,
    _reserved12: [u8; 64usize],
    #[doc = "0x600 - GPIO port registers"]
    pub prt12: PRT,
    _reserved13: [u8; 64usize],
    #[doc = "0x680 - GPIO port registers"]
    pub prt13: PRT,
    _reserved14: [u8; 64usize],
    #[doc = "0x700 - GPIO port registers"]
    pub prt14: PRT,
    _reserved15: [u8; 14528usize],
    #[doc = "0x4000 - Interrupt port cause register 0"]
    pub intr_cause0: INTR_CAUSE0,
    #[doc = "0x4004 - Interrupt port cause register 1"]
    pub intr_cause1: INTR_CAUSE1,
    #[doc = "0x4008 - Interrupt port cause register 2"]
    pub intr_cause2: INTR_CAUSE2,
    #[doc = "0x400c - Interrupt port cause register 3"]
    pub intr_cause3: INTR_CAUSE3,
    #[doc = "0x4010 - Extern power supply detection register"]
    pub vdd_active: VDD_ACTIVE,
    #[doc = "0x4014 - Supply detection interrupt register"]
    pub vdd_intr: VDD_INTR,
    #[doc = "0x4018 - Supply detection interrupt mask register"]
    pub vdd_intr_mask: VDD_INTR_MASK,
    #[doc = "0x401c - Supply detection interrupt masked register"]
    pub vdd_intr_masked: VDD_INTR_MASKED,
    #[doc = "0x4020 - Supply detection interrupt set register"]
    pub vdd_intr_set: VDD_INTR_SET,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Port output data register"]
    pub out: self::prt::OUT,
    #[doc = "0x04 - Port output data set register"]
    pub out_clr: self::prt::OUT_CLR,
    #[doc = "0x08 - Port output data clear register"]
    pub out_set: self::prt::OUT_SET,
    #[doc = "0x0c - Port output data invert register"]
    pub out_inv: self::prt::OUT_INV,
    #[doc = "0x10 - Port input state register"]
    pub in_: self::prt::IN,
    #[doc = "0x14 - Port interrupt status register"]
    pub intr: self::prt::INTR,
    #[doc = "0x18 - Port interrupt mask register"]
    pub intr_mask: self::prt::INTR_MASK,
    #[doc = "0x1c - Port interrupt masked status register"]
    pub intr_masked: self::prt::INTR_MASKED,
    #[doc = "0x20 - Port interrupt set register"]
    pub intr_set: self::prt::INTR_SET,
    #[doc = "0x24 - Port interrupt configuration register"]
    pub intr_cfg: self::prt::INTR_CFG,
    #[doc = "0x28 - Port configuration register"]
    pub cfg: self::prt::CFG,
    #[doc = "0x2c - Port input buffer configuration register"]
    pub cfg_in: self::prt::CFG_IN,
    #[doc = "0x30 - Port output buffer configuration register"]
    pub cfg_out: self::prt::CFG_OUT,
    #[doc = "0x34 - Port SIO configuration register"]
    pub cfg_sio: self::prt::CFG_SIO,
    _reserved14: [u8; 4usize],
    #[doc = "0x3c - Port GPIO5V input buffer configuration register"]
    pub cfg_in_gpio5v: self::prt::CFG_IN_GPIO5V,
}
#[doc = r"Register block"]
#[doc = "GPIO port registers"]
pub mod prt;
#[doc = "Interrupt port cause register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause0](intr_cause0) module"]
pub type INTR_CAUSE0 = crate::Reg<u32, _INTR_CAUSE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE0;
#[doc = "`read()` method returns [intr_cause0::R](intr_cause0::R) reader structure"]
impl crate::Readable for INTR_CAUSE0 {}
#[doc = "Interrupt port cause register 0"]
pub mod intr_cause0;
#[doc = "Interrupt port cause register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause1](intr_cause1) module"]
pub type INTR_CAUSE1 = crate::Reg<u32, _INTR_CAUSE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE1;
#[doc = "`read()` method returns [intr_cause1::R](intr_cause1::R) reader structure"]
impl crate::Readable for INTR_CAUSE1 {}
#[doc = "Interrupt port cause register 1"]
pub mod intr_cause1;
#[doc = "Interrupt port cause register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause2](intr_cause2) module"]
pub type INTR_CAUSE2 = crate::Reg<u32, _INTR_CAUSE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE2;
#[doc = "`read()` method returns [intr_cause2::R](intr_cause2::R) reader structure"]
impl crate::Readable for INTR_CAUSE2 {}
#[doc = "Interrupt port cause register 2"]
pub mod intr_cause2;
#[doc = "Interrupt port cause register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause3](intr_cause3) module"]
pub type INTR_CAUSE3 = crate::Reg<u32, _INTR_CAUSE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE3;
#[doc = "`read()` method returns [intr_cause3::R](intr_cause3::R) reader structure"]
impl crate::Readable for INTR_CAUSE3 {}
#[doc = "Interrupt port cause register 3"]
pub mod intr_cause3;
#[doc = "Extern power supply detection register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_active](vdd_active) module"]
pub type VDD_ACTIVE = crate::Reg<u32, _VDD_ACTIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDD_ACTIVE;
#[doc = "`read()` method returns [vdd_active::R](vdd_active::R) reader structure"]
impl crate::Readable for VDD_ACTIVE {}
#[doc = "Extern power supply detection register"]
pub mod vdd_active;
#[doc = "Supply detection interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_intr](vdd_intr) module"]
pub type VDD_INTR = crate::Reg<u32, _VDD_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDD_INTR;
#[doc = "`read()` method returns [vdd_intr::R](vdd_intr::R) reader structure"]
impl crate::Readable for VDD_INTR {}
#[doc = "`write(|w| ..)` method takes [vdd_intr::W](vdd_intr::W) writer structure"]
impl crate::Writable for VDD_INTR {}
#[doc = "Supply detection interrupt register"]
pub mod vdd_intr;
#[doc = "Supply detection interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_intr_mask](vdd_intr_mask) module"]
pub type VDD_INTR_MASK = crate::Reg<u32, _VDD_INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDD_INTR_MASK;
#[doc = "`read()` method returns [vdd_intr_mask::R](vdd_intr_mask::R) reader structure"]
impl crate::Readable for VDD_INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [vdd_intr_mask::W](vdd_intr_mask::W) writer structure"]
impl crate::Writable for VDD_INTR_MASK {}
#[doc = "Supply detection interrupt mask register"]
pub mod vdd_intr_mask;
#[doc = "Supply detection interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_intr_masked](vdd_intr_masked) module"]
pub type VDD_INTR_MASKED = crate::Reg<u32, _VDD_INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDD_INTR_MASKED;
#[doc = "`read()` method returns [vdd_intr_masked::R](vdd_intr_masked::R) reader structure"]
impl crate::Readable for VDD_INTR_MASKED {}
#[doc = "Supply detection interrupt masked register"]
pub mod vdd_intr_masked;
#[doc = "Supply detection interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_intr_set](vdd_intr_set) module"]
pub type VDD_INTR_SET = crate::Reg<u32, _VDD_INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDD_INTR_SET;
#[doc = "`read()` method returns [vdd_intr_set::R](vdd_intr_set::R) reader structure"]
impl crate::Readable for VDD_INTR_SET {}
#[doc = "`write(|w| ..)` method takes [vdd_intr_set::W](vdd_intr_set::W) writer structure"]
impl crate::Writable for VDD_INTR_SET {}
#[doc = "Supply detection interrupt set register"]
pub mod vdd_intr_set;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HSIOM port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 8usize],
    #[doc = "0x10 - HSIOM port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 8usize],
    #[doc = "0x20 - HSIOM port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 8usize],
    #[doc = "0x30 - HSIOM port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 8usize],
    #[doc = "0x40 - HSIOM port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 8usize],
    #[doc = "0x50 - HSIOM port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 8usize],
    #[doc = "0x60 - HSIOM port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 8usize],
    #[doc = "0x70 - HSIOM port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 8usize],
    #[doc = "0x80 - HSIOM port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 8usize],
    #[doc = "0x90 - HSIOM port registers"]
    pub prt9: PRT,
    _reserved10: [u8; 8usize],
    #[doc = "0xa0 - HSIOM port registers"]
    pub prt10: PRT,
    _reserved11: [u8; 8usize],
    #[doc = "0xb0 - HSIOM port registers"]
    pub prt11: PRT,
    _reserved12: [u8; 8usize],
    #[doc = "0xc0 - HSIOM port registers"]
    pub prt12: PRT,
    _reserved13: [u8; 8usize],
    #[doc = "0xd0 - HSIOM port registers"]
    pub prt13: PRT,
    _reserved14: [u8; 8usize],
    #[doc = "0xe0 - HSIOM port registers"]
    pub prt14: PRT,
    _reserved15: [u8; 7960usize],
    #[doc = "0x2000 - AMUX splitter cell control"]
    pub amux_split_ctl: [AMUX_SPLIT_CTL; 64],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Port selection 0"]
    pub port_sel0: self::prt::PORT_SEL0,
    #[doc = "0x04 - Port selection 1"]
    pub port_sel1: self::prt::PORT_SEL1,
}
#[doc = r"Register block"]
#[doc = "HSIOM port registers"]
pub mod prt;
#[doc = "AMUX splitter cell control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amux_split_ctl](amux_split_ctl) module"]
pub type AMUX_SPLIT_CTL = crate::Reg<u32, _AMUX_SPLIT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMUX_SPLIT_CTL;
#[doc = "`read()` method returns [amux_split_ctl::R](amux_split_ctl::R) reader structure"]
impl crate::Readable for AMUX_SPLIT_CTL {}
#[doc = "`write(|w| ..)` method takes [amux_split_ctl::W](amux_split_ctl::W) writer structure"]
impl crate::Writable for AMUX_SPLIT_CTL {}
#[doc = "AMUX splitter cell control"]
pub mod amux_split_ctl;

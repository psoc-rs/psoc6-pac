#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Profile control"]
    pub ctl: CTL,
    #[doc = "0x04 - Profile status"]
    pub status: STATUS,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Profile command"]
    pub cmd: CMD,
    _reserved3: [u8; 1964usize],
    #[doc = "0x7c0 - Profile interrupt"]
    pub intr: INTR,
    #[doc = "0x7c4 - Profile interrupt set"]
    pub intr_set: INTR_SET,
    #[doc = "0x7c8 - Profile interrupt mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x7cc - Profile interrupt masked"]
    pub intr_masked: INTR_MASKED,
    _reserved7: [u8; 48usize],
    #[doc = "0x800 - Profile counter structure"]
    pub cnt_struct0: CNT_STRUCT,
    _reserved8: [u8; 4usize],
    #[doc = "0x810 - Profile counter structure"]
    pub cnt_struct1: CNT_STRUCT,
    _reserved9: [u8; 4usize],
    #[doc = "0x820 - Profile counter structure"]
    pub cnt_struct2: CNT_STRUCT,
    _reserved10: [u8; 4usize],
    #[doc = "0x830 - Profile counter structure"]
    pub cnt_struct3: CNT_STRUCT,
    _reserved11: [u8; 4usize],
    #[doc = "0x840 - Profile counter structure"]
    pub cnt_struct4: CNT_STRUCT,
    _reserved12: [u8; 4usize],
    #[doc = "0x850 - Profile counter structure"]
    pub cnt_struct5: CNT_STRUCT,
    _reserved13: [u8; 4usize],
    #[doc = "0x860 - Profile counter structure"]
    pub cnt_struct6: CNT_STRUCT,
    _reserved14: [u8; 4usize],
    #[doc = "0x870 - Profile counter structure"]
    pub cnt_struct7: CNT_STRUCT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT_STRUCT {
    #[doc = "0x00 - Profile counter configuration"]
    pub ctl: self::cnt_struct::CTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Profile counter value"]
    pub cnt: self::cnt_struct::CNT,
}
#[doc = r"Register block"]
#[doc = "Profile counter structure"]
pub mod cnt_struct;
#[doc = "Profile control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Profile control"]
pub mod ctl;
#[doc = "Profile status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Profile status"]
pub mod status;
#[doc = "Profile command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Profile command"]
pub mod cmd;
#[doc = "Profile interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Profile interrupt"]
pub mod intr;
#[doc = "Profile interrupt set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Profile interrupt set"]
pub mod intr_set;
#[doc = "Profile interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Profile interrupt mask"]
pub mod intr_mask;
#[doc = "Profile interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Profile interrupt masked"]
pub mod intr_masked;

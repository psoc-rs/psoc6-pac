#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt cause register"]
    pub intr_cause: INTR_CAUSE,
    _reserved1: [u8; 3580usize],
    #[doc = "0xe00 - AREF configuration"]
    pub aref: AREF,
    _reserved2: [u8; 252usize],
    #[doc = "0xf00 - VREF Trim bits"]
    pub vref_trim0: VREF_TRIM0,
    #[doc = "0xf04 - VREF Trim bits"]
    pub vref_trim1: VREF_TRIM1,
    #[doc = "0xf08 - VREF Trim bits"]
    pub vref_trim2: VREF_TRIM2,
    #[doc = "0xf0c - VREF Trim bits"]
    pub vref_trim3: VREF_TRIM3,
    #[doc = "0xf10 - IZTAT Trim bits"]
    pub iztat_trim0: IZTAT_TRIM0,
    #[doc = "0xf14 - IZTAT Trim bits"]
    pub iztat_trim1: IZTAT_TRIM1,
    #[doc = "0xf18 - IPTAT Trim bits"]
    pub iptat_trim0: IPTAT_TRIM0,
    #[doc = "0xf1c - ICTAT Trim bits"]
    pub ictat_trim0: ICTAT_TRIM0,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct AREF {
    #[doc = "0x00 - global AREF control"]
    pub aref_ctrl: self::aref::AREF_CTRL,
}
#[doc = r"Register block"]
#[doc = "AREF configuration"]
pub mod aref;
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](intr_cause) module"]
pub type INTR_CAUSE = crate::Reg<u32, _INTR_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE;
#[doc = "`read()` method returns [intr_cause::R](intr_cause::R) reader structure"]
impl crate::Readable for INTR_CAUSE {}
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim0](vref_trim0) module"]
pub type VREF_TRIM0 = crate::Reg<u32, _VREF_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF_TRIM0;
#[doc = "`read()` method returns [vref_trim0::R](vref_trim0::R) reader structure"]
impl crate::Readable for VREF_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [vref_trim0::W](vref_trim0::W) writer structure"]
impl crate::Writable for VREF_TRIM0 {}
#[doc = "VREF Trim bits"]
pub mod vref_trim0;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim1](vref_trim1) module"]
pub type VREF_TRIM1 = crate::Reg<u32, _VREF_TRIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF_TRIM1;
#[doc = "`read()` method returns [vref_trim1::R](vref_trim1::R) reader structure"]
impl crate::Readable for VREF_TRIM1 {}
#[doc = "`write(|w| ..)` method takes [vref_trim1::W](vref_trim1::W) writer structure"]
impl crate::Writable for VREF_TRIM1 {}
#[doc = "VREF Trim bits"]
pub mod vref_trim1;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim2](vref_trim2) module"]
pub type VREF_TRIM2 = crate::Reg<u32, _VREF_TRIM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF_TRIM2;
#[doc = "`read()` method returns [vref_trim2::R](vref_trim2::R) reader structure"]
impl crate::Readable for VREF_TRIM2 {}
#[doc = "`write(|w| ..)` method takes [vref_trim2::W](vref_trim2::W) writer structure"]
impl crate::Writable for VREF_TRIM2 {}
#[doc = "VREF Trim bits"]
pub mod vref_trim2;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim3](vref_trim3) module"]
pub type VREF_TRIM3 = crate::Reg<u32, _VREF_TRIM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF_TRIM3;
#[doc = "`read()` method returns [vref_trim3::R](vref_trim3::R) reader structure"]
impl crate::Readable for VREF_TRIM3 {}
#[doc = "`write(|w| ..)` method takes [vref_trim3::W](vref_trim3::W) writer structure"]
impl crate::Writable for VREF_TRIM3 {}
#[doc = "VREF Trim bits"]
pub mod vref_trim3;
#[doc = "IZTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iztat_trim0](iztat_trim0) module"]
pub type IZTAT_TRIM0 = crate::Reg<u32, _IZTAT_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IZTAT_TRIM0;
#[doc = "`read()` method returns [iztat_trim0::R](iztat_trim0::R) reader structure"]
impl crate::Readable for IZTAT_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [iztat_trim0::W](iztat_trim0::W) writer structure"]
impl crate::Writable for IZTAT_TRIM0 {}
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim0;
#[doc = "IZTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iztat_trim1](iztat_trim1) module"]
pub type IZTAT_TRIM1 = crate::Reg<u32, _IZTAT_TRIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IZTAT_TRIM1;
#[doc = "`read()` method returns [iztat_trim1::R](iztat_trim1::R) reader structure"]
impl crate::Readable for IZTAT_TRIM1 {}
#[doc = "`write(|w| ..)` method takes [iztat_trim1::W](iztat_trim1::W) writer structure"]
impl crate::Writable for IZTAT_TRIM1 {}
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim1;
#[doc = "IPTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptat_trim0](iptat_trim0) module"]
pub type IPTAT_TRIM0 = crate::Reg<u32, _IPTAT_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTAT_TRIM0;
#[doc = "`read()` method returns [iptat_trim0::R](iptat_trim0::R) reader structure"]
impl crate::Readable for IPTAT_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [iptat_trim0::W](iptat_trim0::W) writer structure"]
impl crate::Writable for IPTAT_TRIM0 {}
#[doc = "IPTAT Trim bits"]
pub mod iptat_trim0;
#[doc = "ICTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ictat_trim0](ictat_trim0) module"]
pub type ICTAT_TRIM0 = crate::Reg<u32, _ICTAT_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICTAT_TRIM0;
#[doc = "`read()` method returns [ictat_trim0::R](ictat_trim0::R) reader structure"]
impl crate::Readable for ICTAT_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [ictat_trim0::W](ictat_trim0::W) writer structure"]
impl crate::Writable for ICTAT_TRIM0 {}
#[doc = "ICTAT Trim bits"]
pub mod ictat_trim0;

#[doc = "SMPU region address 0 (slave structure)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0](addr0) module"]
pub type ADDR0 = crate::Reg<u32, _ADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR0;
#[doc = "`read()` method returns [addr0::R](addr0::R) reader structure"]
impl crate::Readable for ADDR0 {}
#[doc = "`write(|w| ..)` method takes [addr0::W](addr0::W) writer structure"]
impl crate::Writable for ADDR0 {}
#[doc = "SMPU region address 0 (slave structure)"]
pub mod addr0;
#[doc = "SMPU region attributes 0 (slave structure)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [att0](att0) module"]
pub type ATT0 = crate::Reg<u32, _ATT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATT0;
#[doc = "`read()` method returns [att0::R](att0::R) reader structure"]
impl crate::Readable for ATT0 {}
#[doc = "`write(|w| ..)` method takes [att0::W](att0::W) writer structure"]
impl crate::Writable for ATT0 {}
#[doc = "SMPU region attributes 0 (slave structure)"]
pub mod att0;
#[doc = "SMPU region address 1 (master structure)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1](addr1) module"]
pub type ADDR1 = crate::Reg<u32, _ADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR1;
#[doc = "`read()` method returns [addr1::R](addr1::R) reader structure"]
impl crate::Readable for ADDR1 {}
#[doc = "SMPU region address 1 (master structure)"]
pub mod addr1;
#[doc = "SMPU region attributes 1 (master structure)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [att1](att1) module"]
pub type ATT1 = crate::Reg<u32, _ATT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATT1;
#[doc = "`read()` method returns [att1::R](att1::R) reader structure"]
impl crate::Readable for ATT1 {}
#[doc = "`write(|w| ..)` method takes [att1::W](att1::W) writer structure"]
impl crate::Writable for ATT1 {}
#[doc = "SMPU region attributes 1 (master structure)"]
pub mod att1;

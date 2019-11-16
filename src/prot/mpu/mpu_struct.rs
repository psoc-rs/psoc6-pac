#[doc = "MPU region address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "MPU region address"]
pub mod addr;
#[doc = "MPU region attrributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [att](att) module"]
pub type ATT = crate::Reg<u32, _ATT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATT;
#[doc = "`read()` method returns [att::R](att::R) reader structure"]
impl crate::Readable for ATT {}
#[doc = "`write(|w| ..)` method takes [att::W](att::W) writer structure"]
impl crate::Writable for ATT {}
#[doc = "MPU region attrributes"]
pub mod att;

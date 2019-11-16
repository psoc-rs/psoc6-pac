#[doc = "global AREF control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aref_ctrl](aref_ctrl) module"]
pub type AREF_CTRL = crate::Reg<u32, _AREF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AREF_CTRL;
#[doc = "`read()` method returns [aref_ctrl::R](aref_ctrl::R) reader structure"]
impl crate::Readable for AREF_CTRL {}
#[doc = "`write(|w| ..)` method takes [aref_ctrl::W](aref_ctrl::W) writer structure"]
impl crate::Writable for AREF_CTRL {}
#[doc = "global AREF control"]
pub mod aref_ctrl;

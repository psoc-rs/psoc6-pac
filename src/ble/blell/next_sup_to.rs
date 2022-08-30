#[doc = "Register `NEXT_SUP_TO` reader"]
pub struct R(crate::R<NEXT_SUP_TO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEXT_SUP_TO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEXT_SUP_TO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEXT_SUP_TO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NEXT_TIMEOUT_INSTANT` reader - This field defines the clock instant at which the next connection supervision timeout event will occur on a connection This is with reference to the 16-bit internal reference clock."]
pub type NEXT_TIMEOUT_INSTANT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the clock instant at which the next connection supervision timeout event will occur on a connection This is with reference to the 16-bit internal reference clock."]
    #[inline(always)]
    pub fn next_timeout_instant(&self) -> NEXT_TIMEOUT_INSTANT_R {
        NEXT_TIMEOUT_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Next supervision timeout instant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_sup_to](index.html) module"]
pub struct NEXT_SUP_TO_SPEC;
impl crate::RegisterSpec for NEXT_SUP_TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [next_sup_to::R](R) reader structure"]
impl crate::Readable for NEXT_SUP_TO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NEXT_SUP_TO to value 0"]
impl crate::Resettable for NEXT_SUP_TO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

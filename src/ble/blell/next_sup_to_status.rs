#[doc = "Register `NEXT_SUP_TO_STATUS` reader"]
pub struct R(crate::R<NEXT_SUP_TO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEXT_SUP_TO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEXT_SUP_TO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEXT_SUP_TO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NEXT_SUP_TO` reader - HW updates this register for the SuperVision timeout next instant, granularity is 625us"]
pub type NEXT_SUP_TO_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - HW updates this register for the SuperVision timeout next instant, granularity is 625us"]
    #[inline(always)]
    pub fn next_sup_to(&self) -> NEXT_SUP_TO_R {
        NEXT_SUP_TO_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Next Supervision timeout Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_sup_to_status](index.html) module"]
pub struct NEXT_SUP_TO_STATUS_SPEC;
impl crate::RegisterSpec for NEXT_SUP_TO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [next_sup_to_status::R](R) reader structure"]
impl crate::Readable for NEXT_SUP_TO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NEXT_SUP_TO_STATUS to value 0"]
impl crate::Resettable for NEXT_SUP_TO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

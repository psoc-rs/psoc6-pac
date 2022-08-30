#[doc = "Register `SATURATE_INTR_MASKED` reader"]
pub struct R(crate::R<SATURATE_INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SATURATE_INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SATURATE_INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SATURATE_INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SATURATE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type SATURATE_MASKED_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn saturate_masked(&self) -> SATURATE_MASKED_R {
        SATURATE_MASKED_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Saturate interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saturate_intr_masked](index.html) module"]
pub struct SATURATE_INTR_MASKED_SPEC;
impl crate::RegisterSpec for SATURATE_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saturate_intr_masked::R](R) reader structure"]
impl crate::Readable for SATURATE_INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SATURATE_INTR_MASKED to value 0"]
impl crate::Resettable for SATURATE_INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

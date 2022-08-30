#[doc = "Register `RANGE_INTR_MASKED` reader"]
pub struct R(crate::R<RANGE_INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANGE_INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANGE_INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANGE_INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RANGE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type RANGE_MASKED_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn range_masked(&self) -> RANGE_MASKED_R {
        RANGE_MASKED_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Range interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_intr_masked](index.html) module"]
pub struct RANGE_INTR_MASKED_SPEC;
impl crate::RegisterSpec for RANGE_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [range_intr_masked::R](R) reader structure"]
impl crate::Readable for RANGE_INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RANGE_INTR_MASKED to value 0"]
impl crate::Resettable for RANGE_INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

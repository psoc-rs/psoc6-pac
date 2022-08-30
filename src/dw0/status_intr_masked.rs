#[doc = "Register `STATUS_INTR_MASKED` reader"]
pub struct R(crate::R<STATUS_INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH` reader - Reflects the INTR_MASKED.CH bit fields of all channels."]
pub type CH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reflects the INTR_MASKED.CH bit fields of all channels."]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(self.bits)
    }
}
#[doc = "Status of interrupts masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_intr_masked](index.html) module"]
pub struct STATUS_INTR_MASKED_SPEC;
impl crate::RegisterSpec for STATUS_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status_intr_masked::R](R) reader structure"]
impl crate::Readable for STATUS_INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS_INTR_MASKED to value 0"]
impl crate::Resettable for STATUS_INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

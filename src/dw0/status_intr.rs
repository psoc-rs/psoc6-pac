#[doc = "Register `STATUS_INTR` reader"]
pub struct R(crate::R<STATUS_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH` reader - Reflects the INTR.CH bit fields of all channels."]
pub type CH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reflects the INTR.CH bit fields of all channels."]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(self.bits)
    }
}
#[doc = "System interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_intr](index.html) module"]
pub struct STATUS_INTR_SPEC;
impl crate::RegisterSpec for STATUS_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status_intr::R](R) reader structure"]
impl crate::Readable for STATUS_INTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS_INTR to value 0"]
impl crate::Resettable for STATUS_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

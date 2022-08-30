#[doc = "Register `EARLY_INTR_NI` reader"]
pub struct R(crate::R<EARLY_INTR_NI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EARLY_INTR_NI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EARLY_INTR_NI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EARLY_INTR_NI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EARLY_INTR_NI` reader - Connection Next instant when the early interrupt is triggered"]
pub type EARLY_INTR_NI_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Connection Next instant when the early interrupt is triggered"]
    #[inline(always)]
    pub fn early_intr_ni(&self) -> EARLY_INTR_NI_R {
        EARLY_INTR_NI_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "NI at early interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [early_intr_ni](index.html) module"]
pub struct EARLY_INTR_NI_SPEC;
impl crate::RegisterSpec for EARLY_INTR_NI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [early_intr_ni::R](R) reader structure"]
impl crate::Readable for EARLY_INTR_NI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EARLY_INTR_NI to value 0"]
impl crate::Resettable for EARLY_INTR_NI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

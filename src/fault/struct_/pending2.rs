#[doc = "Register `PENDING2` reader"]
pub struct R(crate::R<PENDING2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOURCE` reader - This field specifies the following sources: Bit 0 - 31: TBD."]
pub type SOURCE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0 - 31: TBD."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
#[doc = "Fault pending 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending2](index.html) module"]
pub struct PENDING2_SPEC;
impl crate::RegisterSpec for PENDING2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending2::R](R) reader structure"]
impl crate::Readable for PENDING2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PENDING2 to value 0"]
impl crate::Resettable for PENDING2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `BIST_DATA_EXP[%s]` reader"]
pub struct R(crate::R<BIST_DATA_EXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_DATA_EXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_DATA_EXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_DATA_EXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - This field specified the expected Flash data output."]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field specified the expected Flash data output."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "BIST data expected register(s)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_data_exp](index.html) module"]
pub struct BIST_DATA_EXP_SPEC;
impl crate::RegisterSpec for BIST_DATA_EXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_data_exp::R](R) reader structure"]
impl crate::Readable for BIST_DATA_EXP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BIST_DATA_EXP[%s]
to value 0"]
impl crate::Resettable for BIST_DATA_EXP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

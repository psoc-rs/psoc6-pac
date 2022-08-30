#[doc = "Register `RESULT_VAL2` reader"]
pub struct R(crate::R<RESULT_VAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_VAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_VAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_VAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - Only used in case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt), this counter counts when csd_sense is low."]
pub type VALUE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Only used in case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt), this counter counts when csd_sense is low."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result CSX accumulation counter value 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result_val2](index.html) module"]
pub struct RESULT_VAL2_SPEC;
impl crate::RegisterSpec for RESULT_VAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result_val2::R](R) reader structure"]
impl crate::Readable for RESULT_VAL2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT_VAL2 to value 0"]
impl crate::Resettable for RESULT_VAL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

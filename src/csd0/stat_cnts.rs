#[doc = "Register `STAT_CNTS` reader"]
pub struct R(crate::R<STAT_CNTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_CNTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_CNTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_CNTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_CONV` reader - Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
pub type NUM_CONV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
    #[inline(always)]
    pub fn num_conv(&self) -> NUM_CONV_R {
        NUM_CONV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Current status counts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_cnts](index.html) module"]
pub struct STAT_CNTS_SPEC;
impl crate::RegisterSpec for STAT_CNTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat_cnts::R](R) reader structure"]
impl crate::Readable for STAT_CNTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT_CNTS to value 0"]
impl crate::Resettable for STAT_CNTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

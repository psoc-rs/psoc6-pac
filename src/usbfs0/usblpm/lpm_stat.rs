#[doc = "Register `LPM_STAT` reader"]
pub struct R(crate::R<LPM_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPM_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPM_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPM_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPM_BESL` reader - Best Effort Service Latency This value should match either the Baseline (DeepSleep) or Deep (Hibernate) BESL in the BOS descriptor."]
pub type LPM_BESL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_REMOTEWAKE` reader - 0: Device is prohibited from initiating a remote wake 1: Device is allow to wake the host"]
pub type LPM_REMOTEWAKE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Best Effort Service Latency This value should match either the Baseline (DeepSleep) or Deep (Hibernate) BESL in the BOS descriptor."]
    #[inline(always)]
    pub fn lpm_besl(&self) -> LPM_BESL_R {
        LPM_BESL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: Device is prohibited from initiating a remote wake 1: Device is allow to wake the host"]
    #[inline(always)]
    pub fn lpm_remotewake(&self) -> LPM_REMOTEWAKE_R {
        LPM_REMOTEWAKE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "LPM Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpm_stat](index.html) module"]
pub struct LPM_STAT_SPEC;
impl crate::RegisterSpec for LPM_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpm_stat::R](R) reader structure"]
impl crate::Readable for LPM_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPM_STAT to value 0"]
impl crate::Resettable for LPM_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

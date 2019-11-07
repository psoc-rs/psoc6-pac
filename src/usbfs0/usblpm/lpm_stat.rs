#[doc = "Reader of register LPM_STAT"]
pub type R = crate::R<u32, super::LPM_STAT>;
#[doc = "Reader of field `LPM_BESL`"]
pub type LPM_BESL_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPM_REMOTEWAKE`"]
pub type LPM_REMOTEWAKE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Best Effort Service Latency This value should match either the Baseline (DeepSleep) or Deep (Hibernate) BESL in the BOS descriptor."]
    #[inline(always)]
    pub fn lpm_besl(&self) -> LPM_BESL_R {
        LPM_BESL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: Device is prohibited from initiating a remote wake 1: Device is allow to wake the host"]
    #[inline(always)]
    pub fn lpm_remotewake(&self) -> LPM_REMOTEWAKE_R {
        LPM_REMOTEWAKE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}

#[doc = "Reader of register US_COUNTER"]
pub type R = crate::R<u32, super::US_COUNTER>;
#[doc = "Reader of field `US_COUNTER`"]
pub type US_COUNTER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Current value of the US Counter"]
    #[inline(always)]
    pub fn us_counter(&self) -> US_COUNTER_R {
        US_COUNTER_R::new((self.bits & 0x03ff) as u16)
    }
}

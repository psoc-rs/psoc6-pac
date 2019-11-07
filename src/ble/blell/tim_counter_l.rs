#[doc = "Reader of register TIM_COUNTER_L"]
pub type R = crate::R<u32, super::TIM_COUNTER_L>;
#[doc = "Reader of field `TIM_REF_CLOCK`"]
pub type TIM_REF_CLOCK_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit internal reference clock. The clock is a free run-ning clock, incremented by a 0.625ms periodic pulse. It is used as a reference clock to derive all the timing required as per protocol."]
    #[inline(always)]
    pub fn tim_ref_clock(&self) -> TIM_REF_CLOCK_R {
        TIM_REF_CLOCK_R::new((self.bits & 0xffff) as u16)
    }
}

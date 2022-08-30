#[doc = "Register `TIM_COUNTER_L` reader"]
pub struct R(crate::R<TIM_COUNTER_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_COUNTER_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_COUNTER_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_COUNTER_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM_REF_CLOCK` reader - 16-bit internal reference clock. The clock is a free run-ning clock, incremented by a 0.625ms periodic pulse. It is used as a reference clock to derive all the timing required as per protocol."]
pub type TIM_REF_CLOCK_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit internal reference clock. The clock is a free run-ning clock, incremented by a 0.625ms periodic pulse. It is used as a reference clock to derive all the timing required as per protocol."]
    #[inline(always)]
    pub fn tim_ref_clock(&self) -> TIM_REF_CLOCK_R {
        TIM_REF_CLOCK_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Reference Clock\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim_counter_l](index.html) module"]
pub struct TIM_COUNTER_L_SPEC;
impl crate::RegisterSpec for TIM_COUNTER_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim_counter_l::R](R) reader structure"]
impl crate::Readable for TIM_COUNTER_L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIM_COUNTER_L to value 0"]
impl crate::Resettable for TIM_COUNTER_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

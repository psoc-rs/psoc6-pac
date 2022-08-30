#[doc = "Register `NEXT_RESP_TIMER_EXP` reader"]
pub struct R(crate::R<NEXT_RESP_TIMER_EXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEXT_RESP_TIMER_EXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEXT_RESP_TIMER_EXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEXT_RESP_TIMER_EXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NEXT_RESPONSE_INSTANT` reader - This field defines the clock instant at which the next PDU response timeout event will occur on a connection. This is with reference to the 16-bit internal reference clock."]
pub type NEXT_RESPONSE_INSTANT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the clock instant at which the next PDU response timeout event will occur on a connection. This is with reference to the 16-bit internal reference clock."]
    #[inline(always)]
    pub fn next_response_instant(&self) -> NEXT_RESPONSE_INSTANT_R {
        NEXT_RESPONSE_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Next response timeout instant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_resp_timer_exp](index.html) module"]
pub struct NEXT_RESP_TIMER_EXP_SPEC;
impl crate::RegisterSpec for NEXT_RESP_TIMER_EXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [next_resp_timer_exp::R](R) reader structure"]
impl crate::Readable for NEXT_RESP_TIMER_EXP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NEXT_RESP_TIMER_EXP to value 0"]
impl crate::Resettable for NEXT_RESP_TIMER_EXP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

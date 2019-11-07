#[doc = "Reader of register NEXT_RESP_TIMER_EXP"]
pub type R = crate::R<u32, super::NEXT_RESP_TIMER_EXP>;
#[doc = "Reader of field `NEXT_RESPONSE_INSTANT`"]
pub type NEXT_RESPONSE_INSTANT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the clock instant at which the next PDU response timeout event will occur on a connection. This is with reference to the 16-bit internal reference clock."]
    #[inline(always)]
    pub fn next_response_instant(&self) -> NEXT_RESPONSE_INSTANT_R {
        NEXT_RESPONSE_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}

#[doc = "Reader of register LE_PING_TIMER_NEXT_EXP"]
pub type R = crate::R<u32, super::LE_PING_TIMER_NEXT_EXP>;
#[doc = "Reader of field `CONN_PING_TIMER_NEXT_EXP`"]
pub type CONN_PING_TIMER_NEXT_EXP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The value of ping timer next expiry instant in the terms of native clock value (least 16 bit value of the 17 bit ping counter). This together with CONN_PING_TIMER_NEXT_EXP_WRAP will provide the correct status of ping timer duration."]
    #[inline(always)]
    pub fn conn_ping_timer_next_exp(&self) -> CONN_PING_TIMER_NEXT_EXP_R {
        CONN_PING_TIMER_NEXT_EXP_R::new((self.bits & 0xffff) as u16)
    }
}

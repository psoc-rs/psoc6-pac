#[doc = "Reader of register LE_PING_TIMER_WRAP_COUNT"]
pub type R = crate::R<u32, super::LE_PING_TIMER_WRAP_COUNT>;
#[doc = "Reader of field `CONN_SEC_CURRENT_WRAP`"]
pub type CONN_SEC_CURRENT_WRAP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register holds the current position of the Ping timer."]
    #[inline(always)]
    pub fn conn_sec_current_wrap(&self) -> CONN_SEC_CURRENT_WRAP_R {
        CONN_SEC_CURRENT_WRAP_R::new((self.bits & 0xffff) as u16)
    }
}

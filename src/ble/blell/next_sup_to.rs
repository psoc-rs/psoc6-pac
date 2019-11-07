#[doc = "Reader of register NEXT_SUP_TO"]
pub type R = crate::R<u32, super::NEXT_SUP_TO>;
#[doc = "Reader of field `NEXT_TIMEOUT_INSTANT`"]
pub type NEXT_TIMEOUT_INSTANT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the clock instant at which the next connection supervision timeout event will occur on a connection This is with reference to the 16-bit internal reference clock."]
    #[inline(always)]
    pub fn next_timeout_instant(&self) -> NEXT_TIMEOUT_INSTANT_R {
        NEXT_TIMEOUT_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}

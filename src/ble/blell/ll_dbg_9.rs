#[doc = "Reader of register LL_DBG_9"]
pub type R = crate::R<u32, super::LL_DBG_9>;
#[doc = "Reader of field `WINDOW_WIDEN`"]
pub type WINDOW_WIDEN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Window Widening value in us. The reset value of this register is 0x0000. After reset de-assertion, at the first clock cycle, the value 0x0010 is assigned to the register."]
    #[inline(always)]
    pub fn window_widen(&self) -> WINDOW_WIDEN_R {
        WINDOW_WIDEN_R::new((self.bits & 0xffff) as u16)
    }
}

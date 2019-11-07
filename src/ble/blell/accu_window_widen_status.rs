#[doc = "Reader of register ACCU_WINDOW_WIDEN_STATUS"]
pub type R = crate::R<u32, super::ACCU_WINDOW_WIDEN_STATUS>;
#[doc = "Reader of field `ACCU_WINDOW_WIDEN`"]
pub type ACCU_WINDOW_WIDEN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Accumulated Window Widen Value. HW updates this register at the close of slave connection event"]
    #[inline(always)]
    pub fn accu_window_widen(&self) -> ACCU_WINDOW_WIDEN_R {
        ACCU_WINDOW_WIDEN_R::new((self.bits & 0xffff) as u16)
    }
}

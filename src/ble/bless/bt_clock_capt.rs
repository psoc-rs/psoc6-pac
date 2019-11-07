#[doc = "Reader of register BT_CLOCK_CAPT"]
pub type R = crate::R<u32, super::BT_CLOCK_CAPT>;
#[doc = "Reader of field `BT_CLOCK`"]
pub type BT_CLOCK_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field captures the LF BT clock captured on an LL DSM exit. This register is valid only when MT_STATUS.LL_CLK_STATE is set. This value may be used to manage the low power entry."]
    #[inline(always)]
    pub fn bt_clock(&self) -> BT_CLOCK_R {
        BT_CLOCK_R::new((self.bits & 0xffff) as u16)
    }
}

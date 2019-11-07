#[doc = "Reader of register BT_SLOT_CAPT_STATUS"]
pub type R = crate::R<u32, super::BT_SLOT_CAPT_STATUS>;
#[doc = "Reader of field `BT_SLOT`"]
pub type BT_SLOT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - During slave connection event, HW updates this register with the captured BT_SLOT at anchor point, granularity is 625us"]
    #[inline(always)]
    pub fn bt_slot(&self) -> BT_SLOT_R {
        BT_SLOT_R::new((self.bits & 0xffff) as u16)
    }
}

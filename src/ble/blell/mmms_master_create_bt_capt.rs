#[doc = "Reader of register MMMS_MASTER_CREATE_BT_CAPT"]
pub type R = crate::R<u32, super::MMMS_MASTER_CREATE_BT_CAPT>;
#[doc = "Reader of field `BT_SLOT`"]
pub type BT_SLOT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register captures the BT_SLOT when master connection is created, granularity is 625us"]
    #[inline(always)]
    pub fn bt_slot(&self) -> BT_SLOT_R {
        BT_SLOT_R::new((self.bits & 0xffff) as u16)
    }
}

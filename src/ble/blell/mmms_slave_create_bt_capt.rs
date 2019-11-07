#[doc = "Reader of register MMMS_SLAVE_CREATE_BT_CAPT"]
pub type R = crate::R<u32, super::MMMS_SLAVE_CREATE_BT_CAPT>;
#[doc = "Reader of field `US_CAPT`"]
pub type US_CAPT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - This register captures the BT_SLOT when slave connection is created, granularity is 625us"]
    #[inline(always)]
    pub fn us_capt(&self) -> US_CAPT_R {
        US_CAPT_R::new((self.bits & 0x03ff) as u16)
    }
}

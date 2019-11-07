#[doc = "Reader of register LL_DBG_10"]
pub type R = crate::R<u32, super::LL_DBG_10>;
#[doc = "Reader of field `RF_CHANNEL_NUM`"]
pub type RF_CHANNEL_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Active channel number"]
    #[inline(always)]
    pub fn rf_channel_num(&self) -> RF_CHANNEL_NUM_R {
        RF_CHANNEL_NUM_R::new((self.bits & 0x3f) as u8)
    }
}

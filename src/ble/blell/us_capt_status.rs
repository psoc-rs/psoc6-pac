#[doc = "Reader of register US_CAPT_STATUS"]
pub type R = crate::R<u32, super::US_CAPT_STATUS>;
#[doc = "Reader of field `US_CAPT`"]
pub type US_CAPT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - During slave connection event, HW updates this register with the captured microsecond at anchor point, granularity is 1us"]
    #[inline(always)]
    pub fn us_capt(&self) -> US_CAPT_R {
        US_CAPT_R::new((self.bits & 0x03ff) as u16)
    }
}

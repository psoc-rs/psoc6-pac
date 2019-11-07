#[doc = "Reader of register INIT_WINDOW_NI_ANCHOR_PT"]
pub type R = crate::R<u32, super::INIT_WINDOW_NI_ANCHOR_PT>;
#[doc = "Reader of field `INIT_INT_OFF_CAPT`"]
pub type INIT_INT_OFF_CAPT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Initiator interval offset captured at conn request. The value indicates the master connection anchor point. This value is in 625us slots"]
    #[inline(always)]
    pub fn init_int_off_capt(&self) -> INIT_INT_OFF_CAPT_R {
        INIT_INT_OFF_CAPT_R::new((self.bits & 0xffff) as u16)
    }
}

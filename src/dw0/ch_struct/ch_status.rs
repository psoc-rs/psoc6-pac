#[doc = "Reader of register CH_STATUS"]
pub type R = crate::R<u32, super::CH_STATUS>;
#[doc = "Reader of field `INTR_CAUSE`"]
pub type INTR_CAUSE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Specifies the source of the interrupt cause: '0': NO_INTR '1': COMPLETION '2': SRC_BUS_ERROR '3': DST_BUS_ERROR '4': SRC_MISAL '5': DST_MISAL '6': CURR_PTR_NULL '7': ACTIVE_CH_DISABLED '8': DESCR_BUS_ERROR '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '1', '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
    #[inline(always)]
    pub fn intr_cause(&self) -> INTR_CAUSE_R {
        INTR_CAUSE_R::new((self.bits & 0x0f) as u8)
    }
}

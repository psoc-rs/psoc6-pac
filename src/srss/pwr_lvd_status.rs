#[doc = "Reader of register PWR_LVD_STATUS"]
pub type R = crate::R<u32, super::PWR_LVD_STATUS>;
#[doc = "Reader of field `HVLVD1_OK`"]
pub type HVLVD1_OK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
    #[inline(always)]
    pub fn hvlvd1_ok(&self) -> HVLVD1_OK_R {
        HVLVD1_OK_R::new((self.bits & 0x01) != 0)
    }
}

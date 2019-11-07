#[doc = "Reader of register MBIST_STAT"]
pub type R = crate::R<u32, super::MBIST_STAT>;
#[doc = "Reader of field `SFP_READY`"]
pub type SFP_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SFP_FAIL`"]
pub type SFP_FAIL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Flag indicating the BIST run is done. Note that after starting a BIST run this flag must be set before a new run can be started. For the first BIST run this will be 0."]
    #[inline(always)]
    pub fn sfp_ready(&self) -> SFP_READY_R {
        SFP_READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Report status of the BIST run, only valid if SFP_READY=1"]
    #[inline(always)]
    pub fn sfp_fail(&self) -> SFP_FAIL_R {
        SFP_FAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}

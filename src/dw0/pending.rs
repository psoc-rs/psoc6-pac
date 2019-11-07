#[doc = "Reader of register PENDING"]
pub type R = crate::R<u32, super::PENDING>;
#[doc = "Reader of field `CH_PENDING`"]
pub type CH_PENDING_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
    #[inline(always)]
    pub fn ch_pending(&self) -> CH_PENDING_R {
        CH_PENDING_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

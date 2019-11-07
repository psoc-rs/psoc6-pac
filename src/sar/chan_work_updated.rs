#[doc = "Reader of register CHAN_WORK_UPDATED"]
pub type R = crate::R<u32, super::CHAN_WORK_UPDATED>;
#[doc = "Reader of field `CHAN_WORK_UPDATED`"]
pub type CHAN_WORK_UPDATED_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding WORK register was updated, i.e. was already sampled during the current scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub fn chan_work_updated(&self) -> CHAN_WORK_UPDATED_R {
        CHAN_WORK_UPDATED_R::new((self.bits & 0xffff) as u16)
    }
}

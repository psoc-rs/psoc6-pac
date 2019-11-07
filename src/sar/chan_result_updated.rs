#[doc = "Reader of register CHAN_RESULT_UPDATED"]
pub type R = crate::R<u32, super::CHAN_RESULT_UPDATED>;
#[doc = "Reader of field `CHAN_RESULT_UPDATED`"]
pub type CHAN_RESULT_UPDATED_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding RESULT register was updated, i.e. was sampled during the previous scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub fn chan_result_updated(&self) -> CHAN_RESULT_UPDATED_R {
        CHAN_RESULT_UPDATED_R::new((self.bits & 0xffff) as u16)
    }
}

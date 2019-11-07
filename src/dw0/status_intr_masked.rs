#[doc = "Reader of register STATUS_INTR_MASKED"]
pub type R = crate::R<u32, super::STATUS_INTR_MASKED>;
#[doc = "Reader of field `CH`"]
pub type CH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reflects the INTR_MASKED.CH bit fields of all channels."]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

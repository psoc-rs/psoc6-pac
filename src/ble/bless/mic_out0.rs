#[doc = "Reader of register MIC_OUT0"]
pub type R = crate::R<u32, super::MIC_OUT0>;
#[doc = "Reader of field `MIC_OUT`"]
pub type MIC_OUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is the MIC generated during CCM encryption."]
    #[inline(always)]
    pub fn mic_out(&self) -> MIC_OUT_R {
        MIC_OUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

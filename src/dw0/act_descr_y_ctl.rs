#[doc = "Reader of register ACT_DESCR_Y_CTL"]
pub type R = crate::R<u32, super::ACT_DESCR_Y_CTL>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_Y_CTL of the currently active descriptor."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

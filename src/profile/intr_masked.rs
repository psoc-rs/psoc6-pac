#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `CNT_OVFLW`"]
pub type CNT_OVFLW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CNT_OVFLW_R {
        CNT_OVFLW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

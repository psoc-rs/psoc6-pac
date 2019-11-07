#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `VDAC_EMPTY_MASKED`"]
pub type VDAC_EMPTY_MASKED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn vdac_empty_masked(&self) -> VDAC_EMPTY_MASKED_R {
        VDAC_EMPTY_MASKED_R::new((self.bits & 0x01) != 0)
    }
}

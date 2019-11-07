#[doc = "Reader of register RANGE_INTR_MASKED"]
pub type R = crate::R<u32, super::RANGE_INTR_MASKED>;
#[doc = "Reader of field `RANGE_MASKED`"]
pub type RANGE_MASKED_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn range_masked(&self) -> RANGE_MASKED_R {
        RANGE_MASKED_R::new((self.bits & 0xffff) as u16)
    }
}

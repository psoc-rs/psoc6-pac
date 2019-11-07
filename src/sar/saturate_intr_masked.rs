#[doc = "Reader of register SATURATE_INTR_MASKED"]
pub type R = crate::R<u32, super::SATURATE_INTR_MASKED>;
#[doc = "Reader of field `SATURATE_MASKED`"]
pub type SATURATE_MASKED_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn saturate_masked(&self) -> SATURATE_MASKED_R {
        SATURATE_MASKED_R::new((self.bits & 0xffff) as u16)
    }
}
